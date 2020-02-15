# Rust Learning - Notes

[Documentation](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html)

### Key notes
Use ```--bin``` to create an application
Use ```--lib``` to create a library (crater)
Add ```--vcs none``` to prevent the creation of a github repository
Cargo.toml is a manifest that contains all of the metadata that Cargo needs to compile your package.

To compile use: ```cargo build```
To complie and run use: ```cargo run``
To create releases use: ```cargo build --release```