//! Implementation of the trait `rustc_middle::mir::visit::Visitor` for Translator.
//! <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
//!
//! This trait defines a method for visiting every possible MIR element.
//! It is not required to implement every method, only for the elements we care about.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>

use crate::translator::sync::is_mutex_declaration;
use crate::translator::utils::place_to_local;
use crate::translator::Translator;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::TerminatorKind;

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

    /// Identify MIR assignments of the form: `_X = &_Y` where:
    /// - `_X` is of type `&std::sync::Mutex<T>` and
    /// - `_Y` is of type `std::sync::Mutex<T>`.
    ///
    /// Link the local on the left-hand side to the mutex on the right-hand side.
    fn visit_assign(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        rvalue: &rustc_middle::mir::Rvalue<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        let function = self.call_stack.peek_mut();
        let body = self.tcx.optimized_mir(function.def_id);

        if let rustc_middle::mir::Rvalue::Ref(_, _, rhs) = rvalue {
            let rhs = place_to_local(rhs);
            let local_decl = &body.local_decls[rhs];
            if is_mutex_declaration(local_decl) {
                let lhs = place_to_local(place);
                function.memory.link_local_to_same_mutex(lhs, rhs);
            }
        }

        self.super_assign(place, rvalue, location);
    }

    fn visit_statement(
        &mut self,
        statement: &rustc_middle::mir::Statement<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        let function = self.call_stack.peek_mut();
        function.add_statement(statement, &mut self.net);
        self.super_statement(statement, location);
    }

    fn visit_terminator(
        &mut self,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        let function = self.call_stack.peek_mut();
        function.finish_statement_block(&mut self.net);

        match terminator.kind {
            TerminatorKind::Goto { target } => {
                function.goto(target, &mut self.net);
            }
            TerminatorKind::SwitchInt {
                discr: _,
                ref targets,
            } => {
                // Convert the specific type for the targets vector into a `std::collections::Vec`
                // <rustc_middle::mir::terminator::SwitchTargets>
                function.switch_int(targets.all_targets().to_vec(), &mut self.net);
            }
            TerminatorKind::Resume | TerminatorKind::Abort => {
                function.unwind(&self.program_panic, &mut self.net);
            }
            TerminatorKind::Return => {
                function.return_statement(&mut self.net);
            }
            TerminatorKind::Unreachable => {}
            TerminatorKind::Drop {
                place,
                target,
                unwind,
            } => {
                let transition_drop = function.drop(target, unwind, &mut self.net);
                self.handle_lock_guard_drop(place, &transition_drop);
            }
            TerminatorKind::DropAndReplace { .. } => {
                unimplemented!("TerminatorKind::DropAndReplace not implemented yet")
            }
            TerminatorKind::Call {
                ref func,
                ref args,
                destination,
                target,
                cleanup,
                from_hir_call: _,
                fn_span: _,
            } => {
                let function_call =
                    self.prepare_function_call(func, args.clone(), destination, target, cleanup);
                self.call_function(function_call);
            }
            TerminatorKind::Assert {
                cond: _,
                expected: _,
                msg: _,
                target,
                cleanup,
            } => {
                function.assert(target, cleanup, &mut self.net);
            }
            TerminatorKind::Yield { .. } => {
                unimplemented!("TerminatorKind::Yield not implemented yet")
            }
            TerminatorKind::GeneratorDrop => {
                unimplemented!("TerminatorKind::GeneratorDrop not implemented yet")
            }
            TerminatorKind::FalseEdge { .. } => {
                unimplemented!("TerminatorKind::FalseEdge not implemented yet")
            }
            TerminatorKind::FalseUnwind { .. } => {
                unimplemented!("TerminatorKind::FalseUnwind not implemented yet")
            }
            TerminatorKind::InlineAsm { .. } => {
                unimplemented!("TerminatorKind::InlineAsm not implemented yet")
            }
        }
        self.super_terminator(terminator, location);
    }
}
