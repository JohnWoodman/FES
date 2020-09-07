# FES - Fast Endpoint Scanner
A web application endpoint scanner written in Rust, designed to put less load on the domains it scans with parsing features to help grab the important stuff (inspired by tomnomnom's meg).
```
          _____                    _____                    _____          
         /\    \                  /\    \                  /\    \         
        /::\    \                /::\    \                /::\    \        
       /::::\    \              /::::\    \              /::::\    \       
      /::::::\    \            /::::::\    \            /::::::\    \      
     /:::/\:::\    \          /:::/\:::\    \          /:::/\:::\    \     
    /:::/__\:::\    \        /:::/__\:::\    \        /:::/__\:::\    \    
   /::::\   \:::\    \      /::::\   \:::\    \       \:::\   \:::\    \   
  /::::::\   \:::\    \    /::::::\   \:::\    \    ___\:::\   \:::\    \  
 /:::/\:::\   \:::\    \  /:::/\:::\   \:::\    \  /\   \:::\   \:::\    \ 
/:::/  \:::\   \:::\____\/:::/__\:::\   \:::\____\/::\   \:::\   \:::\____\
\::/    \:::\   \::/    /\:::\   \:::\   \::/    /\:::\   \:::\   \::/    /
 \/____/ \:::\   \/____/  \:::\   \:::\   \/____/  \:::\   \:::\   \/____/ 
          \:::\    \       \:::\   \:::\    \       \:::\   \:::\    \     
           \:::\____\       \:::\   \:::\____\       \:::\   \:::\____\    
            \::/    /        \:::\   \::/    /        \:::\  /:::/    /    
             \/____/          \:::\   \/____/          \:::\/:::/    /     
                               \:::\    \               \::::::/    /      
                                \:::\____\               \::::/    /       
                                 \::/    /                \::/    /        
                                  \/____/                  \/____/   
                                  
```
<img src="https://img.shields.io/badge/Built%20with-Rust-Purple">

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
    fes [FLAGS] [OPTIONS] --paths <paths_file> --urls <urls_file>

FLAGS:
    -a, --anomalies          Output sorted anomalous responses based on hashed response body (use with -t, default
                             threshold is 3) (requires -g flag)
    -r, --follow-redirect    Follow redirects (up to 10)
    -s, --hash-output        Store only the hash of the response body (takes up a lot less space)
    -h, --help               Prints help information
    -V, --version            Prints version information

OPTIONS:
    -f, --status-code=<allowed_statuses>
            Filter and store only the specified status codes (comma separated)

    -g, --parse <dir>
            Specify this flag for parsing an existing fes output directory (this flag is required in order to use the
            following parsing flags: --anomalies (-a), --keyword (-k), --anomaly-threshold (-t)
    -d, --disallowed-status-code=<disallowed_statuses>
            Filter and don't store the specified status codes (comma separated)

    -k, --keyword=<keywords>
            Specify keywords to search for in responses to output (comma separated) (requires -g flag)

    -t, --anomaly-threshold <limit_val>
            Specify the minimum threshold of duplicate responses for anomalies (requires -g flag)

    -c, --concurrency <num>                               Set the number of parallel requests [default: 20]
    -o, --output <output_dir>                             Specify the directory for output [default: fes_out]
    -p, --paths <paths_file>                              File with list of endpoints
    -x, --timeout <timeout>                               Specify the timeout (in seconds) for the requests [default: 3]
    -u, --urls <urls_file>                                File with list of urls
```
## Examples
The below example scans the given urls.txt for the given paths.txt, only saving responses with status code 200 (-f=200), following redirects (-r), sending 100 requests at a time (-c 100), and storing the output in the directory `test_output/` (-o test_output):
```
fes --paths paths.txt --urls urls.txt -f=200 -r -c 100 -o test_output
```
The below example is parsing an already existing FES output directory (-g test_output/), printing unique responses (-a), and printing responses that contain the keyword "password" (-k=password):
```
fes -g test_output/ -a -k=password
``` 
