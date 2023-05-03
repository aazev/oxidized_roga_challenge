[![Build](https://github.com/aazev/oxidized_roga_challenge/actions/workflows/build.yml/badge.svg)](https://github.com/aazev/oxidized_roga_challenge/actions/workflows/build.yml)

# Oxidized RogaLabs Coding Challenge

This project is a rust application that includes a Dockerized MySQL instance.

## Prerequisites

- Rust
- Docker
- Make

## Getting Started

Follow these steps to get the project up and running:

1. Start the Dockerized MySQL instance:

```sh
make start
```

2. Create a new `.env` file in the root of the project. Use the `.development.env` file as a template. Ensure you replace any placeholder values with your actual configuration.

```sh
cp .development.env .env
```

This documentation is still a WIP. More to come soon.

## Docker Commands

If you need to stop the Docker container, you can use the following command:

```sh
make stop
```

To reset the Docker database. Migrations are automatically applied during application initialization:

```sh
make reset
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
