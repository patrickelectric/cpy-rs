# cpy-rs 🦀🛠️🐍
[![Test all targets](https://github.com/patrickelectric/cpy-rs/actions/workflows/action.yml/badge.svg)](https://github.com/patrickelectric/cpy-rs/actions/workflows/action.yml)
[![crates.io](https://img.shields.io/crates/v/cpy-binder.svg)](https://crates.io/crates/cpy-binder)
[![docs.rs](https://img.shields.io/docsrs/cpy-binder?label=docs.rs)](https://docs.rs/cpy-binder)



`cpy-rs` is a Rust library that aids in creating bindings from Rust to C++ and Python. It provides a set of macros to easily export Rust structures, enums, and functions to both C/C++ and Python.

## Features

- **cpy_enum**: To export enums.
- **cpy_struct**: To export structures.
- **cpy_fn**: To export functions for both C++ and Python.
- **cpy_fn_c**: To export exclusive C++ functions.
- **cpy_fn_py**: To export exclusive python functions.
- **cpy_module**: To export the python module.

> Note: It's recommended to end the function signature with `_c` and `_py` when using `cpy_fn_c` and `cpy_fn_py`. When using such functions in C++ and Python, the suffix will be removed by the macro. The suffix is also not necessary inside the `cpy_module`. Check the following example as a guide.

## Example Usage

The repository contains [an example project](https://github.com/patrickelectric/cpy-rs/tree/master/example) that demonstrates how to use `cpy-rs` to create bindings.

* [Check here](https://github.com/patrickelectric/cpy-rs/tree/master/example/py_project) to see an example of a python based code.
* [Check here](https://github.com/patrickelectric/cpy-rs/tree/master/example/cpp_project) to see an CMake / C++ based example.

### Code usage

```rust
use cpy_binder::{cpy_enum, cpy_fn, cpy_fn_c, cpy_fn_py, cpy_module, cpy_struct};

#[cpy_enum]
#[comment = "Material types"]
enum Material {
    Plastic,
    Rubber,
}

#[cpy_struct]
#[comment = "2D Size"]
struct Size2D {
    width: f64,
    height: f64,
}

#[cpy_struct]
#[comment = "Tire structure"]
struct Tire {
    material: Material,
    pressure: f64,
    size: Size2D,
}


#[cpy_fn]
#[comment_c = "@brief Creates and returns a random tire.\n
    @return Tire A randomly generated tire.\n"]
#[comment_py = "Creates and returns a random tire.\n
    Returns:\n
        Tire: A randomly generated tire.\n"]
fn create_random_tire() -> Tire {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let random_material = if rng.gen_bool(0.5) {
        Material::Plastic
    } else {
        Material::Rubber
    };

    Tire {
        material: random_material,
        pressure: rng.gen_range(30.0..60.0),
        size: Size2D {
            width: rng.gen_range(5.0..10.0),
            height: rng.gen_range(10.0..20.0),
        },
    }
}

#[cpy_fn_c]
#[comment = "Function for C ABI"]
fn format_wheel_identifier_c(dimensions: &[u8; 3]) {
    println!("Wheel identifier: {:?}", dimensions);
}

#[cpy_fn_py]
#[comment = "Format wheel identifier for Python"]
fn format_wheel_identifier_py(dimensions: [u8; 3]) {
    println!("Wheel identifier: {:?}", dimensions);
}


// Used to export Python module
cpy_module!(
    name = example,
    types = [Material, Size2D, Tire],
    functions = [
        create_random_tire,
        format_wheel_identifier,
    ]
);
```