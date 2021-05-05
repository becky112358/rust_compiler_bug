# rust_compiler_bug

Attempting to compile this code produces the following:

```
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler\rustc_mir\src\borrow_check\region_infer\mod.rs:2120:35
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0 (e1884a8e3 2020-12-29) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `EXAMPLE_MAP`
#1 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `rust_compiler_bug`

To learn more, run the command again with --verbose.
```
