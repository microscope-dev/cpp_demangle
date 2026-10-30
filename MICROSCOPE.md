# microscope's cpp_demangle fork

The `microscope-cpp20` branch is based on upstream commit `6fc3a53`
(cpp_demangle 0.5.1), with the parser changes in `src/ast.rs`. Without the
change, 0.5.1 can't parse several C++20 manglings that clang emits and
Chromium is full of, so their names stay mangled in profiles. The parts added here:

- Template parameter declarations (`Ty`, `Tk`, `Tn`, `Tt`, `Tp`) in template
  argument lists and in lambda signatures (`Ul`). They are skipped, as
  llvm-cxxfilt does. A `Tk` constraint's name still counts as a substitution
  candidate, so later `S_` references still resolve.
- Requires clauses (`Q <expression>`) in template arguments and after a
  function's parameter types. They are parsed and dropped.
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

microscope pins a commit from this fork as a Git dependency in its `Cargo.toml`.
After pushing a new fork commit, update that `rev` and refresh microscope's
root and fuzz lockfiles. Switch back to crates.io once an upstream release
supports these names.
