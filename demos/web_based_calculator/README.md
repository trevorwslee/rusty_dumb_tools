# DumbCalculator Web-based Demo (work in progress)

Simple web-based Calculator demo for `DumbCalculator` of `rusty_dumb_tools` **_Rust_** crate -- [https://trevorwslee.github.io/DumbCalculator/](https://trevorwslee.github.io/DumbCalculator/)


The Web framework used is [Leptos](https://github.com/leptos-rs/leptos)

Hence, according to [the Leptos book](https://book.leptos.dev/), you will need to run the following to prepare the development environment:

- Install the 'Trunk' tool:
  ```
  cargo install trunk
  ```
- Add the `wasm32-unknown-unknown` target:

  ```
  rustup target add wasm32-unknown-unknown

  ```

  To start the web server for the demo, in the demo sub-project directory, run

  ```
  trunk serve --open
  ```
