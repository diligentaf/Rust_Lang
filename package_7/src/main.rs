fn main() {
    // - A package contains multiple binary crates and optionally one library crate.
    // - Encapsulating implementation details lets you reuse code at higher level.
    // - When reading, writing, and compiling code, programmers and compilers need to know 
    //    - whether a particular name at a particular spot refers to a 
    //    - variable, function, struct, enum, module, constant, or other item and what that item means
    //    -  You can’t have two items with the same name in the same scope; tools are available to resolve name conflicts.
    // - Rust has features to organize codes : 👇
    //    - Packages: A Cargo feature that lets you build, test, and share crates
    //        - package is one or more crates that provide functionality
    //        - package contains Cargo.toml file that describes how to build the crates (like package.json)
    //        - "cargo new" command creates a package with Cargo.toml
    //            - src/main.rs -> crate root for for binary crate
    //            - src/lib.rs -> crate root for for library crate
    //            - If a package contains both src/lib.rs && src/main.rs -> it has two crates: binanry && library 
    //            - package can have multiple binary crates by placing files in src/bin.
    //                - In src/bin, each file will be a separate binary crate.
    //    - Crates: A tree of modules that produces a library or executable
    //        - Start from the crate root: When compiling crate, it starts from crate root (src/main.rs || src/lib.rs)
    //        - Example : "mod gardent" => It will try to find either src/garden.rs || src/garden/mod.rs
    //        - you can also declare submodules like "mod vegetables" and search src/garden/vegetables.rs || src/garden/vegetables/mod.rs
    //        - Private vs Public: Code within a module is private (default is private). To declare public, run "pub mod"
    //        - use "use" keyword to shorten crate::garden::vegetables::Asparagus to "Asparagus"
    //        - 👇  Directories 👇
    //        - backyard
    //          ├── Cargo.lock
    //          ├── Cargo.toml
    //          └── src
    //              ├── garden
    //              │   └── vegetables.rs
    //              ├── garden.rs
    //              └── main.rs
    //    - Crate can either be binary crate or library crate
    //        - Binary crates are programs you can compile to an executable file (must have mail fn)
    //        - Library crates don't have a main function && don't compile to executable.
    //        - Crate root is a source file that Rust compiler starts from
    //    - Modules and use: Let you control the organization, scope, and privacy of paths
    //    - Paths: A way of naming an item, such as a struct, function, or module 
}
