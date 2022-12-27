//! Submodule for the main translation logic.
mod function;
mod local;
mod mutex;
mod mutex_manager;
mod naming;
mod special_function;

use crate::stack::Stack;
use crate::translator::function::Function;
use crate::translator::local::Local;
use crate::translator::mutex_manager::MutexManager;
use crate::translator::naming::{
    function_transition_label_from_function_name, PROGRAM_END, PROGRAM_PANIC, PROGRAM_START,
};
use crate::translator::special_function::SUPPORTED_SPECIAL_FUNCTIONS;
use netcrab::petri_net::{PetriNet, PlaceRef};

use rustc_middle::ty::TyCtxt;

pub struct Translator {
    err_str: Option<&'static str>,
    net: PetriNet,
    program_start: PlaceRef,
    program_end: PlaceRef,
    program_panic: PlaceRef,
    call_stack: Stack<Function>,
    mutex_manager: MutexManager,
}

impl Translator {
    /// Create a new `Translator`.
    /// The initial Petri net contains three places representing the program start state,
    /// the program end state and the abnormal end state after `panic!()`.
    pub fn new() -> Self {
        let mut net = PetriNet::new();
        let program_panic = net.add_place(PROGRAM_PANIC);
        let program_end = net.add_place(PROGRAM_END);
        let program_start = net.add_place(PROGRAM_START);
        net.add_token(&program_start, 1).expect(
            "BUG: Adding initial token to empty PROGRAM_START place should not cause an overflow",
        );

        Self {
            err_str: None,
            net,
            program_start,
            program_end,
            program_panic,
            call_stack: Stack::new(),
            mutex_manager: MutexManager::new(),
        }
    }

    /// Get the result of the translation, i.e. the Petri net.
    /// The ownership is transferred to the caller.
    ///
    /// # Errors
    ///
    /// If the translation failed, then an error is returned.
    pub fn get_result(&mut self) -> Result<PetriNet, &'static str> {
        match self.err_str {
            Some(err_str) => Err(err_str),
            // We do not want to panic here. The user should call `has_err()` first.
            None => Ok(std::mem::take(&mut self.net)),
        }
    }

    /// Set the error string explicitly.
    /// This can be used from outside when the errors happen before the `Translator` is called.
    pub fn set_err_str(&mut self, err_str: &'static str) {
        self.err_str = Some(err_str);
    }

    /// Check whether the function name corresponds to one of the functions
    /// that needs to be translated separately, e.g, mutex functions.
    fn is_special_function(function_name: &str) -> bool {
        for name in SUPPORTED_SPECIAL_FUNCTIONS {
            if function_name == name {
                return true;
            }
        }
        false
    }

    /// Translate the source code to a Petri net.
    ///
    /// # Errors
    ///
    /// If the translation fails, then an error message is set.
    pub fn run<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        let Some((main_function_id, _)) = tcx.entry_fn(()) else {
            self.set_err_str("No main function found in the given source code");
            return;
        };
        let main_return_value = Local::new();
        self.translate_function(
            tcx,
            main_function_id,
            Vec::new(),
            main_return_value,
            self.program_start.clone(),
            self.program_end.clone(),
        )
    }

    fn translate_function<'tcx>(
        &mut self,
        tcx: TyCtxt<'tcx>,
        function_id: rustc_hir::def_id::DefId,
        function_args: Vec<Local>,
        function_return_value: Local,
        start_place: PlaceRef,
        end_place: PlaceRef,
    ) {
        let function_name = tcx.def_path_str(function_id);
        self.net.add_place(&function_name);
        if Self::is_special_function(&function_name) {
            self.translate_special_function(
                function_args,
                function_return_value,
                start_place,
                end_place,
                &function_name,
            );
        } else {
            self.set_err_str("Translation of default function not implemented yet");
            return;
        }
    }

    fn translate_special_function(
        &mut self,
        function_args: Vec<Local>,
        function_return_value: Local,
        start_place: PlaceRef,
        end_place: PlaceRef,
        function_name: &str,
    ) {
        // Create a transition that represents the function call and connect through it the start and end places.
        let transition_ref = self
            .net
            .add_transition(&function_transition_label_from_function_name(function_name));
        self.net
            .add_arc_place_transition(&start_place, &transition_ref)
            .expect("BUG: Adding an arc should not fail");
        self.net
            .add_arc_transition_place(&transition_ref, &end_place)
            .expect("BUG: Adding an arc should not fail");

        match function_name {
            "std::sync::Mutex::<T>::new" => {
                // The return value of this function (a `Local`)
                // should be linked to the mutex.
                // Double check this, since there is nothing else to do.
                if let Err(err_str) = self
                    .mutex_manager
                    .get_mutex_from_local(&function_return_value)
                {
                    panic!("{err_str}");
                };
            }
            "std::sync::Mutex::<T>::lock" => {
                // The first argument to this function is the mutex to be locked.
                // The function return value is a new variable with a MutexGuard.
                if let Err(err_str) = self.mutex_manager.add_lock_guard(
                    function_return_value,
                    function_args[0].clone(),
                    transition_ref,
                    &mut self.net,
                ) {
                    panic!("{err_str}");
                }
            }
            "std::sync::Mutex::<T>::try_lock" => {
                todo!("std::sync::Mutex::<T>::try_lock cannot be translated yet");
            }
            _ => unimplemented!("BUG: Unhandled unique function"),
        };
    }

    /// Example translation
    /// Iterate over the top-level items in the crate, looking for the main function.
    pub fn print_all_expr_hir<'tcx>(
        &self,
        tcx: TyCtxt<'tcx>,
        hir_krate: rustc_middle::hir::map::Map,
    ) {
        for id in hir_krate.items() {
            let item = hir_krate.item(id);
            // Use pattern-matching to find a specific node inside the main function.
            if let rustc_hir::ItemKind::Fn(_, _, body_id) = item.kind {
                let expr = tcx.hir().body(body_id).value;
                if let rustc_hir::ExprKind::Block(block, _) = expr.kind {
                    if let rustc_hir::StmtKind::Local(local) = block.stmts[0].kind {
                        if let Some(expr) = local.init {
                            let hir_id = expr.hir_id; // hir_id identifies the string "Hello, world!"
                            let def_id = tcx.hir().local_def_id(item.hir_id()); // def_id identifies the main function
                            let ty = tcx.typeck(def_id).node_type(hir_id);
                            println!("{expr:#?}: {ty:?}");
                        }
                    }
                }
            }
        }
    }
}
