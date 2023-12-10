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

### Zig

There is a commit called: Add debug example for Zig.

To get IntelliSense in vs code to work I needed to:

- Install extension: Zig Language
- Install zls
- Add zls to path
- Build zls with enable_autofix = false. (Add "enable_autofix" : false to .confgi/zls.json, didnt work for me)
