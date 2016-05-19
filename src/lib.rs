#![feature(plugin_registrar, quote, rustc_private)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::registry::Registry;
use syntax::ast::{Block, Ident, Item, MetaItem};
use syntax::fold::{self, Folder};
use syntax::ptr::P;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::ext::base::{Annotatable, ExtCtxt, SyntaxExtension};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;

pub fn insert_flame_guard(cx: &mut ExtCtxt, _span: Span, _mi: &MetaItem,
                          a: Annotatable) -> Annotatable {
    match a {
        Annotatable::Item(i) => Annotatable::Item(
            Flamer { cx: cx, ident: i.ident }.fold_item(i).expect_one("expected exactly one item")),
        Annotatable::TraitItem(i) => Annotatable::TraitItem(
            i.map(|i| Flamer { cx: cx, ident: i.ident }.fold_trait_item(i).expect_one("expected exactly one item"))),
        Annotatable::ImplItem(i) => Annotatable::ImplItem(
            i.map(|i| Flamer { cx: cx, ident: i.ident }.fold_impl_item(i).expect_one("expected exactly one item"))),
    }
}

struct Flamer<'a, 'cx: 'a> {
    ident: Ident,
    cx: &'a mut ExtCtxt<'cx>,
}

impl<'a, 'cx> Folder for Flamer<'a, 'cx> {
    fn fold_item_simple(&mut self, i: Item) -> Item {
        self.ident = i.ident; // update in case of nested items
        fold::noop_fold_item_simple(i, self)
    }

    fn fold_block(&mut self, block: P<Block>) -> P<Block> {
        block.map(|block| {
            let name = self.cx.expr_str(DUMMY_SP, self.ident.name.as_str());
            quote_block!(self.cx, {
                let _ = ::flame::start_guard($name);
                $block
            }).unwrap()
        })
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("flame"),
        SyntaxExtension::MultiModifier(Box::new(insert_flame_guard)));
}
