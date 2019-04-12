# Rust Configuration Example

One approach to global config in Rust programs.

In most of the programs I have written so far the configuration consists
of a combination of defaults, file configuration and command line options,
in that order of precedence. And once established, it is usually
immutable.

This repo demonstrates one way of setting this up in an ergomomic
fashion. It uses
[lazy_static](https://crates.io/crates/lazy_static)
to create a global singleton instance
of a `Configuration` struct, which can then be referred to anywhere
in the program without having to pass it in everywhere.

See also my [blog post about this](https://www.philipdaniels.com/blog/2019/rust-configuration-example/).
