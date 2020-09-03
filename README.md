# FES - Fast Endpoint Scanner
A web application endpoint scanner written in Rust, designed to put less load on the domains it scans (inspired by tomnomnom's meg).
## Installation
### Debian Based
[You can download the latest binary or .deb file from the releases page](https://github.com/JohnWoodman/FES/releases)
### Building From Source
1. Make sure you have cargo and rust installed
2. Git clone the repo
3. cd into the git repo and run `carg build --release`
4. The binary is located at `target/release/fes`
5. You can then symlink to the binary or move it to a directory in your PATH
## Usage
```
fes 1.0
John Woodman <john.woodman11@gmail.com>
Fast Endpoint Scanner Built In Rust

USAGE:
    fes [OPTIONS] --path <paths_file> --urls <urls_file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --concurrency <num>      Set the number of parallel requests (default: 20)
    -o, --output <output_dir>    Specify the directory for output (default: fes_out)
    -p, --path <paths_file>      File with list of endpoints
    -u, --urls <urls_file>       File with list of urls
```
