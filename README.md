# Envar

## Overview

**Envar** is a simple Rust command-line tool designed to read and display the value of a specified environment variable. This tool provides a quick and easy way to check the value of environment variables on your system, making it especially useful for developers and system administrators who need to debug environment configurations.

## Features

- **Read Environment Variables**: Input the name of any environment variable, and the tool will display its value.
- **User-Friendly Messages**: If the specified environment variable does not exist or is not set, the tool provides a clear and concise message indicating that the variable was not found.
- **Cross-Platform Compatibility**: The tool works seamlessly on Windows, Linux, and macOS, making it a versatile utility for different environments.
- **Simple and Lightweight**: Built with simplicity in mind, this tool has minimal dependencies and a small footprint.

## Build and Installation

To build the `envar` tool, you need to have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once Rust is installed, you can clone this repository and build the project:
```sh
git clone https://github.com/yourusername/print-env-vars.git
cd print-env-vars
cargo build --release
```
This will generate the executable in the target/release directory.

## Usage
### Basic Usage
To run the print-env-vars tool, simply execute the compiled binary and enter the name of the environment variable you wish to read:

```shell
./envar <ENV_VAR>
```
For example, the command:
```shell
./envar PATH
```
will output the value of the "PATH" environment variable:
```shell
The value of the environment variable 'PATH' is: /usr/local/bin:/usr/bin:/bin
```
If an environment variable does not exist or is not set, the tool will display a message indicating that the variable was not found:
```shell
The environment variable 'NON_EXISTENT_VAR' was not found.
```

### Help Option
You can also display usage instructions and examples by using the --help or -h option:
```shell
./envar --help
```
This will display the following help message:
```shell
Usage: envar <ENV_VAR>
Retrieve the value of the specified environment variable.

Options:
  -h, --help     Show this help message

Examples:
  envar PATH     Retrieve the value of the 'PATH' environment variable
```

## Contributing
Contributions are welcome! If you have suggestions for improving the tool or adding new features, feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License.
