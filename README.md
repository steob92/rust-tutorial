# rust-tutorial


This repo is the partner code for the TSI Rust Workshop. The workshop notes are [available here](https://www.physics.mcgill.ca/~obriens/Tutorials/rust_intro/).

## Installing Rust

Comprehensive installation instructions for Rust can be accessed [here](https://www.Rust-lang.org/learn/get-started). The installation process involves utilizing `rustup`, a tool used for installing both the Rust compiler (`rustc`) and the package manager (`cargo`). These tools are compatible with Linux, macOS, and Windows (WSL).
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For alternative methods of installing Rust, refer to [this page](https://forge.Rust-lang.org/infra/other-installation-methods.html).

For this tutorial, we'll utilize the [official Rust Docker image](https://hub.docker.com/_/Rust) to compile and run code within a container. To pull the image, execute:


``` bash
docker pull Rust
```
Create an interactive Docker container using the following command:

```
docker run -it --rm -v $(pwd):/local_data -w /local_data Rust bash
```
Explanation:

- `run` executes the bash command in an interactive mode (`-it`) to provide an interactive Bash shell for work.
- `--rm` ensures the container is deleted after use.
- `-v $(pwd):/local_data` mounts the current directory on the local machine to `/local_data` in the container.
- `-w /local_data` sets the working directory to `/local_data` within the container.


(Free) Learning resources:

* ["The Book", the offical ](https://doc.Rust-lang.org/book/)
* ["Rustlings Course on GitHub"](https://github.com/Rust-lang/Rustlings/)
* ["Offical Website"](https://www.Rust-lang.org/)
* ["Rust Playground" a web coding enviroment for Rust](https://play.Rust-lang.org/?version=stable&mode=debug&edition=2021)
* ["Rust Standard Library Crate"](https://doc.Rust-lang.org/std/index.html)
* ["Command line apps in Rust"](https://Rust-cli.github.io/book/index.html)
* ["The Embedded Rust Book"](https://doc.Rust-lang.org/stable/embedded-book/)
