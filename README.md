# OPC UA for PLC

[![Hippocratic License HL3-BOD-CL-ECO-FFD-LAW-MEDIA-MIL-MY-SV-TAL-USTA-XUAR](https://img.shields.io/static/v1?label=Hippocratic%20License&message=HL3-BOD-CL-ECO-FFD-LAW-MEDIA-MIL-MY-SV-TAL-USTA-XUAR&labelColor=5e2751&color=bc8c3d)](https://firstdonoharm.dev/version/3/0/bod-cl-eco-ffd-law-media-mil-my-sv-tal-usta-xuar.html)

---

A Rust repository for an OPC UA gateway for PLCs. It provides a protocol-agnostic internal tag model and protocol drivers. The repository is organized to enforce strong separation between:

- OPC UA server (exposes nodes, subscriptions, read/write from clients and health status)
- Internal core model (unified, thread-safe representation of PLC variables/tags; has zero OPC UA or protocol knowledge)
- Protocol drivers (translate PLC-specific addressing into the core model)
- Runtime glue (ties everything together, loads config, spawns drivers, starts OPC UA server, handles shutdown)

For more details on the project structure and design, see the [ADR](ADR.md) document.

## OPC UA Feature Coverage

<details>
<summary>OPC UA feature checklist</summary>

Test functions are tagged with `#feature <ID>`.

### Protocol & Transport

- [x] `UA-TCP` — UA-TCP Binary Protocol (`opc.tcp`)
- [x] `UA-SECURE-CONV` — UA-SecureConversation (encrypted channels)
- [ ] `UA-HTTPS` — HTTPS/SOAP Web Services transport
- [ ] `UA-REV-CONN` — Reverse Connect
- [ ] `UA-PUBSUB` — PubSub (MQTT / AMQP / UDP)

### Address Space

- [x] `UA-OBJ` — Object Nodes
- [x] `UA-VAR` — Variable Nodes
- [ ] `UA-METHOD` — Method Nodes
- [x] `UA-REF` — Reference Types (Organizes)
- [x] `UA-BROWSE` — Hierarchical Browse structure
- [x] `UA-NS` — Custom Namespaces
- [x] `UA-NODEID` — String NodeIds
- [ ] `UA-NODEID-NUM` — Numeric / Opaque / GUID NodeIds
- [ ] `UA-VIEW` — View Nodes

### Data Access

- [x] `UA-READ` — Read service
- [x] `UA-WRITE` — Write service
- [x] `UA-TYPES` — Built-in Data Types (Bool, Int16, UInt16, Int32, UInt32, Int64, UInt64, Float, Double, String, DateTime, ByteString)
- [ ] `UA-CUSTOM-TYPES` — Custom / Structured Data Types
- [x] `UA-ACCESS` — Access Levels (CurrentRead, CurrentWrite)
- [x] `UA-QUALITY` — Value Quality / StatusCode mapping
- [x] `UA-TS` — Source / Server Timestamps

### Services

- [x] `UA-SESSION` — Session Service
- [ ] `UA-METHOD` — Method Service
- [ ] `UA-QUERY` — Query Service
- [ ] `UA-HISTORY` — History Read / Update Service
- [ ] `UA-DISCOVERY` — Discovery Service (FindServers, GetEndpoints, RegisterServer)
- [ ] `UA-NODEMGMT` — Node Management Service (Add / Delete Nodes at runtime)

### Security

- [x] `UA-SEC-NONE` — None Security Mode
- [x] `UA-SEC-SIGN` — Sign Security Mode
- [x] `UA-SEC-ENCRYPT` — SignAndEncrypt Security Mode
- [x] `UA-SEC-POLICIES` — Security Policies (Basic128Rsa15, Basic256, Basic256Sha256)
- [x] `UA-AUTH-ANON` — Anonymous Authentication
- [x] `UA-AUTH-PASS` — Username / Password Authentication
- [ ] `UA-AUTH-CERT` — X.509 Certificate Authentication
- [ ] `UA-SEC-TRUST` — Trust Store
- [ ] `UA-SEC-REJECT` — Rejected Certificate Store

### Subscriptions & Events

- [x] `UA-SUBS` — Subscriptions
- [x] `UA-MONITOR` — Monitored Items
- [x] `UA-PUBLISH` — Publish / Republish
- [ ] `UA-EVENT-FILTER` — Event Filters
- [ ] `UA-ALARMS` — Alarms & Conditions (A&C)
- [ ] `UA-AUDIT` — Audit Events

### Advanced Features

- [ ] `UA-HDA` — Historical Data Access (HDA)
- [ ] `UA-AGGREGATES` — Aggregates (Min, Max, Avg, etc.)
- [ ] `UA-PROGRAMS` — Methods / Programs
- [ ] `UA-FILE` — File Transfer
- [ ] `UA-COMPANION` — Companion Specifications
- [ ] `UA-SEMANTIC` — Semantic Annotations
- [ ] `UA-REDUNDANCY` — Redundancy (Server / Client failover)
- [ ] `UA-DIAG` — Server Diagnostics & Status Variables

### Protocol Drivers

Already implemented:

- [x] `DRV-MODBUS` — Modbus TCP
- [x] `DRV-FINS` — Omron FINS
- [ ] `DRV-S7` — Siemens S7 (S7comm)
- [ ] `DRV-EIP` — EtherNet/IP (CIP)
- [ ] `DRV-PROFINET` — PROFINET
- [ ] `DRV-BACNET` — BACnet
- [ ] `DRV-DNP3` — DNP3
- [ ] `DRV-IEC104` — IEC 60870-5-104
- [ ] `DRV-MELSEC` — MELSEC (Mitsubishi)
- [ ] `DRV-SPARKPLUG` — MQTT Sparkplug B
- [ ] `DRV-DF1` — DF1 / PCCC (Allen-Bradley)
- [ ] `DRV-HOSTLINK` — HostLink (Omron)

</details>

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
