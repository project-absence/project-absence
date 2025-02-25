<div align="center">

# Project Absence

[![Docs.rs Badge](https://img.shields.io/badge/docs.rs-project--absence-61c192.svg)](https://docs.rs/project-absence)
[![Crates.io Badge](https://img.shields.io/crates/v/project-absence.svg?color=fe7d37)](https://crates.io/crates/project-absence)
[![Docker Badge](https://img.shields.io/docker/v/kkrypt0nn/project-absence?logo=docker)](https://hub.docker.com/r/kkrypt0nn/project-absence)
[![CI Badge](https://github.com/project-absence/project-absence/actions/workflows/ci.yml/badge.svg)](https://github.com/project-absence/project-absence/actions)
[![Dependency Status Badge](https://deps.rs/repo/github/project-absence/project-absence/status.svg)](https://deps.rs/repo/github/project-absence/project-absence)

[![Discord Server Badge](https://img.shields.io/discord/739934735387721768?logo=discord)](https://discord.gg/mTBrXyWxAF)
[![Last Commit Badge](https://img.shields.io/github/last-commit/project-absence/project-absence)](https://github.com/project-absence/project-absence/commits/main)
[![Conventional Commits Badge](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org/en/v1.0.0/)

</div>

---

> **Warning** This is a WIP tool that is **very unstable** and **not fully optimised**, use at your own care! This README will also be reworked.

### 👁️ Uncover the unseen

Project Absence is a tool for analyzing your domains. Its features include finding open ports, discovering subdomains, discovering files and more will be coming with the time.

## Getting Started

### Rust Features

Project Absence has the following [Rust features](https://doc.rust-lang.org/cargo/reference/features.html) available:

* `clipboard`: Will let you use the `--clipboard/-C` command line argument, only necessary if you want to copy the result to your clipboard (the result is saved in a file at `~/.absence/result.json` either way)

### Installation

To install Project Absence, you can use one of the following methods:

#### Cargo

You need to have [Rust](https://rustup.rs) installed. You can then install using:

```bash
cargo install project-absence
```

#### Docker

You can run the tool from the published [Docker image](https://hub.docker.com/r/kkrypt0nn/project-absence) using:

```bash
docker run -v ~/.absence:/root/.absence -it kkrypt0nn/project-absence
```

#### Docker compose

You can run the tool from a `docker-compose.yml` file, for example:

```yml
services:
  project-absence:
    image: kkrypt0nn/project-absence:latest
    volumes:
      - ${HOME}/.absence:/root/.absence
```

And then run it with

```bash
docker-compose run project-absence
```

#### Build from source

You need to have [Rust](https://rustup.rs) installed. After cloning this repository you can build it using:

```bash
cargo build --release
```

> [!NOTE]
> On **Linux** systems, you have to install the following packages if you want to use the `--clipboard/-C` CLI argument:
>
> - `libxcb1-dev`
> - `libxcb-render0-dev`
> - `libxcb-shape0-dev`
> - `libxcb-xfixes0-dev`
>
> They are required for the [`clipboard`](https://crates.io/crates/clipboard) crate to work properly. The usage of the crate may be put behind a feature in the future so that you are not forced to install these packages.

### Example Usage

Using the tool is straightforward. You may look at the [documentation](https://projectabsence.org/docs/) website for the config and CLI arguments that you can pass.

After editing the config as you wish, running the tool with no specific CLI arguments is as simple as doing

```bash
project-absence -d krypton.ninja
```

## Documentation

Full documentation is available [here](https://projectabsence.org/docs/). It includes detailed explanations of arguments and configurations.

## Troubleshooting

If you encounter issues while using Project Absence, consider the following:

- Ensure you are running the latest version
- Report issues: Use the [GitHub issue tracker](https://github.com/project-absence/project-absence/issues)

## Disclaimer

This tool is designed for **legal and ethical host analysis only**. Misusing it for unauthorized access or other illegal activities is strictly prohibited. The authors and contributors are not responsible for any misuse or legal consequences arising from its use.

## Contributing

People may contribute by following the [Contributing Guidelines](./CONTRIBUTING.md) and
the [Code of Conduct](./CODE_OF_CONDUCT.md)

## License

This project was made with 💜 by [Krypton](https://github.com/kkrypt0nn) and is under the [MIT License](./LICENSE.md).
