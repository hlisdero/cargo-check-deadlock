//! Enum which defines all the different types of function calls that the translator supports.
//! Each variant contains all the information necessary to translate the function call.

use netcrab::petri_net::PlaceRef;

pub enum FunctionCall<'tcx> {
    /// Call to a function which does not return (Return type: -> !).
    /// Non-recursive call for the translation process.
    Diverging {
        function_name: String,
        start_place: PlaceRef,
    },
    /// MIR function call (the "default" case).
    /// Recursive call for the translation process.
    MirFunction {
        function_def_id: rustc_hir::def_id::DefId,
        start_place: PlaceRef,
        end_place: PlaceRef,
    },
    /// Abridged function call.
    /// Non-recursive call for the translation process.
    Foreign {
        function_name: String,
        start_place: PlaceRef,
        end_place: PlaceRef,
        cleanup_place: Option<PlaceRef>,
    },
    /// Call to a mutex synchronization primitive.
    /// Non-recursive call for the translation process.
    Mutex {
        function_name: String,
        args: Vec<rustc_middle::mir::Operand<'tcx>>,
        destination: rustc_middle::mir::Place<'tcx>,
        start_place: PlaceRef,
        end_place: PlaceRef,
        cleanup_place: Option<PlaceRef>,
    },
    /// Function call which starts an abnormal termination of the program.
    /// Non-recursive call for the translation process.
    Panic {
        function_name: String,
        start_place: PlaceRef,
    },
}
