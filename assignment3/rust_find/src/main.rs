// CMSC388Z Homework #3
// Read Before You Start

//     This assignment is due on October 8th, 2021 at noon.
//     Please submit your src/main.rs, src/lib.rs and tests/integrated_tests.rs onto GradeScope electronically following the instructions.
//     Please make sure you are using the latest version of Rust.

//     $ rustup update
//     $ rustc --version
//     rustc 1.55.0 (c8dfcfe04 2021-09-06)

//     Please make sure your program doesn't contain any warning or error when submitting.
//     Public tests are provided. Make sure your program is passing these before you submit them.
//     Please feel free to refer to any appropriate online resource. If you are not sure, you can email Dongze (dhe17 at umd dot edu) or Chase (ckanipe at terpmail dot umd dot edu) for clarification.
//     This is an individual project, please do not discuss any code-related questions with anyone.
//     This feedback survey will be open throughout the semester, if you have any comments or suggestions for the course, please feel free to report them to us anonymously.
//     We highly recommend using VS code + rust_analyzer extension. But it is OK if you have your preferred code editor.

// Please use Piazza to ask questions!
// Introduction

// In this assignment, you need to implement a command-line utility that is similar to the Linux command find from scratch. That is, your program should be able to recursively search an input directory root_dir and all its subdirectories to find files that match some given regex patterns. You will find this chapter in TRPL very useful.

// Through this assignment, you will practice

//     Basic interaction with the operating system
//     Error handling
//     struct/impl
//     Lifetime
//     Rust Build System

// Note that the error handling strategy we used in this assignment is very basic. The purpose is to only help you understanding the concept. For more advanced error handling strategies, please refer to this chapter in Rust By Example.
// Overview

// Your program will find the files in a given set of directories or their sub-directories whose name matches at least one of the given regex patterns and (optionally) whose size is over a given size threshold. By default, the path of the matched file names will be printed via stdout. Optionally, if users specify an output file path, the path of the matched files will be written to that file. For example,

// $ rust_find --patterns ".*\.rs" --dirs "./src" "./tests"
// ./src/lib.rs
// ./src/main.rs
// ./tests/integrated_tests.rs

// This command will return all files which end with .rs in the directory ./src and ./tests, and all their sub-directories. We will use Rust's regular expression engine, so the syntax for regular expressions may be different from what you're used to in other languages or shells.

// If you would like to run your program when implementing, you can do

// $ cargo run -- --patterns ".*\.rs" --dirs "./src"
// ./src/lib.rs
// ./src/main.rs

// Notice that the -- between cargo run and the program's arguments is to help cargo to recognize the arguments passing to it and the program's arguments. You may specify only one value for each argument when doing cargo run, here I use ./src and *./.rs.
// Implementation details

// There are mainly 5 steps in the assignment:

//     Setting up the environment
//     Building command-line interface
//     Taking and parsing arguments
//     Getting matched files
//     Exporting results

// The provided guideline is designed to be tested easily and smoothly. I encourage you to write a unit testing function for every function you implemented.
// set-up

// In this assignment, we will build a binary + library program. So, you need to first create a lib program. In your terminal, do the following:

// cargo new rust_find --lib && cd rust_find
// touch src/main.rs
// mkdir -p "tests" && touch "tests/integrated_tests.rs"

// Then, you need to specify that your program contains a binary and a library in Cargo.toml. Meanwhile, you can add all the dependencies we will use in this assignment into Cargo.toml as well.

// rust_find/Cargo.toml

// [lib]
// name = "lib"
// path = "src/lib.rs"

// [[bin]]
// name = "rust_find"
// path = "src/main.rs"

// [dependencies]
// regex = "1.5"
// slog = "2.7.0"
// slog-term = "2.8.0"

// [dependencies.clap]
// version = "3.0.0-beta.4"
// features = ["wrap_help"]

// In this assignment, you will work on 3 files, src/main.rs, src/lib.rs, and tests/integrated_tests.rs.

//     In src/main.rs, we parse command-line arguments and evaluate a run() function, which is defined in the src/lib.rs.
//     In src/lib.rs, we define all the structs and functions.
//     In tests/integrated_tests.rs, we write testing functions.

// Taking command-line arguments

// We will use clap-rs to build our command-line interface. You can find many quick examples in their document. Here I will show that how to set it up using Builder Pattern from a string. In the following example, for the --patterns argument,

//     -p is its short version
//     <patterns> is saying the value name is patterns. You will use this name to access it in your code.
//     The short sentence, "List of file patterns to find.", is a description of this argument, which will be displayed if you do rust_find --help after compiling.

// TODO: You need to also define --dirs or -d for short, which specifies a set of directories (multiple_values(true)) that your program will find the matched filenames from, and --size, or -s for short, which specifies the minimum size, in bytes, a matched files need to have to be reported. The --size argument is optional (required(false)).

//src/main.rs

use clap::{App, Arg}; // tell Rust you will use these two structs in clap
use lib::{run, Config}; // tell Rust you will use these two things from our "lib" module

fn main() {
    // Define command-line interface
    let matches = App::new("rust_find")
        .version("0.1.0")
        .author("Your Name <you.email@umd.edu>")
        .about("Find files that match a regex pattern")
        .arg(
            Arg::from("-p , --patterns=<patterns> 'List of file patterns to find.'")
                .takes_value(true)
                .required(true)
                .multiple_values(true), // this argument can takes multiple values
        )
        .arg(
            Arg::from("-o, --output=<output> 'Write results to output file instead of stdout.'")
                .takes_value(true) // argument if true or flag if false.
                .required(false), // this is an optional argument
        )
        .arg(
            Arg::from("-d, --dirs=<dirs>'")
                .takes_value(true) // argument if true or flag if false.
                .multiple_values(true)
                .required(false),
        )
        .arg(
            Arg::from("-s, --size=<size>'")
                .takes_value(true) // argument if true or flag if false.
                .required(false), // this is an optional argument
        )
        .get_matches();
    // .get_matches_from(vec!["rust-find", "--patterns=.*/.rs", "--output=./tests.out"]);

    let args = Config::from_args(&matches); // will be defined later

    if let Err(err) = run(&args) {
        //Error handling here!
        panic!("{}", err)
    }
}
