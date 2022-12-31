//! Representation of a basic block of a MIR function in the Petri net.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
//!
//! The `BasicBlock` stores one reference to the start and end place in the Petri net.
//! It also stores a vector of `Statement` which form a chain of places and transitions.
use crate::translator::error_handling::format_err_str_add_arc;
use crate::translator::function::statement::Statement;
use crate::translator::naming::{
    function_switch_int_transition_label_from_block_index, BASIC_BLOCK_ASSERT,
    BASIC_BLOCK_ASSERT_CLEANUP, BASIC_BLOCK_DROP, BASIC_BLOCK_DROP_UNWIND, BASIC_BLOCK_EMPTY,
    BASIC_BLOCK_END_PLACE, BASIC_BLOCK_GOTO, BASIC_BLOCK_UNWIND,
};
use netcrab::petri_net::{PetriNet, PlaceRef};

pub struct BasicBlock {
    /// The start place of this basic block in the Petri net.
    pub start_place: PlaceRef,
    /// The end place of this basic block in the Petri net, before the terminator.
    pub end_place: PlaceRef,
    statements: Vec<Statement>,
}

impl BasicBlock {
    /// Creates a new basic block and adds its representation to the Petri net.
    pub fn new(start_place: PlaceRef, net: &mut PetriNet) -> Self {
        let end_place = net.add_place(BASIC_BLOCK_END_PLACE);

        Self {
            start_place,
            end_place,
            statements: Vec::new(),
        }
    }

    /// Adds a statement to the basic block.
    pub fn add_statement(&mut self, statement: &rustc_middle::mir::Statement, net: &mut PetriNet) {
        let start_place = self.prepare_start_place_next_statement(net);
        self.statements
            .push(Statement::new(statement, &start_place, net));
    }

    /// Connects the last statement transition to the end place of this basic block.
    /// In case there are no statements, an additional transition is created to
    /// connect the start and end place of this basic block.
    pub fn finish_statement_block(&self, net: &mut PetriNet) {
        if let Some(statement) = self.statements.last() {
            statement.connect_to_end_place(&self.end_place, net);
        } else {
            self.handle_basic_block_with_no_statements(net);
        }
    }

    /// Connects the end place of this block to the start place of the target basic block.
    pub fn goto(&self, target: &BasicBlock, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &target.start_place,
            net,
            BASIC_BLOCK_GOTO,
            "goto transition",
            "to block start place",
        );
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    pub fn switch_int(&self, target: &BasicBlock, index: usize, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &target.start_place,
            net,
            &function_switch_int_transition_label_from_block_index(index),
            "switch int transition",
            "target block start place",
        );
    }

    /// Connects the end place of this block to the unwind place.
    pub fn unwind(&self, unwind_place: PlaceRef, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &unwind_place,
            net,
            BASIC_BLOCK_UNWIND,
            "unwind transition",
            "unwind place",
        );
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    pub fn drop(&self, target: &BasicBlock, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &target.start_place,
            net,
            BASIC_BLOCK_DROP,
            "drop transition",
            "target block start place",
        );
    }

    /// Connects the end place of this block to the start place of the `unwind` basic block.
    pub fn drop_unwind(&self, unwind: &BasicBlock, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &unwind.start_place,
            net,
            BASIC_BLOCK_DROP_UNWIND,
            "drop unwind transition",
            "unwind block start place",
        );
    }

    /// Connects the end place of this block to the start place of the `assert` basic block.
    pub fn assert(&self, target: &BasicBlock, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &target.start_place,
            net,
            BASIC_BLOCK_ASSERT,
            "assert transition",
            "target block start place",
        );
    }

    /// Connects the end place of this block to the start place of the `cleanup` basic block.
    pub fn assert_cleanup(&self, cleanup: &BasicBlock, net: &mut PetriNet) {
        self.connect_end_to_next_place(
            &cleanup.start_place,
            net,
            BASIC_BLOCK_ASSERT_CLEANUP,
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
        let transition_cleanup = net.add_transition(transition_label);
        net.add_arc_place_transition(&self.end_place, &transition_cleanup)
            .expect(&format_err_str_add_arc(
                "end place of the block",
                transition_name,
            ));
        net.add_arc_transition_place(&transition_cleanup, &next_place)
            .expect(&format_err_str_add_arc(transition_name, next_place_name));
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

    /// Creates an extra transition and connects the start and end place through it.
    /// This "fake statement" exists only in the model and not in the Rust source code.
    fn handle_basic_block_with_no_statements(&self, net: &mut PetriNet) {
        // if there is only a terminator (no statement) we have to connect start and end place of the block
        let transition_empty = net.add_transition(BASIC_BLOCK_EMPTY);
        net.add_arc_place_transition(&self.start_place, &transition_empty)
            .expect(&format_err_str_add_arc(
                "start place",
                "empty basic block transition",
            ));
        net.add_arc_transition_place(&transition_empty, &self.end_place)
            .expect(&format_err_str_add_arc(
                "empty basic block transition",
                "end place",
            ));
    }
}
