# Rust Learning - Managing Growing Projects with Packages, Crates, and Modules

[Documentation](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

### Notes
Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, and include:

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module

### Packages and Crates
A crate is a binary or library. 
A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

### Defining Modules to Control Scope and Privacy
Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

### Paths for Referring to an Item in the Module Tree
A path can take two forms:

An absolute path starts from a crate root by using a crate name or a literal crate.
A relative path starts from the current module and uses self, super, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

Choosing whether to use a relative or absolute path is a decision you’ll make based on your project. The decision should depend on whether you’re more likely to move item definition code separately from or together with the code that uses the item.

> If you want to make an item like a function or struct private, you put it in a module.

### Making Structs and Enums Public
If we use p[ub before a struct definition](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public), we make the struct public, but the struct’s fields will still be private.

Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. 

### Bringing Paths into Scope with the use Keyword

[Documentation](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#bringing-paths-into-scope-with-the-use-keyword)

### The Glob Operator
If we want to bring all public items defined in a path into scope, we can specify that path followed by *, [the glob operator](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#the-glob-operator):
```use std::collections::*;```

Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.

### Separation Modules into Different Files

[Docs](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html#separating-modules-into-different-files)