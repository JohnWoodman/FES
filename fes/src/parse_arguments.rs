pub mod parse_argument {
    use clap::{App, Arg};

    pub fn get_arguments() -> clap::ArgMatches<'static> {
        let matches = App::new("fes")
            .version("1.1")
            .author("John Woodman <john.woodman11@gmail.com>")
            .about("Fast Endpoint Scanner Built In Rust")
            .arg(
                Arg::with_name("paths_file")
                    .short("p")
                    .long("paths")
                    .takes_value(true)
                    .required_unless("dir")
                    .help("File with list of endpoints"),
            )
            .arg(
                Arg::with_name("urls_file")
                    .short("u")
                    .long("urls")
                    .takes_value(true)
                    .required_unless("dir")
                    .help("File with list of urls"),
            )
            .arg(
                Arg::with_name("num")
                    .short("c")
                    .long("concurrency")
                    .takes_value(true)
                    .help("Set the number of parallel requests [default: 20]"),
            )
            .arg(
                Arg::with_name("output_dir")
                    .short("o")
                    .long("output")
                    .takes_value(true)
                    .help("Specify the directory for output [default: fes_out]"),
            )
            .arg(
                Arg::with_name("hash_write")
                    .short("s")
                    .long("hash-output")
                    .takes_value(false)
                    .help("Store only the hash of the response body (takes up a lot less space)"),
            )
            .arg(
                Arg::with_name("anomaly")
                    .short("a")
                    .long("anomalies")
                    .requires("dir")
                    .takes_value(false)
                    .help(
                        "Output sorted anomalous responses based on hashed response body (use with -t, default threshold is 3) (requires -g flag)",
                    ),
            )
            .arg(
                Arg::with_name("limit_val")
                    .short("t")
                    .long("anomaly-threshold")
                    .requires("dir")
                    .takes_value(true)
                    .help("Specify the minimum threshold of duplicate responses for anomalies (requires -g flag)"),
            )
            .arg(
                Arg::with_name("allowed_statuses")
                    .short("f")
                    .long("status-code")
                    .require_delimiter(true)
                    .require_equals(true)
                    .takes_value(true)
                    .help("Filter and store only the specified status codes (comma separated)"),
            )
            .arg(
                Arg::with_name("disallowed_statuses")
                    .short("d")
                    .long("disallowed-status-code")
                    .require_delimiter(true)
                    .require_equals(true)
                    .takes_value(true)
                    .help("Filter and don't store the specified status codes (comma separated)"),
            )
            .arg(
                Arg::with_name("keywords")
                    .short("k")
                    .long("keyword")
                    .requires("dir")
                    .require_delimiter(true)
                    .require_equals(true)
                    .takes_value(true)
                    .help(
                        "Specify keywords to search for in responses to output (comma separated) (requires -g flag)",
                    ),
            )
            .arg(
                Arg::with_name("dir")
                    .short("g")
                    .long("parse")
                    .takes_value(true)
                    .help("Specify this flag for parsing an existing fes output directory (this flag is required in order to use the following parsing flags: --anomalies (-a), --keyword (-k), --anomaly-threshold (-t)"),
            )
            .arg(
                Arg::with_name("timeout")
                    .short("x")
                    .long("timeout")
                    .takes_value(true)
                    .default_value("3")
                    .help("Specify the timeout (in seconds) for the requests"),
            )
            .arg(
                Arg::with_name("follow_redirects")
                    .short("r")
                    .long("follow-redirect")
                    .takes_value(false)
                    .help("Follow redirects (up to 10)"),
            )
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .takes_value(false)
                    .help("Verbose output"),
            )
            .get_matches();

        matches
    }

    pub fn print_logo() {
        red_ln!(
            "{}",
            r"          _____                    _____                    _____          "
        );
        red_ln!(
            "{}",
            r"         /\    \                  /\    \                  /\    \         "
        );
        red_ln!(
            "{}",
            r"        /::\    \                /::\    \                /::\    \        "
        );
        red_ln!(
            "{}",
            r"       /::::\    \              /::::\    \              /::::\    \       "
        );
        red_ln!(
            "{}",
            r"      /::::::\    \            /::::::\    \            /::::::\    \      "
        );
        red_ln!(
            "{}",
            r"     /:::/\:::\    \          /:::/\:::\    \          /:::/\:::\    \     "
        );
        red_ln!(
            "{}",
            r"    /:::/__\:::\    \        /:::/__\:::\    \        /:::/__\:::\    \    "
        );
        red_ln!(
            "{}",
            r"   /::::\   \:::\    \      /::::\   \:::\    \       \:::\   \:::\    \   "
        );
        red_ln!(
            "{}",
            r"  /::::::\   \:::\    \    /::::::\   \:::\    \    ___\:::\   \:::\    \  "
        );
        red_ln!(
            "{}",
            r" /:::/\:::\   \:::\    \  /:::/\:::\   \:::\    \  /\   \:::\   \:::\    \ "
        );
        red_ln!(
            "{}",
            r"/:::/  \:::\   \:::\____\/:::/__\:::\   \:::\____\/::\   \:::\   \:::\____\"
        );
        red_ln!(
            "{}",
            r"\::/    \:::\   \::/    /\:::\   \:::\   \::/    /\:::\   \:::\   \::/    /"
        );
        red_ln!(
            "{}",
            r" \/____/ \:::\   \/____/  \:::\   \:::\   \/____/  \:::\   \:::\   \/____/ "
        );
        red_ln!(
            "{}",
            r"          \:::\    \       \:::\   \:::\    \       \:::\   \:::\    \     "
        );
        red_ln!(
            "{}",
            r"           \:::\____\       \:::\   \:::\____\       \:::\   \:::\____\    "
        );
        red_ln!(
            "{}",
            r"            \::/    /        \:::\   \::/    /        \:::\  /:::/    /    "
        );
        red_ln!(
            "{}",
            r"             \/____/          \:::\   \/____/          \:::\/:::/    /     "
        );
        red_ln!(
            "{}",
            r"                               \:::\    \               \::::::/    /      "
        );
        red_ln!(
            "{}",
            r"                                \:::\____\               \::::/    /       "
        );
        red_ln!(
            "{}",
            r"                                 \::/    /                \::/    /        "
        );
        red_ln!(
            "{}",
            r"                                  \/____/                  \/____/         "
        );
    }
}
