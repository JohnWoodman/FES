use std::str;

#[macro_use]
extern crate colour;

mod fes_requests;
mod parse_arguments;
mod read_files;
mod sort_hashes;
mod write_files;
pub use crate::fes_requests::fes_request;
pub use crate::parse_arguments::parse_argument;
pub use crate::read_files::read_file;
pub use crate::sort_hashes::sort_hash;
use std::fs;
use std::path::Path;

fn main() {
    parse_argument::print_logo();
    let matches = parse_argument::get_arguments();
    let hash_write: bool = matches.is_present("hash_write");
    let output_dir = matches.value_of("output_dir").unwrap_or("fes_out");
    let local_dir = matches.value_of("dir").unwrap_or("fes_out");
    let anomaly: bool = matches.is_present("anomaly");
    let a_thresh = matches
        .value_of("limit_val")
        .unwrap_or("3")
        .parse::<i32>()
        .expect("Error: Integer not specified for anomaly threshold");
    let parallel_requests: usize = matches.value_of("num").unwrap_or("20").parse().unwrap();
    let mut keywords = vec![];
    if matches.is_present("keywords") {
        keywords = matches.values_of("keywords").unwrap().collect::<Vec<_>>();
    }

    if matches.is_present("paths_file") && matches.is_present("urls_file") {
        if Path::new(output_dir).exists() {
            fs::remove_dir_all(output_dir).unwrap();
        }
        let follow_redirects: bool = matches.is_present("follow_redirects");
        let timeout = matches
            .value_of("timeout")
            .unwrap()
            .parse::<u64>()
            .expect("Error: Integer not specified for timeout");
        let paths_file = matches.value_of("paths_file").unwrap();
        let urls_file = matches.value_of("urls_file").unwrap();
        let mut allowed_status = vec![];
        let mut disallowed_status = vec![];
        if matches.is_present("allowed_statuses") {
            allowed_status = matches
                .values_of("allowed_statuses")
                .unwrap()
                .collect::<Vec<_>>();
        };
        if matches.is_present("disallowed_statuses") {
            disallowed_status = matches
                .values_of("disallowed_statuses")
                .unwrap()
                .collect::<Vec<_>>();
        };
        let url_string: Vec<String> = read_file::read_lines(urls_file).unwrap();
        let urls: Vec<&str> = url_string.iter().map(AsRef::as_ref).collect();
        let paths_string: Vec<String> = read_file::read_lines(paths_file).unwrap();
        let paths: Vec<&str> = paths_string.iter().map(AsRef::as_ref).collect();
        fes_request::get_request(
            urls,
            paths,
            parallel_requests,
            output_dir,
            hash_write,
            allowed_status,
            disallowed_status,
            timeout,
            follow_redirects,
        );
        if matches.is_present("dir") {
            sort_hash::read_hashes(output_dir, a_thresh, keywords, anomaly);
        }
    } else if matches.is_present("dir") {
        sort_hash::read_hashes(local_dir, a_thresh, keywords, anomaly);
    }
}
/* ----------TODO----------
 * Figure out how to parse HTML for keywords, also save response to file (like meg)
 * add flag that allows you to only store the headers of anomaly requests (saves space). Make sure
 * you do it in a way that doesn't require resending the request, because it could be different.
 * For the lightweight version (less diskspace), hash the response and store that instead of the
 * full response. Then check for anomalies based off threshold given (or just sort all hashes,
 * putting the unique ones first.
 */
