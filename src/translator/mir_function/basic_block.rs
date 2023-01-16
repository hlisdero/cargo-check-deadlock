//! Representation of a basic block of a MIR function in the Petri net.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
//!
//! The `BasicBlock` stores one reference to the start and end place in the Petri net.
//! It also stores a vector of `Statement` which forms a chain of places and transitions.

use crate::error_handling::handle_err_add_arc;
use crate::naming::basic_block::{
    assert_cleanup_transition_label, assert_transition_label, drop_transition_label,
    drop_unwind_transition_label, end_place_label, goto_transition_label,
    switch_int_transition_label, unwind_transition_label,
};
use crate::translator::mir_function::statement::Statement;
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

pub struct BasicBlock {
    /// Name of the function to which this block belongs.
    function_name: String,
    /// Index of this block in the current function.
    index: usize,
    /// The start place of this basic block in the Petri net.
    pub start_place: PlaceRef,
    /// The end place of this basic block in the Petri net, before the terminator.
    pub end_place: PlaceRef,
    statements: Vec<Statement>,
}

impl BasicBlock {
    /// Creates a new basic block and adds its representation to the Petri net.
    /// Assumes that the end place is the same as the start place until a statement is added.
    pub fn new(function_name: &str, index: usize, start_place: PlaceRef) -> Self {
        Self {
            function_name: function_name.to_string(),
            index,
            start_place: start_place.clone(),
            end_place: start_place,
            statements: Vec::new(),
        }
    }

    /// Adds a statement to the basic block.
    pub fn add_statement(&mut self, statement: &rustc_middle::mir::Statement, net: &mut PetriNet) {
        let start_place = self.prepare_start_place_next_statement(net);
        let statement_index = self.statements.len();
        self.statements.push(Statement::new(
            &self.function_name,
            self.index,
            statement_index,
            statement,
            &start_place,
            net,
        ));
    }

    /// Connects the last statement transition to the end place of this basic block. The end place is created here.
    /// In case there are no statements in this block, the end place continues to be the same
    /// as the start place, so there is nothing to do.
    pub fn finish_statement_block(&mut self, net: &mut PetriNet) {
        if let Some(statement) = self.statements.last() {
            let label = end_place_label(&self.function_name, self.index);
            self.end_place = net.add_place(&label);
            statement.connect_to_end_place(&self.end_place, net);
        }
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    pub fn goto(&self, target: &Self, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &target.start_place,
            net,
            &goto_transition_label(&self.function_name, self.index),
            "goto transition",
            "to block start place",
        );
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    pub fn switch_int(&self, target: &Self, index: usize, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &target.start_place,
            net,
            &switch_int_transition_label(&self.function_name, index),
            "switch int transition",
            "target block start place",
        );
    }

    /// Connects the end place of this block to the unwind place.
    pub fn unwind(&self, unwind_place: &PlaceRef, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            unwind_place,
            net,
            &unwind_transition_label(&self.function_name, self.index),
            "unwind transition",
            "unwind place",
        );
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    /// Returns the new transition created to connect the two basic blocks.
    pub fn drop(&self, target: &Self, net: &mut PetriNet) -> TransitionRef {
        let transition =
            net.add_transition(&drop_transition_label(&self.function_name, self.index));
        net.add_arc_place_transition(&self.end_place, &transition)
            .unwrap_or_else(|_| handle_err_add_arc("end place of the block", "drop transition"));
        net.add_arc_transition_place(&transition, &target.start_place)
            .unwrap_or_else(|_| handle_err_add_arc("drop transition", "target block start place"));
        transition
    }

    /// Connects the end place of this block to the start place of the `unwind` basic block.
    pub fn drop_unwind(&self, unwind: &Self, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &unwind.start_place,
            net,
            &drop_unwind_transition_label(&self.function_name, self.index),
            "drop unwind transition",
            "unwind block start place",
        );
    }

    /// Connects the end place of this block to the start place of the `assert` basic block.
    pub fn assert(&self, target: &Self, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &target.start_place,
            net,
            &assert_transition_label(&self.function_name, self.index),
            "assert transition",
            "target block start place",
        );
    }

    /// Connects the end place of this block to the start place of the `cleanup` basic block.
    pub fn assert_cleanup(&self, cleanup: &Self, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &cleanup.start_place,
            net,
            &assert_cleanup_transition_label(&self.function_name, self.index),
            "assert cleanup transition",
            "cleanup block start place",
        );
    }

    /// Connects the end place of this block to the start place of the next basic block.
    /// This is the generic function used to implement the handlers for the other terminators.
    fn connect_end_to_next_place(
        &self,
        next_place: &PlaceRef,
        net: &mut PetriNet,
        transition_label: &str,
        transition_name: &str,
        next_place_name: &str,
    ) {
        let transition = net.add_transition(transition_label);
        net.add_arc_place_transition(&self.end_place, &transition)
            .unwrap_or_else(|_| handle_err_add_arc("end place of the block", transition_name));
        net.add_arc_transition_place(&transition, next_place)
            .unwrap_or_else(|_| handle_err_add_arc(transition_name, next_place_name));
    }

    /// Prepares the start place for the next statement.
    /// If it is the first statement, then the start place of the block
    /// is the same as the start place of the statement.
    /// Otherwise the end place of the last statement corresponds to
    /// the start place of the new statement, forming a chain of statements.
    #[inline]
    fn prepare_start_place_next_statement(&self, net: &mut PetriNet) -> PlaceRef {
        self.statements.last().map_or_else(
            || self.start_place.clone(),
            |statement| statement.create_end_place(net),
        )
    }
}
