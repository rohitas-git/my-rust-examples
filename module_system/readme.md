my_project
├── Cargo.toml
└─┬ src
  ├── main.rs
  ├── config.rs
  ├─┬ routes
  │ ├── health_route.rs
  │ └── user_route.rs
  └─┬ models
    └── user_model.rs

    Surprisingly, the compiler only sees the crate module which is our main.rs file. This is because we need to explicitly build the module tree in Rust - there’s no implicit mapping between file system tree to module tree.

# We need to explicitly build the module tree in Rust, there’s no implicit mapping to file system

    To add a file to the module tree, we need to declare that file as a submodule using the mod keyword. The next thing that confuses people is that you would assume we declare a file as module in the same file. But we need to declare this in a different file! 

# The mod keyword declares a submodule

synatx for mod keyword: 
mod my_module;
*Here, the compiler looks for my_module.rs or my_module/mod.rs in the same directory.*

# The pub keyword makes things public (we can also alias it )

# External dependencies are globally available to all modules inside a project
    Dependencies added to Cargo.toml are available globally to all modules inside the project. We don’t need to explicitly import or declare anything to use a dependency.

# Summary
The module system is explicit - there’s no 1:1 mapping with file system
We declare a file as module in its parent, not in itself
The mod keyword is used to declare submodules
We need to explicitly declare functions, structs etc as public so they can be consumed in other modules
The pub keyword makes things public
The use keyword is used to shorten the module path
We don’t need to explicitly declare 3rd party modules

# The super keyword in module path refers to the parent scope

# The use keyword is used to shorten the module path


