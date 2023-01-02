//! Implement the trait `rustc_middle::mir::visit::Visitor` for Translator
//! <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
//!
//! This trait defines a method for visiting every possible MIR element.
//! It is not required to implement every method, only for the elements we care about.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
use crate::translator::error_handling::EMPTY_CALL_STACK;
use crate::translator::Translator;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::TerminatorKind;

use super::special_function::is_panic;

impl<'tcx> Visitor<'tcx> for Translator<'tcx> {
    /// Entered a MIR function block. Do nothing.
    fn visit_body(&mut self, body: &rustc_middle::mir::Body<'tcx>) {
        self.super_body(body);
    }

    /// Entered a basic block. Do nothing
    fn visit_basic_block_data(
        &mut self,
        block: rustc_middle::mir::BasicBlock,
        data: &rustc_middle::mir::BasicBlockData<'tcx>,
    ) {
        let function = self.call_stack.peek_mut().expect(EMPTY_CALL_STACK);
        function.activate_block(block, &mut self.net);
        self.super_basic_block_data(block, data);
    }

    fn visit_assign(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        rvalue: &rustc_middle::mir::Rvalue<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        println!("VISIT_ASSIGN: {place:?} {rvalue:?} {location:?}");
        self.super_assign(place, rvalue, location);
    }

    fn visit_statement(
        &mut self,
        statement: &rustc_middle::mir::Statement<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        let function = self.call_stack.peek_mut().expect(EMPTY_CALL_STACK);
        function.add_statement(statement, &mut self.net);
        self.super_statement(statement, location);
    }

    fn visit_terminator(
        &mut self,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        let function = self.call_stack.peek_mut().expect(EMPTY_CALL_STACK);
        function.finish_statement_block(&mut self.net);

        match &terminator.kind {
            TerminatorKind::Goto { target } => {
                function.goto(*target, &mut self.net);
            }
            TerminatorKind::SwitchInt { discr: _, targets } => {
                // Convert the specific type for the targets vector into a `std::collections::Vec`
                // <rustc_middle::mir::terminator::SwitchTargets>
                function.switch_int(targets.all_targets().to_vec(), &mut self.net);
            }
            TerminatorKind::Resume | TerminatorKind::Abort => {
                function.unwind(self.program_panic.clone(), &mut self.net);
            }
            TerminatorKind::Return => {
                function.return_statement(&mut self.net);
            }
            TerminatorKind::Unreachable => {}
            TerminatorKind::Drop {
                place: _,
                target,
                unwind,
            } => {
                function.drop(*target, *unwind, &mut self.net);
            }
            TerminatorKind::DropAndReplace {
                place: _,
                value: _,
                target: _,
                unwind: _,
            } => unimplemented!("TerminatorKind::DropAndReplace not implemented yet"),
            TerminatorKind::Call {
                func,
                args: _,
                destination: _,
                target,
                cleanup: _,
                from_hir_call: _,
                fn_span: _,
            } => {
                let function_def_id = Self::extract_def_id_of_called_function_from_operand(
                    func,
                    function.def_id,
                    self.tcx,
                );
                let function_name = self.tcx.def_path_str(function_def_id);
                if is_panic(&function_name) {
                    function.unwind(self.program_panic.clone(), &mut self.net);
                } else if self.tcx.is_foreign_item(function_def_id)
                    || !self.tcx.is_mir_available(function_def_id)
                {
                    unimplemented!("Foreign function not implemented yet");
                } else {
                    let Some(return_block) = target else { 
                        unimplemented!("Diverging functions not implemented yet")
                    };
                    let (start_place, end_place) = function
                        .get_start_and_end_place_for_function_call(*return_block, &mut self.net);
                    self.push_function_to_call_stack(function_def_id, start_place, end_place);
                    self.translate_top_call_stack();
                }
            }
            TerminatorKind::Assert {
                cond: _,
                expected: _,
                msg: _,
                target,
                cleanup,
            } => {
                function.assert(*target, *cleanup, &mut self.net);
            }
            TerminatorKind::Yield {
                value: _,
                resume: _,
                resume_arg: _,
                drop: _,
            } => unimplemented!("TerminatorKind::Yield not implemented yet"),
            TerminatorKind::GeneratorDrop => {
                unimplemented!("TerminatorKind::GeneratorDrop not implemented yet")
            }
            TerminatorKind::FalseEdge {
                real_target: _,
                imaginary_target: _,
            } => {
                unimplemented!("TerminatorKind::FalseEdge not implemented yet")
            }
            TerminatorKind::FalseUnwind {
                real_target: _,
                unwind: _,
            } => {
                unimplemented!("TerminatorKind::FalseUnwind not implemented yet")
            }
            TerminatorKind::InlineAsm {
                template: _,
                operands: _,
                options: _,
                line_spans: _,
                destination: _,
                cleanup: _,
            } => {
                unimplemented!("TerminatorKind::InlineAsm not implemented yet")
            }
        }
        self.super_terminator(terminator, location);
    }
}
