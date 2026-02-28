# tropxeskoob

Export your Skoob bookshelf to JSON and CSV formats easily.

Skoob is a Brazilian social network for readers. This tool allows you to backup your bookshelf data for analysis, migration, or personal records.

[Português (Brasil)](README.pt-BR.md)

## Features

- Login via email/password or JWT token.
- Export all books from your shelf (Read, Want to Read, Reading, etc.).
- Output formats: JSON (complete data) and CSV (flattened for spreadsheets).
- Automatic pagination handling.
- Respectful API rate limiting.

## Installation

### Using Cargo

```bash
cargo install tropxeskoob
```

### Using Nix

```bash
nix profile install github:pedrobrantes/tropxeskoob
```

## Usage

### Simple Login

```bash
tropxeskoob --email your@email.com
```

### Using a Token (Recommended for Social Login)

If you log in to Skoob using Google or Facebook, you should use a JWT token.

```bash
tropxeskoob --token YOUR_JWT_TOKEN --user-id YOUR_USER_ID
```

See [GET_TOKEN.en-US.md](GET_TOKEN.en-US.md) for instructions on how to obtain these values.

### Options

```text
Usage: tropxeskoob [OPTIONS]

Options:
  -e, --email <EMAIL>        Your Skoob email
  -p, --password <PASSWORD>  Your Skoob password
  -t, --token <TOKEN>        Manually provide a JWT token
  -u, --user-id <USER_ID>    Manually provide a user ID
  -o, --output <OUTPUT>      Output file path/basename [default: full_bookshelf]
      --json                 Export to JSON format
      --csv                  Export to CSV format
  -h, --help                 Print help
  -V, --version              Print version
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
