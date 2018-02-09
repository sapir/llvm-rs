LLVM-rs
=======
[![Build Status](https://travis-ci.org/Lapz/llvm-rs.svg?branch=master)](https://travis-ci.org/Lapz/llvm-rs)
[![Dependency Status](https://dependencyci.com/github/Lapz/llvm-rs/badge)](https://dependencyci.com/github/Lapz/llvm-rs)

This is a library that wraps [LLVM](http://llvm.org) using Rust idioms and the cbox library. There is
[good quality documentation available](https://lapz.github.io/llvm-rs/) if you
want to check out the API. It's basically a simplified version of the C++ API which has
[documentation](http://llvm.org/doxygen).

Using in your projects
----------------------
To use this in your project, add the following to your `Cargo.toml`

```toml
[dependencies]
...
llvm-rs = { git ="https://github.com/lapz/llvm-rs/"}
```

Bugs
----
While using this library, if you ever encounter a segmentation fault or similar unverbose error messages, please file an issue with what code caused the problem. Thanks!

Notes
---- 
This is a fork of the original bindings that were written by [TomBebb](https://github.com/TomBebb/llvm-rs) that has been updated and with more bindings written.

