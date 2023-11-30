# WC Tool - Rust

Simple implementation of the wc Unix command line tool.

Run the program with
`cargo run -- <path to text file> <optional arg>`

The program accepts the following as optional arguments:

* -c --> bytes in file
* -l --> lines in file
* -w --> words in file
* -m --> characters in file

If no additional arguments are given, the program will print out all of the info associated with the file at the path given.

Example

`2585 19 2585 391 ../../documents/test.txt`
