// test double attrs

#![feature(plugin)]
#![plugin(flamer)]
#![flame]

extern crate flame;

#[flame]
fn a() {
    // nothing to do here
}

fn b() {
    a()
}

#[noflame]
fn c() {
    b()
}

#[test]
fn main() {
    c();
    let frames = flame::frames();
    assert_eq!(1, frames.len());
    let roots = &frames[0].roots;
    // if more than 3 roots, a() was flamed twice or c was flamed
    assert_eq!(3, roots.len()); // 1 for main, a and b each
}
