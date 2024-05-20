


run:
    cargo run --features bevy/dynamic_linking



[unix]
install-tools:
    sudo dnf install lld clang
    sudo dnf install alsa-lib-devel
    sudo dnf install libudev-devel
