# Exercism

Solutions to https://exercism.org

Right now Im doing the C# track.

## Installing CLI

You need to install the CLI and:
- set the configure token that you get fron Exercism
- set workspace to the directory this readme is in.
- submit by using ´exercism submit -path-/filename´

## Commands while working with the exercises:

### Rust

Run all tests: cargo test -- --include-ignored

Add a new exercise: exercism submit src/lib.rs Cargo.toml

### C++

For running C++ tests: cmake -DEXERCISM_RUN_ALL_TESTS=True . && make

After the first run: make

Debugging: change in cpp/.vscode/launch.json "program": "${workspaceFolder}/lasagna/lasagna", (or what ever the exercise is called)

### Zig

There is a commit called: Add debug example for Zig. You also need to build main before debugging it with: zig build-exe main.zig.

To get IntelliSense in vs code to work I needed to:

- Install extension: Zig Language
- Install zls
- Add zls to path
- Build zls with enable_autofix = false. (Add "enable_autofix" : false to .confgi/zls.json, didnt work for me)


### CSharp

 - The intellisense only works of I open up the exercise folder.
 - For some exercies I needed to specify what .csproj file to build or test. with  dotnet test InterestIsInteresting.csproj 
