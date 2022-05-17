# RUST
Rust is **Machine Oriented High Order language**, or **mohol** best known as a **System Programming Language** that is, they are used more for writing Systems software. such as Drivers, and compilers tools to make code of software more efficient, thereby having more-direct access to the physical hardware of the machine.

some of the similar languages like Rust are C, C++, Swift, and golang. rather than the user-facing software programs like Web application programming, gaming Software that languages like Java, JavaScript, Python, C#, and many more caters to you.

Rust is considered an extremely fast and powerful programming language. one of the reason is it doesnot use [garbage collector](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)) or [reference counting](https://en.wikipedia.org/wiki/Reference_counting) like other memory-safe languages.

In this repo, I have pushed the Fundamentals and syntax of Rust programming language explained with the help of comments alongside.

To begin First set up the Environment to execute Rust programming following the instruction below.
Recommended OS is Linux, [Windows with WSL](https://docs.microsoft.com/en-us/windows/wsl/install) setup or macOS

### [Installing Rust](https://www.rust-lang.org/learn/get-started)
For Linux
```bash 
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
```

- Verifying the Installation
```bash
$ rustc --version
$ cargo --version
```
If you are not getting the expected results to try restarting the terminal.

Open your fav IDE (Recommended VScode) and must Install Rust(rls) extension it's a linter for Rust programming language, helps in auto-completion of the code, and has many other useful features.

### Clone Repo

```bash
https://github.com/kirteeprajapati/RUST.git
```
Inside the [src/main.rs](https://github.com/kirteeprajapati/RUST/blob/main/src/main.rs) file you can comment or uncomment the functions you want to call for execution.

- Quick Start 

```bash 
# Run With Cargo
$ cargo run

# Build
$ cargo build

# Build for production
$ cargo build --release
```


