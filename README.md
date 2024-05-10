# RCLI (Rust CLI)

A command-line interface (CLI) tool implemented in Rust for various utility tasks.

## Installation

1. Clone this repository.
2. Navigate to the project directory.
3. Build the project using `cargo build --release`.
4. The binary file will be available in the `target/release` directory.

## Usage

```bash
Usage: rcli <COMMAND>

Commands:
  csv      Show CSV, or convert CSV to other formats
  genpass  Generate a password
  base64   Base64 encode & decode
  time     Time utils
  text     Text sign & verify
  http     Http server
  jwt      Jwt sign & verify
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Commands:

- `csv`: Convert CSV to other formats.
- `genpass`: Generate a password.
- `base64`: Base64 encode & decode.
- `time`: Time utilities.
- `text`: Text signing & verification.
- `http`: HTTP server.
- `jwt`: JWT signing & verification.

For help on a specific command, use:

```sh
rcli <COMMAND> -h
```

### Options:

- `-h, --help`: Print help.
- `-V, --version`: Print version.

## Examples

### Convert CSV data to Json

```sh
rcli csv --input <INPUT> --format json
```

This will convert the CSV data to JSON format.

### Generate a password

```sh
rcli genpass --length 16 --no-uppercase --no-symbol
```

This will generate a 16-character password without uppercase letters and symbols.

### Encode a string to Base64

```sh
rcli base64 encode "Hello, world!"
```

This will encode the string "Hello, world!" to Base64.

### Time utilities

```sh
rcli time
```

This will display the current Unix timestamp.

### Sign a message

```sh
rcli text sign "Message to sign" <PRIVATE_KEY_FILE>
```

This will sign the message "Message to sign" using the specified private key file.

### Generate a JWT

```sh
rcli jwt sign --sub <SUB> --aud <AUD> --exp [EXP] [KEY]
```

This will generate a JWT using the specified sub, aud, exp and private key file.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
