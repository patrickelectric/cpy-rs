# cpy-rs: Python and C++ Multibinding Export Macro

![cpy-rs Logo](https://github.com/patrickelectric/cpy-rs/blob/main/cpy-rs-logo.png)

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Introduction

cpy-rs is a powerful macro that simplifies the deployment of libraries to both Python and C++. 
This project allows you to create a multibinding export core, making easier for developers to create and distribute libraries that are already made in Rust, but now making them accessible to a broader range of users.

This README provides essential information on how to get started with cpy-rs, its key features, installation guidelines, and usage examples.

## Features

- Easy deployment of libraries to Python and C++ environments.
- Simplified multibinding export core for seamless integration.
- Empowers developers to distribute Python and C++ libraries with minimal effort.
- Supports a wide range of use cases, making it versatile for different projects.

## Getting Started

### Prerequisites

Before using cpy-rs, ensure you have the following prerequisites installed on your system:

- Rust 
- Python
- C++ compiler

### Installation

To use cpy-rs in your project, run `cargo add cpy-binder`, or include it as a dependency in your Rust project's `Cargo.toml` file:

```toml
[dependencies]
cpy-binder = "0.3.0"

### Usage

1. Import the `cpy-binder` macro into your Rust project:

```rust
#[cfg(feature = "python")]
use pyo3::{prelude::*, wrap_pyfunction};

use cpy_binder::export_cpy;
```

2. Initiate your Rust module collections like following.

```rust
// Rust function using cpy macro
export_cpy!(
    mod my_module {
        fn return_true() -> bool {
            let x = true
        }
    }
```

3. Build your Rust project with the `cpy` feature enabled:

```bash
cargo build
```

4. Your Rust functions are now accessible from Python and C++!

For detailed examples and more advanced usage, please refer to the [Examples](https://github.com/patrickelectric/cpy-rs/tree/main/example) section.

## Examples

Here's a quick sneak peek at a Python and C++ code using a Rust function exported with `cpy-rs`:

```python
# Python code
import my_module

result = my_module.return_true()
print(result)
```

```cpp
// C++ code
#include <iostream>
#include "my_module.cpp"

int main() {
    bool result = my_rust_module::return_true();
    std::cout << std::boolalpha << result << std::endl;
    return 0;
}
```

## Contributing

We welcome contributions from the community to improve cpy-rs and add more features. If you'd like to contribute, please follow the guidelines specified in [CONTRIBUTING.md]().

## License

cpy-rs is open-source and distributed under the MIT License. For more information, see [LICENSE](). Feel free to use this project for your own purposes and contribute back to the community!