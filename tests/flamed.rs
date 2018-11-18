//! Test basic `flamer` usage.

#![cfg(feature = "test-nightly")]
#![feature(proc_macro_hygiene)]

extern crate flame;
#[macro_use] extern crate flamer;

#[flame]
mod inner {
    fn e() -> u32 {
        2
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

    pub fn a() -> u32 {
        let mut result = 0;
        for _ in 0..3 {
            result += b()
        }
        result / 10
    }

    #[noflame]
    #[allow(unused)]
    mod flamed {
        pub fn id() {
        }
    }
}

use inner::*;

#[test]
fn test_flame() {
    assert_eq!(459, a());
    let spans = flame::spans();
    assert_eq!(1, spans.len());
    let roots = &spans[0];
    assert_eq!(3, roots.children.len());
}
