A plugin to insert appropriate `flame::start_guard(_)` calls (for use with [flame](https://github.com/TyOverby/flame))

[![Build Status](https://travis-ci.org/llogiq/flamer.svg)](https://travis-ci.org/llogiq/flamer) 
[![Current Version](http://meritbadge.herokuapp.com/flamer)](https://crates.io/crates/flamer)

**This needs a nightly rustc!** Because flamer is a compiler plugin, it uses unstable APIs, which are not available on stable or beta. It may be possible to extend flamer to allow use with syntex, but this hasn't been tried yet.

Usage:

In your Cargo.toml add `flame = "*"` to your `[dependencies]` (You may also opt for an optional dependency). Then in your crate root, add the following:

```
#![feature(plugin)]
#![plugin(flamer)]
```

You should then be able to annotate every item (or even the whole crate) with `#[flame]` annotations. Note that this only instruments the annotated methods, it does not print out the results.

Refer to flame's documentation to see how output works.

License: Apache 2.0

