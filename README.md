# rustos

[![Build Code](https://github.com/gky360/rustos/workflows/Build%20Code/badge.svg)](https://github.com/gky360/rustos/actions)

Writing an OS in Rust.
RustでOS自作入門。

### Development

For development in vscode, follow the steps below.

1. Remove the following line from `.cargo/config`.

```
[unstable]
build-std = ["core", "compiler_builtins", "alloc"]
```

2. Run `cargo install cargo-xbuild && cargo xbuild`. This will generate `target/sysroot` dir that is needed for rls for vscode.
3. Revert changes you made to `.cargo/config`.

### References

The following posts and books gave me great help to go through this project.

- [Writing an OS in Rust](https://os.phil-opp.com/)
    - enabling to run Rust code on the bare metal without an underlying operating system
    - creating a bootable disk image
    - enabling unit and integration testing
- [30日でできる！OS自作入門](https://book.mynavi.jp/supportsite/detail/4839919844.html)
    - all chapters after Chapter 3
- [「30日でできる！OS自作入門」をRustで。 - TSUGULOG](https://yoshitsugu.net/tags/OS%E8%87%AA%E4%BD%9C%E5%85%A5%E9%96%80.html)
    - filling the gap between the post "Writing an OS in Rust" and the book "30日でできる！OS自作入門"
- [x86_64 crate](https://github.com/rust-osdev/x86_64)
    - supporting x86_64 specific instructions, registers, and structures
