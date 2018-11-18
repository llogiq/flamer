//! Test double attrs.

#![cfg(feature = "test-nightly")]
#![feature(proc_macro_hygiene)]

extern crate flame;
#[macro_use] extern crate flamer;

#[flame]
mod inner {
    fn a() {
        // nothing to do here
    }

    #[flame]
    fn b() {
        a()
    }

    #[noflame]
    pub fn c() {
        b()
    }
}

#[test]
fn main() {
    inner::c();
    let spans = flame::spans();
    assert_eq!(1, spans.len());
    let roots = &spans[0];
    println!("{:?}",roots);
    // if more than 2 roots, a() was flamed twice or c was flamed
    // main is missing because main isn't closed here
    assert_eq!("b", roots.name);
    assert_eq!(1, roots.children.len());
    assert_eq!("inner::a", roots.children[0].name);
}
