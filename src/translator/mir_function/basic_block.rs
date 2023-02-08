//! Representation of a basic block of a MIR function in the Petri net.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
//!
//! The `BasicBlock` stores one reference to the start and end place in the Petri net.
//! It also stores a vector of `Statement` which forms a chain of places and transitions.

use super::statement::Statement;
use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place,
};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::naming::basic_block::{
    assert_cleanup_transition_label, assert_transition_label, drop_transition_label,
    drop_unwind_transition_label, end_place_label, goto_transition_label, start_place_label,
    switch_int_transition_label, unreachable_transition_label, unwind_transition_label,
};

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
    /// Creates a new start place for the basic block if it is not provided.
    /// Assumes that the end place is the same as the start place until a statement is added.
    pub fn new(
        function_name: &str,
        index: usize,
        start_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) -> Self {
        let start_place = start_place.map_or_else(
            || net.add_place(&start_place_label(function_name, index)),
            |start_place| start_place,
        );
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
        );
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    pub fn switch_int(&self, target: &Self, index: usize, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &target.start_place,
            net,
            &switch_int_transition_label(&self.function_name, index),
        );
    }

    /// Connects the end place of this block to the unwind place.
    pub fn unwind(&self, unwind_place: &PlaceRef, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            unwind_place,
            net,
            &unwind_transition_label(&self.function_name, self.index),
        );
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    /// Returns the new transition created to connect the two basic blocks.
    pub fn drop(&self, target: &Self, net: &mut PetriNet) -> TransitionRef {
        let transition =
            net.add_transition(&drop_transition_label(&self.function_name, self.index));
        add_arc_place_transition(net, &self.end_place, &transition);
        add_arc_transition_place(net, &transition, &target.start_place);
        transition
    }

    /// Connects the end place of this block to the start place of the `unwind` basic block.
    pub fn drop_unwind(&self, unwind: &Self, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &unwind.start_place,
            net,
            &drop_unwind_transition_label(&self.function_name, self.index),
        );
    }

    /// Connects the end place of this block to the start place of the `assert` basic block.
    pub fn assert(&self, target: &Self, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &target.start_place,
            net,
            &assert_transition_label(&self.function_name, self.index),
        );
    }

    /// Connects the end place of this block to the start place of the `cleanup` basic block.
    pub fn assert_cleanup(&self, cleanup: &Self, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &cleanup.start_place,
            net,
            &assert_cleanup_transition_label(&self.function_name, self.index),
        );
    }

    /// Connects the end place of this block to the end place.
    pub fn unreachable(&self, end_place: &PlaceRef, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            end_place,
            net,
            &unreachable_transition_label(&self.function_name, self.index),
        );
    }

    /// Connects the end place of this block to the start place of the next basic block.
    /// This is the generic function used to implement the handlers for the other terminators.
    fn connect_end_to_next_place(
        &self,
        next_place: &PlaceRef,
        net: &mut PetriNet,
        transition_label: &str,
    ) {
        let transition = net.add_transition(transition_label);
        add_arc_place_transition(net, &self.end_place, &transition);
        add_arc_transition_place(net, &transition, next_place);
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
