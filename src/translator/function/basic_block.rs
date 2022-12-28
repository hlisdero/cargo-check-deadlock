//! Representation of a basic block of a MIR function in the Petri net.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
//!
//! The `BasicBlock` stores one reference to the start and end place in the Petri net.
//! It also stores a vector of `Statement` which form a chain of places and transitions.
use crate::translator::function::statement::Statement;
use crate::translator::naming::{BASIC_BLOCK_EMPTY, BASIC_BLOCK_END_PLACE};
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
        net.add_arc_place_transition(&self.start_place, &transition_empty).expect(
            "BUG: Adding an arc from the start place to the empty basic block transition should not fail",
        );
        net.add_arc_transition_place(&transition_empty, &self.end_place).expect(
            "BUG: Adding an arc from the empty basic block transition to the end place should not fail",
        );
    }
}
