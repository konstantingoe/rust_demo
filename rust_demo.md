# Rust for statisticians

This file contains a demo of the Rust programming language. The setup assumes that users operate on `macOS`. However, as soon as `cargo` is installed, everything should be platform-independent.

## How to install using `homebrew`

If on macOS, `homebrew` makes installing packages very convenient, and I recommend using it. To install `homebrew` run:

        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

Check for success by typing `brew --version`. Next, run `brew upgrade` to upgrade and `brew doctor` to check whether you're ready to install anything new. Lastly, run:

        brew install rustup

After the command is finished, type

        rustup-init

You should see something similar to this:

![Rust install via brew](resources/01_setup_brew.png)

After restarting your terminal, we're all set.

## How to manually install without `brew`

Installation on Linux or macOS can also be done directly in the terminal via:

        $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

Then, follow the guide on the official [website](https://doc.rust-lang.org/book/ch01-01-installation.html).

## Intro and first steps

Rust has been growing extensively and is for five years running the most "loved" programming language according to a 2020 developer [survey](https://insights.stackoverflow.com/survey/2020?utm_source=thenewstack&utm_medium=website&utm_content=inline-mention&utm_campaign=platform#technology-most-loved-dreaded-and-wanted-languages-loved) on  Stack Overflow:

![Source: Stack Overflow Survey](resources/07_stack_survey.png)

Rust is a **statically** and **strongly** typed systems programming language. That means all types are known at compile time, which makes it close to impossible to write incorrect programs. While `R` and `Python` are interpreted high-level programming languages, `Rust` is a compiled language (similar to C++) and can be both high and low-level.

### Cargo

Cargo is Rust’s build system and package manager (they are called `crates` in Rust).
Check whether `cargo` is available in your terminal by typing `cargo --version`.

Let's dive right in and create a new project using `cargo`. First, create a new folder in your preferred location and then type:

        cargo new hello_world
        cd hello_world

You can use your favorite code editor to open the project. I work with VS Code, which supports working with many programming languages, including Rust. To open the `hello_world` program using VS Code, we simply type:

        code .

You should see something like this:
![cargo](resources/02_setup_cargo.png)

In VS Code, you should see this:
![VS Code hello_world](resources/08_vs_code_helloworld.png)

In VS Code, open a new terminal and `cargo build` the program. This command creates an executable file in `target/debug/hello_world`. We can then execute the built file by simply navigating to it:

        ./target/debug/hello_world

Sometimes, you might want to build and execute at the same time. Then, `cargo run` does these steps for you.

## Statistics in Rust

You might wonder why you should know and use Rust in the first place. In particular, if you're already using Python, R, or Julia. Is this not enough?
It will be in most cases. Whenever you perform operations that are memory-hungry and computationally expensive, you might want to consider implementing this
part of your code in Rust. In R, the community has, for the most part, relied on C++ for such tasks (in Python, there is CPython).

### Linear algebra demo

I will first demonstrate that Rust can be used for more than just printing *Hello World!* to the console.

        cd ..
        cargo new linear_algebra
        cd linear_algebra
        code .

As we can see, we still get the default *Hello World!* print statement in the `main.rs`. Now, let us delete this and replace it with the following:

```rust
use nalgebra::{DMatrix, DVector, Scalar};

```
We used non-default crates in this program, namely `nalgebra` and `ndarray`. To add them to the project, we need to tell this to `cargo`:

        cargo add ndarray nalgebra

Notice the `.toml` file that gets updated automatically.