# intel_tex
========

[![Latest version](https://img.shields.io/crates/v/intel-tex.svg)](https://crates.io/crates/intel-tex)
[![Documentation](https://docs.rs/intel-tex/badge.svg)](https://docs.rs/intel-tex)
[![](https://tokei.rs/b1/github/gwihlidal/intel-tex-rs)](https://github.com/gwihlidal/intel-tex-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![APACHE2](https://img.shields.io/badge/license-APACHE2-blue.svg)

Rust bindings for Intel's ISPC texture compressor.

* https://github.com/GameTechDev/ISPCTextureCompressor

State of the art texture compression for BC6H, BC7, ETC1, ASTC and BC1/BC3.

Requirements:

* ISPC compiler to be installed
  * https://ispc.github.io/
* Also need `libclang` installed (for rust-bindgen)
  * https://rust-lang.github.io/rust-bindgen/requirements.html

In the future, it may be possible to try and package ISPC with the crate (embedded, curl'd, etc..), but crates.io has a 10mb crate limit, so some thought is needed there. In addition to ISPC, the rust bindings to the generated code (provided by the ISPC crate) also needs rust bindgen, which requires libclang to be installed. This dependency is likely easier to break, if the ispc-rs crate is modified to support using a pre-generated FFI binding instead of always generating in build.rs.

For convenience, ISPC binaries for macOS, Linux, and Windows are in the repository (but not the crate).
Additionally, libclang exists in the LLVM installer for Windows, also included.
https://github.com/gwihlidal/intel-tex-rs/tree/master/dependencies

## Supported compression formats:

* BC1, BC3 (aka DXT1, DXT5)
* BC6H (FP16 HDR input)
* BC7
* ETC1

## Pending compression formats:

* ASTC (LDR, block sizes up to 8x8)
    * Work in progress

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
intel_tex = "0.1.2"
```

and add this to your crate root:

```rust
extern crate intel_tex;
```

## Example

```shell
$ cargo run --release --example main

Width is 4096
Height is 4096
ColorType is RGB(8)
Converting RGB -> RGBA
Block count: 1048576
Compressing to BC7...
  Done!
Saving lambertian.dds file
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

Contributions are always welcome; please look at the [issue tracker](https://github.com/gwihlidal/intel-tex-rs/issues) to see what
known improvements are documented.

## Code of Conduct

Contribution to the intel-tex crate is organized under the terms of the
Contributor Covenant, the maintainer of intel-tex, @gwihlidal, promises to
intervene to uphold that code of conduct.