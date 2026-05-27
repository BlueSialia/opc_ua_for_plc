# OPC UA for PLC

[![Hippocratic License HL3-BOD-CL-ECO-FFD-LAW-MEDIA-MIL-MY-SV-TAL-USTA-XUAR](https://img.shields.io/static/v1?label=Hippocratic%20License&message=HL3-BOD-CL-ECO-FFD-LAW-MEDIA-MIL-MY-SV-TAL-USTA-XUAR&labelColor=5e2751&color=bc8c3d)](https://firstdonoharm.dev/version/3/0/bod-cl-eco-ffd-law-media-mil-my-sv-tal-usta-xuar.html)

---

A Rust repository for an OPC UA gateway for PLCs. It provides a protocol-agnostic internal tag model and protocol drivers. The repository is organized to enforce strong separation between:

- OPC UA server (exposes nodes, subscriptions, read/write from clients and health status)
- Internal core model (unified, thread-safe representation of PLC variables/tags; has zero OPC UA or protocol knowledge)
- Protocol drivers (translate PLC-specific addressing into the core model)
- Runtime glue (ties everything together, loads config, spawns drivers, starts OPC UA server, handles shutdown)

For more details on the project structure and design, see the [ADR](ADR.md) document.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

You only need Rust and Cargo. Both can be installed using `rustup`. Install it from your favourite repository or with:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing

Clone this repository locally and compile it.

```
git clone git@github.com:BlueSialia/opc_ua_for_plc.git
cd opc_ua_for_plc
cargo build
```

TODO: End with an example of getting some data out of the system or using it for a little demo

## Running the tests

You can run the test suite with:

```
cargo test --all
```

On top of it you should also periodically run the linter with:

```
cargo clippy --all-targets --all-features -- -D warnings
```

And verify that the documentation builds without warnings:

```
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

### Break down into end to end tests

TODO: Explain what these tests test and why

```
Give an example
```

### And coding style tests

In order to maintain a similar style over the codebase we use:

```
cargo fmt
```

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

* [Rust](https://www.rust-lang.org/) - Language
* [Cargo](https://doc.rust-lang.org/cargo/) - Dependency Management
* [open62541](https://github.com/HMIProject/open62541) - Bindings for the C99 library `open62541`

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/BlueSialia/opc_ua_for_plc/tags). 

## Authors

* **Jorge Domínguez** - *Initial work* - [BlueSialia](https://github.com/BlueSialia)

See also the list of [contributors](https://github.com/BlueSialia/opc_ua_for_plc/contributors) who participated in this project.

## License

This project is licensed under the Hippocratic License 3.0 - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

None yet.
