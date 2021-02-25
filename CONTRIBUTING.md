# Contributing

Thanks for your interest in contributing to AmFn Engine. We look forward to
your suggestions, bug reports, and pull requests.

*Note:* Anyone who interacts with AmFn Engine in any space, including but not
limited to this GitHub repository, must follow our [code of
conduct](CODE_OF_CONDUCT.md).


## Submitting bug reports

Have a look at our [issue tracker]. If you can't find an issue (open or closed)
describing your problem (or a very similar one) there, please open a new issue with
the following details:

- Which versions of Rust and AmFn Engine are you using?
- Which feature flags are you using?
- What are you trying to accomplish?
- What is the full error you are seeing?
- How can we reproduce this?

[issue tracker]: ../../issues


## Contribute code to AmFn Engine

### Setting up AmFn Engine locally

1. Install Rust using [rustup], which allows you to easily switch between Rust
   versions. AmFn Engine supports Rust Stable.

2. Clone this repository, open a command-line tool, change directory to the
   root of the AmFn Engine cloned repository, and enter the command:
   ```
   cargo build
   ```

3. The integration tests for AmFn Engine are packaged with the AmFn Batch Process,
   which drives the tests. Please install the AmFn Batch Process in order to
   run the AmFn Engine integration tests.

[rustup]: https://rustup.rs/

### Coding Style

We follow the [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md), 
enforced using [rustfmt](https://github.com/rust-lang/rustfmt). To run rustfmt locally:

1. Use rustup to set rust toolchain to the version specified in the
   [rust-toolchain file](./rust-toolchain).

2. Install the rustfmt and clippy by running
   ```
   rustup component add rustfmt-preview
   rustup component add clippy-preview
   ```

3. Run clippy using cargo from the root of the AmFn Engine.
   ```
   cargo clippy
   ```
   Each pull request needs to compile without any warnings.

4. Run rustfmt using cargo from the root of the AmFn Engine.

   To see changes that need to be made, run:

   ```
   cargo fmt --all -- --check
   ```

   If all code is properly formatted (e.g. if you have not made any changes),
   this should run without error or output. If your code needs to be reformatted,
   you will see a diff between your code and properly formatted code. If you 
   see code here that you didn't make any changes to then you are probably 
   running the wrong version of rustfmt. Once you are ready to apply the 
   formatting changes, run:

   ```
   cargo fmt --all
   ```

   You won't see any output, but all your files will be corrected.

You can also use rustfmt to make corrections or highlight issues in your editor.
Check out [their README](https://github.com/rust-lang/rustfmt) for details.
