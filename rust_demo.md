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

## First steps
