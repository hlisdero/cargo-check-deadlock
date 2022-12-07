use netcrab::petri_net::PetriNet;
use rustc_middle::ty::TyCtxt;

const PROGRAM_START: &'static str = "PROGRAM_START";
const PROGRAM_END: &'static str = "PROGRAM_END";
const PROGRAM_PANIC: &'static str = "PROGRAM_PANIC";

pub struct Translator {
    net: PetriNet,
    err_str: Option<&'static str>,
}

impl Translator {
    /// Create a new `Translator`.
    /// The initial Petri net contains three places representing the program start state,
    /// the program end state and the abnormal end state after `panic!()`.
    pub fn new() -> Self {
        let mut net = PetriNet::new();
        net.add_place(PROGRAM_PANIC);
        net.add_place(PROGRAM_END);
        let program_start = net.add_place(PROGRAM_START);
        net.add_token(&program_start, 1).expect(
            "BUG: Adding initial token to empty PROGRAM_START place should not cause an overflow",
        );

        Self {
            net: net,
            err_str: None,
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
    /// This is used when the errors happen before the `Translator` is called.
    pub fn set_err_str(&mut self, err_str: &'static str) {
        self.err_str = Some(err_str);
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