# Rust Stratum V2

A WIP library implementation of the [Stratum V2 Protocol](https://braiins.com/stratum-v2).

The detailed spec can be found [here](https://docs.google.com/document/d/1FadCWj-57dvhxsnFM_7X806qyvhR0u3i85607bGHxvg/edit#heading=h.we2r5emgsjcx).

## Building

The library can be built and tested using [`cargo`](https://github.com/rust-lang/cargo/):

```
git clone git@github.com:ccdle12/rust-stratum-v2.git
cd rust-stratum-v2
cargo build
```

You can run tests with:

```
cargo test
```

Please refer to the [`cargo` documentation](https://doc.rust-lang.org/stable/cargo/) for more detailed instructions.


## Documentation

Documentation can be built locally using cargo:

```
cargo doc --no-deps --open
```
