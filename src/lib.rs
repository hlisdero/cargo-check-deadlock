//! Library code that implements the translation of Rust source code to a Petri net.
//!
//! It uses rustc's query system to navigate the AST.
//!
//! Adapted from the example in the rustc-dev guide:
//! <https://rustc-dev-guide.rust-lang.org/rustc-driver-interacting-with-the-ast.html>
//!
//! NOTE: For the library code to compile, you will need to first run the following:
//!   rustup component add rustc-dev llvm-tools-preview
//!
//! Tested with rustc 1.67.0-nightly (b3bc6bf31 2022-11-24)

// This feature gate is necessary to access the internal crates of the compiler.
// It has existed for a long time and since the compiler internals will never be stabilized,
// the situation will probably stay like this.
// <https://doc.rust-lang.org/unstable-book/language-features/rustc-private.html>
#![feature(rustc_private)]

// Compiler crates need to be imported in this way because they are not published on crates.io.
// These crates are only available when using the nightly toolchain.
extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;

mod compiler_config;
mod sysroot;

/// Entry point for the translation of the Rust code to a Petri net.
///
/// # Errors
///
/// If the `sysroot` cannot be found, then an error is returned.
pub fn run() -> Result<(), &'static str> {
    let sysroot = sysroot::get_from_rustc()?;
    let config = compiler_config::prepare_rustc_config(sysroot);

    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(compiler_query);
    });

    Ok(())
}

/// The query to the compiler
/// The reference to the query lives as long as the query itself.
fn compiler_query<'a>(queries: &'a rustc_interface::Queries<'a>) {
    // Get the global typing context (tcx or TyCtxt), which is the central data structure in the compiler.
    // <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html>
    let Ok(query) = queries.global_ctxt() else {
        panic!("Unable to get the global typing context")
    };

    // Analyze the crate and inspect the types under the cursor.
    query.take().enter(|tcx| {
        // Every compilation contains a single crate.
        let hir_krate = tcx.hir();
        print_all_expr_hir(tcx, hir_krate);
    });
}

// Iterate over the top-level items in the crate, looking for the main function.
fn print_all_expr_hir(tcx: rustc_middle::ty::TyCtxt, hir_krate: rustc_middle::hir::map::Map) {
    for id in hir_krate.items() {
        let item = hir_krate.item(id);
        // Use pattern-matching to find a specific node inside the main function.
        if let rustc_hir::ItemKind::Fn(_, _, body_id) = item.kind {
            let expr = &tcx.hir().body(body_id).value;
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
