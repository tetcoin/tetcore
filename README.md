# Tetcore &middot; [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/CONTRIBUTING.adoc)

[![appveyor](https://img.shields.io/appveyor/build/xdv/tetcore)](https://ci.appveyor.com/project/xdv/tetcore) [![Rust - Debug Build](https://github.com/tetcoin/tetcore/actions/workflows/rust-debug.yml/badge.svg)](https://github.com/tetcoin/tetcore/actions/workflows/rust-debug.yml) [![Rust - Release Build](https://github.com/tetcoin/tetcore/actions/workflows/rust-release.yml/badge.svg)](https://github.com/tetcoin/tetcore/actions/workflows/rust-release.yml)

<p align="center">
  <img src="/docs/media/tetcore.png">
</p>


Tetcore is the framework for blockchain innovation 🚀.

## Trying it out

Simply go to [core.tetcoin.org](https://core.tetcoin.org) and follow the
[installation](https://core.tetcoin.org/docs/en/knowledgebase/getting-started/) instructions. You can
also try out one of the [tutorials](https://tetcoin.org/en/tutorials).

## Contributions & Code of Conduct

Please follow the contributions guidelines as outlined in [`docs/CONTRIBUTING.adoc`](docs/CONTRIBUTING.adoc). In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](docs/CODE_OF_CONDUCT.md).

## Security

The security policy and procedures can be found in [`docs/SECURITY.md`](docs/SECURITY.md).

## License

- Tetcore Primitives (`tp-*`), Fabric (`fabric-*`) and the nobles (`nobles-*`), binaries (`/bin`) and all other utilities are licensed under [Apache 2.0](LICENSE-APACHE2).
- Tetcore Client (`/client/*` / `tc-*`) is licensed under [GPL v3.0 with a classpath linking exception](LICENSE-GPL3).

The reason for the split-licensing is to ensure that for the vast majority of teams using Tetcore to create feature-chains, then all changes can be made entirely in Apache2-licensed code, allowing teams full freedom over what and how they release and giving licensing clarity to commercial teams.

In the interests of the community, we require any deeper improvements made to Tetcore's core logic (e.g. Tetcore's internal consensus, crypto or database code) to be contributed back so everyone can benefit.
