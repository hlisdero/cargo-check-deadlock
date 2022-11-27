use netcrab::petri_net::PetriNet;
use rustc_middle::ty::TyCtxt;

pub struct Translator<'tcx> {
    tcx: TyCtxt<'tcx>,
    net: PetriNet,
}

impl<'tcx> Translator<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        let net = PetriNet::new();
        Self { tcx, net }
    }

    // Iterate over the top-level items in the crate, looking for the main function.
    pub fn print_all_expr_hir(&self, hir_krate: rustc_middle::hir::map::Map) {
        for id in hir_krate.items() {
            let item = hir_krate.item(id);
            // Use pattern-matching to find a specific node inside the main function.
            if let rustc_hir::ItemKind::Fn(_, _, body_id) = item.kind {
                let expr = self.tcx.hir().body(body_id).value;
                if let rustc_hir::ExprKind::Block(block, _) = expr.kind {
                    if let rustc_hir::StmtKind::Local(local) = block.stmts[0].kind {
                        if let Some(expr) = local.init {
                            let hir_id = expr.hir_id; // hir_id identifies the string "Hello, world!"
                            let def_id = self.tcx.hir().local_def_id(item.hir_id()); // def_id identifies the main function
                            let ty = self.tcx.typeck(def_id).node_type(hir_id);
                            println!("{expr:#?}: {ty:?}");
                        }
                    }
                }
            }
        }
    }
}
