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
use linecount::count_lines;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
        let verbose: bool = matches.is_present("verbose");
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
        let path_lines: usize = count_lines(File::open(paths_file).unwrap()).unwrap();
        let url_lines: usize = count_lines(File::open(urls_file).unwrap()).unwrap();
        let max_total: u64 = path_lines as u64 * url_lines as u64;
        let paths_string: Vec<String> = read_file::read_lines(paths_file).unwrap();
        let paths: Vec<&str> = paths_string.iter().map(AsRef::as_ref).collect();
        let file = File::open(urls_file).expect("Unable to open file.");
        let reader = BufReader::new(file);
        let mut url_string: Vec<String> = Vec::new();
        let mut buf_counter = 0;
        for line in reader.lines() {
            if buf_counter < 1000 {
                url_string.push(line.unwrap_or("".to_string()));
                buf_counter += 1;
            } else {
                buf_counter = 0;
                let urls: Vec<&str> = url_string.iter().map(AsRef::as_ref).collect();
                fes_request::get_request(
                    urls,
                    &paths,
                    parallel_requests,
                    output_dir,
                    hash_write,
                    &allowed_status,
                    &disallowed_status,
                    timeout,
                    follow_redirects,
                    max_total,
                    verbose,
                );
                url_string = Vec::new();
            }
        }
        if !url_string.is_empty() {
            let urls: Vec<&str> = url_string.iter().map(AsRef::as_ref).collect();
            fes_request::get_request(
                urls,
                &paths,
                parallel_requests,
                output_dir,
                hash_write,
                &allowed_status,
                &disallowed_status,
                timeout,
                follow_redirects,
                max_total,
                verbose,
            );
        }
        if matches.is_present("dir") {
            sort_hash::read_hashes(output_dir, a_thresh, keywords, anomaly);
        }
    } else if matches.is_present("dir") {
        sort_hash::read_hashes(local_dir, a_thresh, keywords, anomaly);
    }
}
