pub mod parse_argument {
    use clap::{App, Arg};

    pub fn get_arguments() -> clap::ArgMatches<'static> {
        let matches = App::new("fes")
            .version("1.0")
            .author("John Woodman <john.woodman11@gmail.com>")
            .about("Fast Endpoint Scanner Built In Rust")
            .arg(
                Arg::with_name("paths_file")
                    .short("p")
                    .long("path")
                    .takes_value(true)
                    .required(true)
                    .help("File with list of endpoints"),
            )
            .arg(
                Arg::with_name("urls_file")
                    .short("u")
                    .long("urls")
                    .takes_value(true)
                    .required(true)
                    .help("File with list of urls"),
            )
            .arg(
                Arg::with_name("num")
                    .short("c")
                    .long("concurrency")
                    .takes_value(true)
                    .help("Set the number of parallel requests (default: 20)"),
            )
            .arg(
                Arg::with_name("output_dir")
                    .short("o")
                    .long("output")
                    .takes_value(true)
                    .help("Specify the directory for output (default: fes_out)"),
            )
            .get_matches();

        matches
    }
}
