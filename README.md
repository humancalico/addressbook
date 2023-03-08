This project provides a single-table, in-memory, append-only database used as an address book 

## Installation

You can download the latest release for the addressbook CLI at https://github.com/humancalico/addressbook/releases

A pre-generated data.tsv file has already been provided which contains around 100,000 records that should be downloaded with the addressbook CLI

## Usage

```bash
<path_to_addressbook_binary> <path_to_tsv_file>
```

Upon running the addressbook command with the path of the TSV file the data will be read and stored in memory and you will be dropped in an interactive CLI.
The CLI can run 3 commands:
 - add
 - list
 - find

## Build from source

### Prerequisites

To build and run this project, you need to have the following software installed on your machine:

- Rust programming language (version 1.48 or higher)

You can download and install Rust from the official website: https://www.rust-lang.org/tools/install

### Build Instructions

To build the project in release mode, follow these steps:

1. Open a terminal or command prompt.
2. Clone this repository by running the following command:
```bash
git clone https://github.com/humancalico/addressbook.git
```
3. Build the project in release mode by running the following command:
```bash
cargo build --release
```
This will compile the project in release mode and create an executable file in the target/release directory.
4. Run the executable by running the following command:
```bash
./target/release/addressbook assets/data.tsv
```
