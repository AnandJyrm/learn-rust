# coverage

Explore grcov for coverage

## Prerequisites

LLVM/Clang for compiling the C code.

## Build/Execution

1. Build the FFI C_lib with coverage enabled

```bash
cd C_lib
clang -g -shared -o libcov_c.so -fprofile-instr-generate -fcoverage-mapping  cov_c.c
```

2. Build the RUST code with coverage enabled

The additional flags are present under .cargo/config.toml. Build will generate some .profraw files
that can be deleted

```bash
cargo build
find . -name "default*.profraw" -delete ;
```

Depending on the structure of the header file the cargo build might fail,
with following error:

> Unable to generate bindings: ClangDiagnostic("/usr/include/stdio.h:33:10: fatal error: 'stddef.h' file not found\n")

To avoid this, the stdio.h header must not be included in cov_c.h or `LLVM_CONFIG_PATH` needs to be set to the
llvm-config binary from the llvm installation

3. Executing instrumented code

```bash
LLVM_PROFILE_FILE=%p.profraw ./target/debug/coverage 
```

This should generate a single profraw file with the name `<process_pid>.profraw`

4. Report generation

```
mkdir bindir
cp target/debug/coverage bindir/
cp C_lib/libcov_c.so bindir/
grcov . -b bindir/ -t markdown
grcov . -b bindir/ -t html -o html/
```

This will report the coverage from both the C and rust files
