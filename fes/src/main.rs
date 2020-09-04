use std::str;

mod fes_requests;
mod parse_arguments;
mod read_files;
mod sort_hashes;
mod write_files;
pub use crate::fes_requests::fes_request;
pub use crate::parse_arguments::parse_argument;
pub use crate::read_files::read_file;
pub use crate::sort_hashes::sort_hash;

fn main() {
    let matches = parse_argument::get_arguments();
    let hash_write: bool = matches.is_present("hash_write");
    let output_dir = matches.value_of("output_dir").unwrap_or("fes_out");
    let anomaly_dir = matches.value_of("dir").unwrap_or("fes_out");
    let a_thresh = matches
        .value_of("limit_val")
        .unwrap_or("0")
        .parse::<i32>()
        .expect("Error: Integer not specified for anomaly threshold");
    let parallel_requests: usize = matches.value_of("num").unwrap_or("20").parse().unwrap();

    if matches.is_present("paths_file") && matches.is_present("urls_file") {
        let paths_file = matches.value_of("paths_file").unwrap();
        let urls_file = matches.value_of("urls_file").unwrap();
        let url_string: Vec<String> = read_file::read_lines(urls_file).unwrap();
        let urls: Vec<&str> = url_string.iter().map(AsRef::as_ref).collect();
        let paths_string: Vec<String> = read_file::read_lines(paths_file).unwrap();
        let paths: Vec<&str> = paths_string.iter().map(AsRef::as_ref).collect();
        fes_request::get_request(urls, paths, parallel_requests, output_dir, hash_write);
        sort_hash::read_hashes(output_dir, a_thresh);
    } else if matches.is_present("dir") {
        sort_hash::read_hashes(anomaly_dir, a_thresh);
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
