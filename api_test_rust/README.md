# Partful API

## Build

$ cargo build --release

## Running application
V
Move to the following folder
$ cd target/release

Run the application to get the input instructions
$ ./api_test_rust --help

In order for the application to work, the correct api_key and url will need to be inserted, this is currently hard coded, but will be apdated to be read from a .env file in the future.

## Help documentation
$ cargo doc

To open documentation in your detaul browser
$ cargo doc --open

