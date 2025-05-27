@echo off
echo Running performance profiling...
set CARGO_PROFILE_RELEASE_DEBUG=true
cargo build --release
cargo run --release -- batch 10 1
pause
