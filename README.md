# codecov-test

A deliberately small Rust program for experimenting with test coverage and
Codecov pull-request comments.

## Run locally

```sh
cargo run
cargo test
```

To generate the same LCOV report as CI, install
[`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) and run:

```sh
cargo llvm-cov --workspace --lcov --output-path lcov.info
```

## Codecov experiment

The tests cover positive numbers and zero, but intentionally do not cover the
negative-number branch. To see Codecov's automatic pull-request comment:

1. Install and authorize the Codecov GitHub App for this repository.
2. Create a branch and add a test for a negative number.
3. Open a pull request into `main`.

The CI workflow generates `lcov.info` and uploads it with GitHub OIDC, so no
`CODECOV_TOKEN` repository secret is needed. Coverage status checks are
informational, and `codecov.yml` asks Codecov to post a detailed comment even
when coverage does not change.
