#![feature(plugin)]
#![plugin(flamer)]
#![flame]

extern crate flame;

use std::fs::File;

fn e() -> u32 {
    1
}

fn d() -> u32 {
    e() << e()
}

fn c() -> u32 {
    d() * d() * d() - 1
}

fn b() -> u32 {
    (0..3).map(|_| c()).fold(0, |x, y| x + y)
}

fn a() -> u32 {
    let mut result = 0;
    for _ in 0..20 {
        result += b()
    }
    result / 10
}


#[test]
fn test_flame() {
    assert_eq!(42, a());
    
    flame::dump_svg(&mut File::create("out.svg").unwrap()).unwrap();
}
