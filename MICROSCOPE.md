# microscope's cpp_demangle fork

The `microscope-cpp20` branch includes upstream `master` through commit
`076632f` and keeps microscope's additional C++20 support and safe Rust
implementation. Upstream supplies non-type and pack parameter declarations,
requires clauses, `_BitInt` substitution fixes, member-like friends,
`co_await`, and clone-suffix filtering. The fork adds:

- Type (`Ty`), constrained (`Tk`), and template (`Tt`) parameter declarations
  in upstream's `TemplateParamDecl` AST, also used for lambda signatures
  (`Ul`). Declarations and requires clauses are parsed but omitted from
  rendered arguments. Constraint names remain substitution candidates.
- Template parameters of an enclosing level (`TL<level>_<index>_`). They print
  as `$T` / `$TN` like llvm-cxxfilt, since the demangler doesn't track which
  level's arguments are in scope.
- Empty argument packs in template and function arguments, which printed as
  stray commas.
- Safe Rust throughout the library, enforced by `#![forbid(unsafe_code)]`.
  Function arguments are formatted from borrowed slices instead of casting
  references to wrapper types, and numeric parsing checks UTF-8. The separate
  `c_api` crate still requires unsafe code for its raw-pointer C interface;
  it is not part of the Rust library used by microscope.

Regression cases from microscope live in `tests/microscope.rs`. Run the full
suite, including the libiberty regression corpus, with `cargo test`.
When working alongside microscope, use its pinned environment:

```sh
# From the microscope checkout:
just dev --command cargo test --manifest-path ../cpp_demangle/Cargo.toml
just dev --command cargo test --manifest-path ../cpp_demangle/Cargo.toml --no-default-features --features alloc
just dev --command ./scripts/contained.sh cargo fuzz run --fuzz-dir ../cpp_demangle/fuzz parse_and_stringify -- -max_total_time=120 -max_len=4096 -rss_limit_mb=2048
```

The fuzz target compares owned and borrowed parsing and rendering, including
rendering options and a low recursion limit. Rendering errors are valid
outcomes even when parsing succeeds.

## Upstream and advisories

The fork follows [gimli-rs/cpp_demangle](https://github.com/gimli-rs/cpp_demangle)
`master` at `076632f924b1c87c7de8ef6a5c604edb97964978`, 11 commits after the
0.5.1 release, and keeps upstream's version, 0.5.1. RustSec matches advisories
by crate name and version, so cargo-deny and cargo-audit in microscope check
this fork as if it were 0.5.1 from crates.io. They know nothing of the code
the fork adds, and judge upstream's code by the version alone, not by which
upstream commits the fork has merged. When merging upstream, update the commit
above and read
[cpp_demangle's advisories](https://rustsec.org/packages/cpp_demangle.html)
for fixes the fork lacks. There were none as of November 2026.

## Updating microscope

microscope pins a commit from this fork as a Git dependency in its `Cargo.toml`.
After pushing a new fork commit, update that `rev` and refresh microscope's
root and fuzz lockfiles. Switch back to crates.io once an upstream release
supports these names.
