set shell := ["nu.exe", "-c"]

default: test run

run:
    cd ./demo/ ; cargo run -- build

test: clippy
    cargo insta test --review

review:
    cargo insta review

clippy:
    cargo clippy

fix:
    cargo clippy fix

build: test
    cargo build --release

fmt:
    cargo fmt --all
