# granite2

## Translate Rust source code to a Petri net

Translate the MIR representation of the source code to a Petri net which can then be exported.
The project is intended to be used to find deadlocks in Rust code by translating the source code,
exporting it to the LoLA format and then using the LoLA model checker to verify the property of absence of deadlock.

### Supported export formats

- Petri Net Markup Language (PNML) [https://www.pnml.org/](https://www.pnml.org/)
- LoLA - A Low Level Petri Net Analyzer [https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/](https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/)
- DOT (graph description language) [https://en.wikipedia.org/wiki/DOT\_(graph_description_language)](<https://en.wikipedia.org/wiki/DOT_(graph_description_language)>)

## Getting started

To get a local copy up and running follow these simple example steps.

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

The `settings.json` configures VS Code to instruct the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension to autodiscover the right toolchain.
This proves extremely useful to get feedback on the types, compiler errors, etc. that result from working with the private crates of `rustc`.

As time goes on and the compiler internals change, the code will inevitably need changes to work again.

**The current state of the repository compiled without warnings and with all tests passing with** `rustc 1.71.0-nightly (2f6bc5d25 2023-05-09)`

### Installation

1. Clone the repo

   ```sh
   git clone https://github.com/hlisdero/netcrab.git
   ```

2. Make sure that the sysroot points to a nightly toolchain when running it from the project directory
   _The output should be something like:_ `$HOME/.rustup/toolchains/nightly-<platform>`

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

Write a valid Rust program that compiles correctly, e.g. `rust_program.rs` then run

```sh
granite2 <path_to_program>/rust_program.rs --format=lola --format=pnml --format=dot
```

Three files called `net.lola`, `net.pnml` and `net.dot` should appear in the CWD.

_Note: For more examples, please refer to the integration tests._

### Debugging

The program supports the verbosity flags defined in the crate [clap_verbosity_flag](https://docs.rs/clap-verbosity-flag/latest/clap_verbosity_flag/).
For example, running the program with the flag `-vvv` prints debug messages that can be useful for pinpointing which line of the MIR representation is not being translated correctly.

```sh
granite2 <path_to_program>/rust_program.rs -vvv
```

## Visualizing the results

### Locally

To see the MIR representation of the source code, you can compile the code with the corresponding flag: `rustc --emit=mir <path_to_source_code>`

To graph a net in `.dot` format, install the `dot` tool following the instructions on the [GraphViz website](https://graphviz.org/download/).

Run `dot -Tpng net.dot -o outfile.png` to generate a PNG image from the resulting `.dot` file.

Run `dot -Tsvg net.dot -o outfile.svg` to generate a SVG image from the resulting `.dot` file.

More information and other formats can be found in the [documentation](https://graphviz.org/doc/info/command.html).

### Online

To see the MIR representation of the source code, you may use the [Rust Playground](https://play.rust-lang.org/).
Simply select the option "MIR" instead of "Run" in the dropdown menu.

To graph a given DOT result, you may use the [Graphviz Online tool](https://dreampuf.github.io/GraphvizOnline/) by [dreampuf](https://github.com/dreampuf).

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

## Contact

Project Link: [https://github.com/hlisdero/netcrab](https://github.com/hlisdero/netcrab)

## Acknowledgments

Based on the original work by Tom Meyer found in <https://github.com/Skasselbard/Granite>

The [rustc dev guide](https://rustc-dev-guide.rust-lang.org/) was a trusty guide in the journey of understanding the compiler. It is a must-read for every interested contributor!

The [nightly documentation](https://doc.rust-lang.org/stable/nightly-rustc/) was also a valuable resource to figure out the details of MIR and how to translate every function.
