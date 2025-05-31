use std::fs;
use syn::visit_mut::{self, VisitMut};
use syn::{Block, File, parse_file};
struct FunctionBodyRemover;

impl VisitMut for FunctionBodyRemover {
    fn visit_impl_item_fn_mut(&mut self, func: &mut syn::ImplItemFn) {
        func.block = Block {
            brace_token: syn::token::Brace::default(),
            stmts: Vec::new(),
        };

        visit_mut::visit_impl_item_fn_mut(self, func);
    }

    fn visit_item_fn_mut(&mut self, func: &mut syn::ItemFn) {
        func.block = Box::new(Block {
            brace_token: syn::token::Brace::default(),
            stmts: Vec::new(),
        });

        visit_mut::visit_item_fn_mut(self, func);
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Pass a filename to a rust file");
    let code = fs::read_to_string(path).expect("Unable to read file");
    let mut syntax_tree: File = parse_file(&code).expect("Unable to parse file");
    let mut remover = FunctionBodyRemover;
    remover.visit_file_mut(&mut syntax_tree);
    let formatted_code = prettyplease::unparse(&syntax_tree);
    println!("{}", formatted_code);
}
