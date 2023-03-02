Features are tag to our code to enable their optional compilation. Tagged using #[cfg(features = "lambda")]

Each dependency has its implicit feature which we can use for optional compilation of dependency. Of the form: "dep:gif"
To enable optional dependency:
[dependencies]
gif = [version = 1.1, optional=true]
[features]
gif = [dep:gif]

In Cargo.toml, under [feature], we can list the features. 
By default, all features are disable and only features in [default] are enable by default.

feature_name =[...] //If it is enabled, it also enables the features in its []

Features are defined in the [features] table in Cargo.toml. Each feature specifies an array of other features or optional dependencies that it enables.

Features for the package being built can be enabled on the command-line with flags such as --features. Features for dependencies can be enabled in the dependency declaration in Cargo.toml.

Cargo sets features in the package using the rustc --cfg flag, and code can test for their presence with the cfg attribute or the cfg macro.

Features can list other features to enable.


## Optional dependencies
Dependencies can be marked "optional", which means they will not be compiled by default. 

By default, this optional dependency implicitly defines a feature that looks like this:
[features]
gif = ["dep:gif"]


If you specify the optional dependency with the dep: prefix anywhere in the [features] table, that disables the implicit feature.
[dependencies]
ravif = { version = "0.6.3", optional = true }
rgb = { version = "0.8.25", optional = true }

[features]
avif = ["dep:ravif", "dep:rgb"]


## Dependency Features

Features of dependencies can be enabled within the dependency declaration. The features key indicates which features to enable:
[dependencies]
//Enables the `derive` feature of serde.
serde = { version = "1.0.118", features = ["derive"] }


Features of dependencies can also be enabled in the [features] table. 
The syntax is "package-name/feature-name". For example:
[dependencies]
jpeg-decoder = { version = "0.1.20", default-features = false, optional = true }

[features]
//Enables parallel processing support by enabling the "rayon" feature of jpeg-decoder.
parallel = ["jpeg-decoder/rayon"]

The "package-name/feature-name" syntax will also enable package-name if it is an optional dependency. Often this is not what you want. You can add a ? as in "package-name?/feature-name" which will only enable the given feature if something else enables the optional dependency.

parallel = ["jpeg-decoder?/rayon"]

