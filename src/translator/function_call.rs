//! Enum which defines all the different types of function calls that the translator supports.
//! Each variant contains all the information necessary to translate the function call.

use netcrab::petri_net::PlaceRef;

pub enum FunctionCall<'tcx> {
    /// A function which does not return.
    Diverging {
        function_name: String,
        start_place: PlaceRef,
    },
    /// Normal function call: Recursive call for the translation process.
    Default {
        function_def_id: rustc_hir::def_id::DefId,
        start_place: PlaceRef,
        end_place: PlaceRef,
    },
    /// Abridged function call: Non-recursive call for the translation process.
    Foreign {
        function_name: String,
        start_place: PlaceRef,
        end_place: PlaceRef,
        cleanup_place: Option<PlaceRef>,
    },
    /// Call to a mutex synchronization primitive: Non-recursive call for the translation process.
    Mutex {
        function_name: String,
        args: Vec<rustc_middle::mir::Operand<'tcx>>,
        destination: rustc_middle::mir::Place<'tcx>,
        start_place: PlaceRef,
        end_place: PlaceRef,
        cleanup_place: Option<PlaceRef>,
    },
    /// Any function call which triggers an abnormal termination of the program.
    Panic {
        function_name: String,
        start_place: PlaceRef,
    },
}
