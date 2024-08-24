# shuttle-xata-backend

This repository implement shuttle as backend and connect to the xata database for request and response.

## build & deploy

First you need to install **cargo-shuttle**.
You can follow the instructions from [shuttle](https://docs.shuttle.rs/getting-started/installation)

Next, if you want to test local and with hot-reload, you can install **cargo-watch**.
Also, there's tutorials from [shuttle](https://docs.shuttle.rs/getting-started/local-run)

```shell
$ cargo watch -x "shuttle run --external"
```

To run the server locally,

```shell
$ cargo shuttle run --external --allow-dirty
```

For build, simply run normal build script of cargo.

```shell
$ cargo build --release
```

## xata setting

The Secret.toml file is where you set the workspace-id, API-key, password, ...

## License

License and usage are under shuttle and xata.
