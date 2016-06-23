#![feature(plugin_registrar, quote, rustc_private)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::registry::Registry;
use syntax::ast::{Attribute, Block, Expr, ExprKind, Ident, Item, ItemKind, Mac,
                  MetaItem, MetaItemKind};
use syntax::fold::{self, Folder};
use syntax::ptr::P;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::ext::base::{Annotatable, ExtCtxt, SyntaxExtension};
use syntax::ext::build::AstBuilder;
use syntax::ext::expand::{expand_expr, expand_item};
use syntax::feature_gate::AttributeType;
use syntax::parse::token;
use syntax::util::small_vector::SmallVector;

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
    fn fold_item(&mut self, item: P<Item>) -> SmallVector<P<Item>> {
        if let ItemKind::Mac(_) = item.node {
            let expanded = expand_item(item, &mut self.cx.expander());
            expanded.into_iter()
                    .flat_map(|i| fold::noop_fold_item(i, self).into_iter())
                    .collect()
        } else {
            fold::noop_fold_item(item, self)
        }
    }

    fn fold_item_simple(&mut self, i: Item) -> Item {
        fn is_flame_annotation(attr: &Attribute) -> bool {
            if let MetaItemKind::Word(ref name) = attr.node.value.node {
                name == "flame" || name == "noflame"
            } else {
                false
            }
        }
        // don't double-flame nested annotations
        if i.attrs.iter().any(is_flame_annotation) { return i; }
        if let ItemKind::Mac(_) = i.node {
            return i;
        }else {
            self.ident = i.ident; // update in case of nested items
            fold::noop_fold_item_simple(i, self)
        }
    }

    fn fold_block(&mut self, block: P<Block>) -> P<Block> {
        block.map(|block| {
            let name = self.cx.expr_str(DUMMY_SP, self.ident.name.as_str());
            quote_block!(self.cx, {
                let g = ::flame::start_guard($name);
                let r = $block;
                g.end();
                r
            }).unwrap()
        })
    }

    fn fold_expr(&mut self, expr: P<Expr>) -> P<Expr> {
        if let ExprKind::Mac(_) = expr.node {
            expand_expr(expr.unwrap(), &mut self.cx.expander()).map(|e|
                fold::noop_fold_expr(e, self))
        } else {
            expr
        }
    }

    fn fold_mac(&mut self, mac: Mac) -> Mac {
        mac
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("flame"),
        SyntaxExtension::MultiModifier(Box::new(insert_flame_guard)));
    reg.register_attribute(String::from("noflame"), AttributeType::Whitelisted);
}
