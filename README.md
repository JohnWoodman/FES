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
    -a, --anomalies      Output sorted anomalous responses based on hashed response body (use with -t, default threshold
                         is 3) (requires -g flag)
    -s, --hash-output    Store only the hash of the response body (takes up a lot less space)
    -h, --help           Prints help information
    -V, --version        Prints version information

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
    -u, --urls <urls_file>                                File with list of urls
```
