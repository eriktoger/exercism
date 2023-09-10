# Exercism

Solutions to https://exercism.org

Right now Im doing the rust track.

## Commands while working with the exercises:

### Rust

Run all tests: cargo test -- --include-ignored

Add a new exercise: exercism submit src/lib.rs Cargo.toml

### C++

For running C++ tests: cmake -DEXERCISM_RUN_ALL_TESTS=True . && make

After the first run: make

Debugging: change in cpp/.vscode/launch.json "program": "${workspaceFolder}/lasagna/lasagna", (or what ever the exercise is called)
