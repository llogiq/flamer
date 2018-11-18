//! Test optional prefix.

extern crate flame;
extern crate flamer;

use flamer::{flame, noflame};

#[flame("top")]
fn a() {
    let l = Lower {};
    l.a();
}

#[flame]
fn b() {
    a()
}

#[noflame]
fn c() {
    b()
}

pub struct Lower;

impl Lower {
    #[flame("lower")]
    pub fn a(self) {
        // nothing to do here
    }
}

#[test]
fn main() {
    c();
    let spans = flame::spans();
    assert_eq!(1, spans.len());
    let roots = &spans[0];
    println!("{:?}",roots);
    // if more than 2 roots, a() was flamed twice or c was flamed
    // main is missing because main isn't closed here
    assert_eq!("b", roots.name);
    assert_eq!(1, roots.children.len());
    assert_eq!("top::a", roots.children[0].name);
    assert_eq!(1, roots.children[0].children.len());
    assert_eq!("lower::a", roots.children[0].children[0].name);
}
