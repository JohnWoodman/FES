# FES - Fast Endpoint Scanner
A web application endpoint scanner written in Rust, designed to put less load on the domains it scans (inspired by tomnomnom's meg).

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
