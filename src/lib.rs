#![feature(plugin_registrar, rustc_private)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::registry::Registry;
use syntax::ast::{Block, DeclKind, DUMMY_NODE_ID, Expr, ExprKind, Ident,
                  ImplItem, ImplItemKind, Item, ItemKind, LitKind, Local,
                  MetaItem, Pat, PatKind, Path, PathParameters, PathSegment,
                  Stmt, StmtKind, StrStyle, TraitItem, TraitItemKind};
use syntax::ptr::P;
use syntax::codemap::{DUMMY_SP, dummy_spanned, Span};
use syntax::ext::base::{Annotatable, ExtCtxt, MultiItemModifier,
                        SyntaxExtension};
use syntax::parse::token;

pub fn insert_flame_guard(_cx: &mut ExtCtxt, _span: Span, _mi: &MetaItem,
                          a: Annotatable) -> Annotatable {
    match a {
        Annotatable::Item(ref i) =>
            Annotatable::Item(P(flame_item(i))),
        Annotatable::TraitItem(ref i) =>
            Annotatable::TraitItem(P(flame_trait_item(i))),
        Annotatable::ImplItem(ref i) =>
            Annotatable::ImplItem(P(flame_impl_item(i))),
    }
}

fn flame_item(i: &Item) -> Item {
    let base = i.clone();
    match i.node {
        ItemKind::Fn(ref decl,
                     unsafety,
                     constness,
                     abi,
                     ref generics,
                     ref block) =>
            Item {
                node: ItemKind::Fn(decl.clone(),
                                   unsafety,
                                   constness,
                                   abi,
                                   generics.clone(),
                                   P(flame_block(i.ident, block))),
                ..base
            },
        _ => base
    }
}

fn flame_trait_item(ti: &TraitItem) -> TraitItem {
    let base = ti.clone();
    match ti.node {
        TraitItemKind::Method(ref sig, Some(ref block)) =>
            TraitItem {
                id: DUMMY_NODE_ID,
                node: TraitItemKind::Method(sig.clone(),
                          Some(P(flame_block(base.ident, block)))),
                .. base
            },
        _ => base
    }
}

fn flame_impl_item(ii: &ImplItem) -> ImplItem {
    let base = ii.clone();
    match ii.node {
        ImplItemKind::Method(ref sig, ref block) =>
            ImplItem {
                node: ImplItemKind::Method(sig.clone(),
                                           P(flame_block(ii.ident, block))),
                .. base
            },
        _ => base
    }
}

fn flame_block(ident: Ident, block: &Block) -> Block {
    let mut base = block.clone();
    let mut stmts = vec![flame_stmt(ident)];
    stmts.append(&mut base.stmts);
    Block {
        stmts: stmts,
        .. base
    }
}

fn segment(s: &str) -> PathSegment {
    PathSegment {
        identifier: Ident::with_empty_ctxt(token::intern(s)),
        parameters: PathParameters::none()
    }
}

fn expr(node: ExprKind) -> Expr {
    Expr {
        id: DUMMY_NODE_ID,
        node: node,
        span: DUMMY_SP,
        attrs: None
    }
}

fn path(names: &[&str]) -> Expr {
    expr(ExprKind::Path(None, Path {
        span: DUMMY_SP,
        global: false,
        segments: names.iter().cloned().map(segment).collect()
    }))
}

fn flame_stmt(ident: Ident) -> Stmt {
    let name = dummy_spanned(LitKind::Str(
        token::InternedString::new_from_name(ident.name), StrStyle::Cooked));
    let init = expr(ExprKind::Call(P(path(&["flame", "start_guard"])),
                    vec![P(expr(ExprKind::Lit(P(name))))]));
    let local = Local {
        pat: P(Pat {
            id: DUMMY_NODE_ID,
            node: PatKind::Wild,
            span: DUMMY_SP
        }),
        ty: None,
        init: Some(P(init)),
        id: DUMMY_NODE_ID,
        span: DUMMY_SP,
        attrs: None
    };
    dummy_spanned(StmtKind::Decl(P(dummy_spanned(DeclKind::Local(P(local)))), 
                                 DUMMY_NODE_ID))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("flame"),
        SyntaxExtension::MultiModifier(
            Box::new(insert_flame_guard) as Box<MultiItemModifier + 'static>));
}
