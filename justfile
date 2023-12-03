#!/usr/bin/env just --justfile

lint day:
  cargo clippy -p {{day}}

run day part:
  cargo run -p {{day}} --bin {{part}}

test day part:
    cargo test -p {{day}}

bench day:
    cargo bench --bench bench -p {{day}} -q

new day:
    cargo generate --path ./template --name {{day}} --vcs none