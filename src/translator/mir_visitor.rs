//! Implementation of the trait `rustc_middle::mir::visit::Visitor` for Translator.
//! <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
//!
//! This trait defines a method for visiting every possible MIR element.
//! It is not required to implement every method, only for the elements we care about.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>

use super::sync::{handle_aggregate_assignment, link_if_sync_variable, mutex};
use super::Translator;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::TerminatorKind::{
    Assert, Call, Drop, FalseEdge, FalseUnwind, GeneratorDrop, Goto, InlineAsm, Resume, Return,
    SwitchInt, Terminate, Unreachable, Yield,
};
use rustc_middle::mir::UnwindAction;

impl<'tcx> Visitor<'tcx> for Translator<'tcx> {
    /// Entering a new basic block of the current MIR function.
    /// Activate it. This is the first step to start processing it.
    fn visit_basic_block_data(
        &mut self,
        block: rustc_middle::mir::BasicBlock,
        data: &rustc_middle::mir::BasicBlockData<'tcx>,
    ) {
        let function = self.call_stack.peek_mut();
        function.activate_block(block, &mut self.net);

        self.super_basic_block_data(block, data);
    }

    /// Keep track of synchronization variables in assignments
    /// (mutexes, mutex guards, join handles and condition variables).
    /// The idea is to link the right-hand side with the left-hand side of the assignment
    /// if a synchronization variable is involved.
    fn visit_assign(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        rvalue: &rustc_middle::mir::Rvalue<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        match rvalue {
            rustc_middle::mir::Rvalue::Use(
                rustc_middle::mir::Operand::Copy(rhs) | rustc_middle::mir::Operand::Move(rhs),
            )
            | rustc_middle::mir::Rvalue::Ref(_, _, rhs) => {
                let function = self.call_stack.peek_mut();
                link_if_sync_variable(place, rhs, &mut function.memory, function.def_id, self.tcx);
            }
            rustc_middle::mir::Rvalue::Aggregate(_, operands) => {
                let function = self.call_stack.peek_mut();
                handle_aggregate_assignment(
                    place,
                    &operands.raw,
                    &mut function.memory,
                    function.def_id,
                    self.tcx,
                );
            }
            // No need to do anything for the other cases for now.
            _ => {}
        }

        self.super_assign(place, rvalue, location);
    }

    fn visit_terminator(
        &mut self,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        let function = self.call_stack.peek_mut();

        match terminator.kind {
            Goto { target } => {
                function.goto(target, &mut self.net);
            }
            SwitchInt {
                discr: _,
                ref targets,
            } => {
                // Convert the specific type for the targets vector into a `std::collections::Vec`
                // <rustc_middle::mir::terminator::SwitchTargets>
                function.switch_int(targets.all_targets().to_vec(), &mut self.net);
            }
            Resume | Terminate => {
                function.unwind(&self.program_panic, &mut self.net);
            }
            Return => {
                function.return_statement(&mut self.net);
            }
            Unreachable => {
                function.unreachable(&self.program_end, &mut self.net);
            }
            Drop {
                place,
                target,
                unwind,
            } => {
                let (transition, cleanup_transition) = match unwind {
                    UnwindAction::Cleanup(cleanup) => {
                        function.drop(target, Some(cleanup), &mut self.net)
                    }
                    // Do NOT model the `Terminate` case.
                    // It is not relevant for deadlock detection and makes the Petri nets unnecessarily bigger.
                    UnwindAction::Continue | UnwindAction::Terminate => {
                        function.drop(target, None, &mut self.net)
                    }
                    UnwindAction::Unreachable => {
                        function.unreachable(&self.program_end, &mut self.net);
                        function.drop(target, None, &mut self.net)
                    }
                };

                let memory = &mut function.memory;
                let net = &mut self.net;
                mutex::handle_mutex_guard_drop(place, &transition, net, memory);
                if let Some(cleanup_transition) = cleanup_transition {
                    mutex::handle_mutex_guard_drop(place, &cleanup_transition, net, memory);
                }
            }
            Call {
                ref func,
                ref args,
                destination,
                target,
                unwind,
                from_hir_call: _,
                fn_span: _,
            } => {
                self.call_function(func, args, destination, target, unwind);
            }
            Assert {
                cond: _,
                expected: _,
                msg: _,
                target,
                unwind,
            } => {
                match unwind {
                    UnwindAction::Cleanup(cleanup) => {
                        function.assert(target, Some(cleanup), &mut self.net);
                    }
                    // Do NOT model the `Terminate` case.
                    // It is not relevant for deadlock detection and makes the Petri nets unnecessarily bigger.
                    UnwindAction::Continue | UnwindAction::Terminate => {
                        function.assert(target, None, &mut self.net);
                    }
                    UnwindAction::Unreachable => {
                        function.assert(target, None, &mut self.net);
                        function.unreachable(&self.program_end, &mut self.net);
                    }
                }
            }
            Yield { .. } => {
                unimplemented!("TerminatorKind::Yield not implemented yet")
            }
            GeneratorDrop => {
                unimplemented!("TerminatorKind::GeneratorDrop not implemented yet")
            }
            FalseEdge { .. } => {
                unimplemented!("TerminatorKind::FalseEdge not implemented yet")
            }
            FalseUnwind { .. } => {
                unimplemented!("TerminatorKind::FalseUnwind not implemented yet")
            }
            InlineAsm { .. } => {
                unimplemented!("TerminatorKind::InlineAsm not implemented yet")
            }
        }

        self.super_terminator(terminator, location);
    }
}
