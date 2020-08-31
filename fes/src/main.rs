use std::str;

mod fes_requests;
mod parse_arguments;
mod read_files;
mod write_files;
pub use crate::fes_requests::fes_request;
pub use crate::parse_arguments::parse_argument;
pub use crate::read_files::read_file;

fn main() {
    let matches = parse_argument::get_arguments();
    let urls_file = matches.value_of("urls_file").unwrap();
    let paths_file = matches.value_of("paths_file").unwrap();
    let output_dir = matches.value_of("output_dir").unwrap_or("fes_out");
    let parallel_requests: usize = matches.value_of("num").unwrap_or("20").parse().unwrap();
    let url_string: Vec<String> = read_file::read_lines(urls_file).unwrap();
    let urls: Vec<&str> = url_string.iter().map(AsRef::as_ref).collect();
    let paths_string: Vec<String> = read_file::read_lines(paths_file).unwrap();
    let paths: Vec<&str> = paths_string.iter().map(AsRef::as_ref).collect();

    fes_request::get_request(urls, paths, parallel_requests, output_dir);
}
/* ----------TODO----------
 * Figure out how to parse HTML for keywords, also save response to file (like meg)
 * For the lightweight version (less diskspace), hash the response and store that instead of the
 * full response. Then check for anomalies based off threshold given (or just sort all hashes,
 * putting the unique ones first.
 */
