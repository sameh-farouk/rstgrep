
[![LinkedIn][linkedin-shield]][linkedin-url]


<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#acknowledgements">Acknowledgements</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project
note: this project for-educational-purposes-only and it have a basic code in rust as i started to learn rust just days ago.

this is a mini grep clone. That is a tool that we can give a string and a file path and it’ll print only the lines that contain the given string.

while i code this project  I practiced the fundamentals and learnt about:
* Parsing CLI arguments with `StructOpt`
* using `BufReader` to read files contents.
* handling Potential failure using a shortcut method on `Results`, called `unwrap`
* use `anyhow` library to provide a context and build a custom error message and get a “chain” of error messages pointing out the root cause.

### Built With

* [Rust](https://www.rust-lang.org/)


<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple example steps.

### Prerequisites

* Rust

to install Rust run
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
If you have installed Rustup some time ago, chances are your Rust version is out of date. Get the latest version of Rust by running `rustup update`.

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/sameh-farouk/rstgrep.git
   ```
3. cd into `rstgrep`:
   ```sh
   cd rstgrep
   ```
4. Compile and run it in the same step using `cargo run` command:
   ```sh
   cargo run
   ```



<!-- USAGE EXAMPLES -->
## Usage

### from src code

`cargo run -- <pattern> <path>`

   ```sh
   cargo run -- main ./src/main.rs
   ```

for help you can run
`cargo run -- --help`


### from a release binary

you can downlaod the latest release from [here](https://github.com/sameh-farouk/rstgrep/releases)

or build it yourself from source code

1. first to build the binary run `cargo build` command:
   ```sh
   cargo build --release
   ```
you will find the built binary in `./target/release/`

2. run `./rstgrep <pattern> <path>`
:
   ```sh
   cd target/release
   ./rstgrep main ../../src/main.rs
   ```

for help you can run
`./rstgrep --help`

```
rstGREP 0.2.0
an alternative to grep written in Rust.

USAGE:
    rstgrep [FLAGS] <pattern> <path>

FLAGS:
    -c, --count          print only a count of selected lines per FILE
    -h, --help           Prints help information
    -n, --line-number    Show relative line number in the file
    -r, --recursive      Read all files under each directory, recursively
    -V, --version        Prints version information

ARGS:
    <pattern>    The pattern to look for
    <path>       The path to the file to read
```
<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements
* [The Rust Programming Language: The Book](https://doc.rust-lang.org/book/title-page.html)
* [Command line apps in Rust](https://rust-cli.github.io/book/index.html)
<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/sameh-farouk-software-developer/
