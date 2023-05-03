update-snapshots:
  TRYCMD=overwrite cargo test

test:
  cargo test

build-release:
  cargo build --release
