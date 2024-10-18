A quick test to check which instructions are available in the x86-64-v2 target arch, for purposes of figuring out whether the feature detection that the [crc32fast](https://github.com/srijs/rust-crc32fast/) crate does is happening at runtime or compile-time.

Examine assembly with `cargo asm --target-cpu=x86-64-v2 'test'` and look for a branch (`je` or `jne`).

Results:

1. sse2 is available in x86-64-v1 (default target CPU)
2. sse4.1 is available in x86-64-v2
3. pclmulqdq is not available even in x86-64-v4, even though in practice it seems the pclmulqdq instruction is actually available on all consumer CPUs from 2013 onwards (2010 for Intel).

Points 1 and 2 are expected based on https://en.wikipedia.org/wiki/X86-64#Microarchitecture_levels - but point 3 I wasn't sure of.

See also:

* there are some instructions for checking target features in [this unfinished book on Rust SIMD](https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html)
* https://doc.rust-lang.org/reference/attributes/codegen.html#the-target_feature-attribute to explicitly enable features for parts of the codebase.
