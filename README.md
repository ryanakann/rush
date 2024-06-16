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

## Feature Plans

### Built-in Commands

1. `cd` (change directory): Changes the current working directory.

```sh
cd /path/to/directory
```

2. `ls` (list): Lists directory contents.

```sh
ls /path/to/directory
```

3. `pwd` (print working directory): Displays the current working directory.

```sh
pwd
```

4. `echo`: Prints text to the terminal.

```sh
echo "Hello, world!"
```

5. `export`: Sets or exports environment variables.

```sh
export VARIABLE=VALUE
```

6. `unset`: Removes environment variables or shell variables.

```sh
unset VARIABLE
```

7. `alias`: Creates an alias for a command.

```sh
alias ll='ls -la'
```

8. `unalias`: Removes an alias.

```sh
unalias ll
```

9. `history`: Displays the command history.

```sh
history
```

12. `source`: Executes commands from a file in the current shell.

```sh
source ~/.rushrc
```

13. `exit`: Exits the shell.

```sh
exit
```

### Pipelines and Redirections

1. Piping with `|`
2. Redirect
  1. Write out with `>`
  1. Append out with `>`
  1. Read in with `<`

### Environment Variables

1. Define with `=`
1. Reference with `$`

### Job Control

Allow background processing with `&`

### Configuration Files

Read `~/.rushrc` on shell start.
