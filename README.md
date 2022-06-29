# LAMaR

## Linear Algebra Maths - Rust

A very simple, GLM inspired, linear algebra maths library written in Rust.

It currently does not support everything that GLM does, and likely never will - this was
written as nothing more than a personal project.

## Use

Using the crate is very simple. While it currently is not on crates.io, it can be included within your `Cargo.toml` like so:

``` toml
[dependencies]
lamar = { git = "https://github.com/FFuuZZuu/lamar", branch = "main" }
```

## Documentation

To generate the documentaiton for this app, clone the repository and run
```sh
cargo doc --opem
```
to generate and open the docs in your default browser.

## Contribution

Contributions are welcome! Just create a pull request or issue request and I will respond promptly.
I do request that any contributed code is both documented and is unit tested, so then it adhears
with the rest of the project.
