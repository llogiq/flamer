// test double attrs

#![feature(plugin, custom_attribute)]
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
    let frames = flame::spans();
    assert_eq!(1, frames.len());
    let roots = &frames[0].roots;
    println!("{:?}",roots);
    // if more than 2 roots, a() was flamed twice or c was flamed
    // main is missing because main isn't closed here
    assert_eq!(1, roots.len());
    assert_eq!("b", roots[0].name);
    assert_eq!(1, roots[0].children.len());
    assert_eq!("a", roots[0].children[0].name);
}
