# cargo-check-deadlock

## Detect deadlocks at compile time in Rust source code

The tool supports detecting deadlocks caused by incorrect use of [mutexes](https://doc.rust-lang.org/std/sync/struct.Mutex.html) (`std::sync::Mutex`) and [condition variables](https://doc.rust-lang.org/std/sync/struct.Condvar.html) (`std::sync::Condvar`).
It also supports detecting deadlocks caused by calling `join` on a thread that never returns.

It does this by translating the [Mid-level Intermediate Representation (MIR) representation](https://rustc-dev-guide.rust-lang.org/mir/index.html) of the Rust source code to a [Petri net](https://en.wikipedia.org/wiki/Petri_net), a mathematical and graphical model.
The Petri net is then analyzed by the model checker [LoLA](https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/) to find out if the net can reach a deadlock.
This approach is an exhaustive check of all possible program states. It is *not* just testing a couple of possible executions, it is also *not* [fuzz testing](https://en.wikipedia.org/wiki/Fuzzing).

For more details about what works and what does not, see [Limitations](#limitations). For more context about this project, see [Context](#context).

### Supported export formats

- Petri Net Markup Language (PNML) [https://www.pnml.org/](https://www.pnml.org/): A standard XML-based format used in many other tools that work with Petri nets.
- LoLA - A Low-Level Petri Net Analyzer [https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/](https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/): This format is needed for the model checker used in this project.
- DOT (graph description language) [https://en.wikipedia.org/wiki/DOT\_(graph_description_language)](<https://en.wikipedia.org/wiki/DOT_(graph_description_language)>): A straightforward visualization of the resulting Petri net. See the corresponding [section](#visualizing-the-results).

## Installation from `crates.io`

Assuming you already have Rust installed on your system, simply run:

```sh
cargo install cargo-check-deadlock
```

You must then install the model checker as explained in the section [Model checker](#model-checker).

## Setting up the environment for development

To get a local copy for development up and running follow these simple example steps.

### Prerequisites

- Install Rust using `rustup` as described on the [Rust Website](https://www.rust-lang.org/tools/install)
- Install the latest nightly version with the `rustc-dev` component running:

   ```sh
   rustup toolchain install nightly
   ```

- Activate the [nightly toolchain](https://rust-lang.github.io/rustup/concepts/channels.html):

   ```sh
   rustup default nightly
   ```

- Install the components needed to work with the compiler internals

   ```sh
   rustup component add rust-src rustc-dev llvm-tools-preview
   ```

### Compiler version

The project must be compiled with the nightly toolchain to access the private crates of the compiler.
The toolchain file `rust-toolchain` in the root folder overrides the currently active toolchain for this project.
See the `rustup` documentation for more information: <https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file>

The `settings.json` configures VS Code to instruct the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension to auto-discover the right toolchain.
This proves extremely useful to get feedback on the types, compiler errors, etc. that appear when working with the private crates of `rustc`.

As time goes on and the compiler internals change, the code will inevitably need changes to work again.

**The current state of the repository compiles without warnings and with all tests passing with**
`rustc 1.72.0-nightly (065a1f5df 2023-06-21)`

### Installation

1. Clone the repo

   ```sh
   git clone https://github.com/hlisdero/cargo-check-deadlock.git
   ```

2. Make sure that the sysroot points to a nightly toolchain when running it from the project directory
   *The output should be something like* `$HOME/.rustup/toolchains/nightly-<platform>`

   ```sh
   rustc --print=sysroot
   ```

3. Build the project with `cargo`

   ```sh
   cargo build
   ```

4. Run the tests to check that everything works with `cargo`

   ```sh
   cargo test
   ```

## Usage

Write a valid Rust program that compiles correctly, e.g. `rust_program.rs`, then run

```sh
cargo check-deadlock <path_to_program>/rust_program.rs
```

The result is printed on stdout. A file named `net.lola` should appear in the CWD.

If you would like to export to other formats or use a custom filename or output folder, use

```sh
cargo check-deadlock <path_to_program>/rust_program.rs --dot --pnml --filename=example --output-folder=output/
```

In this case, files named `example.pnml` and `example.dot` should appear in the `output/` folder.

To obtain the full list of CLI options, use the `--help` flag.

*Note: For more examples, please refer to the integration tests.*

### Debugging

The program supports the verbosity flags defined in the crate [clap_verbosity_flag](https://docs.rs/clap-verbosity-flag/latest/clap_verbosity_flag/).
For example, running the program with the flag `-vvv` prints debug messages that can be useful for pinpointing which line of the MIR representation is not being translated correctly.

```sh
cargo check-deadlock <path_to_program>/rust_program.rs -vvv
```

LoLA model checker supports printing a "witness path" that shows a sequence of transition firings leading to a deadlock.
This is very useful when extending the translator and the Petri net does not match the expected result for a given program.
A convenient [script](./scripts/run_lola_and_print_witness_path.sh) can be found to print the witness path for a `.lola` file.

## Visualizing the results

### Locally

To see the MIR representation of the source code, you can compile the code with the corresponding flag: `rustc --emit=mir <path_to_source_code>`

To graph a net in `.dot` format, install the `dot` tool following the instructions on the [GraphViz website](https://graphviz.org/download/).

Run `dot -Tpng net.dot -o outfile.png` to generate a PNG image from the resulting `.dot` file.

Run `dot -Tsvg net.dot -o outfile.svg` to generate a SVG image from the resulting `.dot` file.

More information and other formats can be found in the [documentation](https://graphviz.org/doc/info/command.html).

### Online

To see the MIR representation of the source code, you may use the [Rust Playground](https://play.rust-lang.org/).
Simply select the option "MIR" instead of "Run" in the dropdown menu. Remember to select the nightly version too.

To graph a given DOT result, you may use the [Graphviz Online tool](https://dreampuf.github.io/GraphvizOnline/) by [dreampuf](https://github.com/dreampuf).

Alternatively, you may use [Edotor](https://edotor.net/) or [Sketchviz](https://sketchviz.com/new).

## Model checker

The model checker LoLA can be downloaded [from the official website](https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/). It must be compiled from the source code.

An alternative mirror with detailed instructions is available on GitHub: <https://github.com/hlisdero/lola>

A last option is to copy the precompiled 64-bit executable `./assets/lola` to the `$PATH`. A [script](./scripts/copy_lola_executable_to_cargo_home.sh) for this purpose can be found in the repo.

Support for other model checkers and export formats may be added in the future.
Adding other backends could be a great way to compare their performance and accuracy.
The export formats are implemented in the custom Petri net library used in this project: <https://github.com/hlisdero/netcrab>

## Limitations

As Rust is a very complex language, supporting all the cases in which a deadlock may arise is impossible to do in practice.
The goal of this project is to demonstrate that an approach using Petri nets is feasible and could detect errors at compile time, therefore enhancing the safety and reliability of Rust code.
The most difficult case to detect at the moment is lost signals. This particular deadlock case arises when a thread calls `notify_one` on a condition variable before another thread called `wait`.

It is recommended to check out the [example programs](./examples/programs/) to see which kinds of programs can be translated and analyzed successfully.
Particularly interesting examples are the [dining philosophers problem](./examples/programs/thread/dining_philosophers.rs) and the [producer-consumer problem](./examples/programs/condvar/producer_consumer.rs).

Currently, the programs that the translator can deal with are fairly limited:

- Closures outside of `thread:spawn` are not supported.
- Creating multiple threads in a loop is not supported.
- Using arrays, `Vec`, and other data structures may cause the translation to give false results.
- [Channels](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) are not supported.
- [RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html) is not supported.
- [Barrier](https://doc.rust-lang.org/std/sync/struct.Barrier.html) is not supported.
- Async is not supported.
- Synchronization mechanisms from external libraries such as [tokio](https://crates.io/crates/tokio) or [semaphore](https://crates.io/crates/semaphore) are not supported.

## Contributing

Contributions are what makes the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0). See [LICENSE-MIT](./LICENSE-MIT), [LICENSE-APACHE](./LICENSE-APACHE) for more information.

## Acknowledgments

This project extends and reimplements ideas from the original work by Tom Meyer found in <https://github.com/Skasselbard/Granite> as part of the master thesis titled ["A Petri-Net Semantics for Rust"](https://github.com/Skasselbard/Granite/blob/master/doc/MasterThesis/main.pdf).

The [rustc dev guide](https://rustc-dev-guide.rust-lang.org/) was a trusty guide in the journey of understanding the compiler. It is a must-read for every interested contributor!

The [nightly documentation](https://doc.rust-lang.org/stable/nightly-rustc/) was also a valuable resource to figure out the details of MIR and how to translate every function.

## Context

This project is part of Horacio Lisdero Scaffino's undergraduate thesis titled "Compile-time Deadlock Detection in Rust using Petri Nets" at the [School of Engineering of the University of Buenos Aires](https://fi.uba.ar/).

The thesis is publicly available on GitHub: <https://github.com/hlisdero/thesis>

It provides important background topics and implementation details about this project.
