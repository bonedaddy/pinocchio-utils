# Rust Template

Template repository for creating new rust projects including a CLI and YAML based configuration management.

1. [Features](#features)
2. [Usage](#usage)
    1. [Linting](#linting)
    2. [Docker](#docker)
        1. [Prerequisites](#prerequisites)
        2. [Building](#building)

## Features

* CLI 
* Configuration management
* Docker image tooling
* Lint script (`cargo fmt` + `clippy`)
* GitHub Actions integration

## Usage

After cloning this repository there are a few things you'll likely want to change:

* The name of the `tag` used when building the docker images (edit the `Makefile`)
* Rust toolchain version (edit `rust-toolchain.toml`)
* If changing the rust toolchain version also update the version in [./github/workflow/ci.yml]

### Linting

The lint script rust runs `cargo fmt`, and then `cargo clippy --fix`. To run the script

```shell
$> ./scripts/lint.sh
```

### Docker

The dockerfile's are optimized to minimize compilation times by using multi-stage builds and [cargo chef](https://github.com/LukeMathWalker/cargo-chef) to cache dependency compilation. The final build stage trims as much as possible to minimize the final image size.

#### Prerequisites

Requires that you have the [BuildKit](https://github.com/docker/buildx) plugin installed.

#### Building

There are two image files

* `Dockerfile` can be used to build the CLI in release mode
* `Dockerfile.dev` can be used to build the CLI in debug mode

To build the release docker image

```shell
$> make build-docker-release
```

To build the debug docker image

```shell
$> make build-docker-debug
```