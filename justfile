


run:
    cargo run --features bevy/dynamic_linking

# Perform all verifications (compile, test, lint, etc.)
verify: test lint

# Run the tests
test:
	cargo hack test --feature-powerset --locked
	cargo deny check licenses

# Run the static code analysis
lint:
	cargo fmt --all -- --check
	cargo hack clippy --feature-powerset --all-targets --workspace --locked
	cargo deny check

fmt:
    cargo fmt

ci-tools:
    sudo apt-get install libasound2-dev
    sudo apt-get install libudev-dev

[unix]
install-tools:
    sudo dnf install lld clang
    sudo dnf install alsa-lib-devel
    sudo dnf install libudev-devel
