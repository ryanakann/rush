# rush - (ru)st (sh)ell

rush is a simple Unix-like shell written in Rust. It provides a basic interactive command-line interface for executing commands and managing processes. I created this project to help me understand the basics of systems programming and process management in Rust.

## Features

- Basic command execution
- Built-in `cd` command
- Error handling for command execution
- Customizable and extendable

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.54 or later)

### Installation

1. Clone the repository:

```sh
git clone https://github.com/ryanakann/rush.git
cd rush
```

2. Build the project

```sh
cargo build --release
```

3. Run the shell

```sh
./target/release/rush
```

## Usage

### Example Commands

1. Change directory

```sh
cd path/to/directory
```

2. List directory

```sh
ls path/to/directory
```

3. Exit the shell

```sh
exit
```