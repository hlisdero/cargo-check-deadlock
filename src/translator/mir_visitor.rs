//! Implement the trait `rustc_middle::mir::visit::Visitor` for Translator
//! <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
//!
//! This trait defines a method for visiting every possible MIR element.
//! It is not required to implement every method, only for the elements we care about.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
use crate::translator::Translator;
use rustc_middle::mir::visit::Visitor;

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
        println!("VISIT_BASIC_BLOCK_DATA: {:?} {:?}", block, data);
        self.super_basic_block_data(block, data)
    }

    fn visit_assign(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        rvalue: &rustc_middle::mir::Rvalue<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        println!("VISIT_ASSIGN: {:?} {:?} {:?}", place, rvalue, location);
        self.super_assign(place, rvalue, location);
    }
}
