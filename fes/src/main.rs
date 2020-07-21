use futures::executor::block_on;
use futures::{stream, StreamExt};
use reqwest::Client;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio;

fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path).expect("Unable to open file.");
    let reader = BufReader::new(file);
    let v: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(v)
}
fn main() {
    let url_String: Vec<String> = read_lines("urls.txt").unwrap();
    let urls: Vec<&str> = url_String.iter().map(AsRef::as_ref).collect();
    let paths_String: Vec<String> = read_lines("paths.txt").unwrap();
    let paths: Vec<&str> = paths_String.iter().map(AsRef::as_ref).collect();
    let now = Instant::now();
    get_request(urls, paths);
    println!("{:?}", now.elapsed());
}
const PARALLEL_REQUESTS: usize = 20;
#[tokio::main]
async fn get_request(urls: Vec<&str>, paths: Vec<&str>) {
    let client = Client::new();
    for path in paths {
        let urls = urls.clone();
        let bodies = stream::iter(urls)
            .map(|url| {
                let client = &client;
                async move {
                    let mut full_url = String::new();
                    full_url.push_str(url);
                    full_url.push_str(path);
                    let resp = client
                        .get(&full_url)
                        .timeout(Duration::from_secs(3))
                        .send()
                        .await;
                    resp
                }
            })
            .buffer_unordered(PARALLEL_REQUESTS);

        bodies
            .for_each(|b| async {
                match b {
                    Ok(b) => println!("Got bytes from {:?}", b.status()),
                    Err(e) => println!("Got an error: {}", e),
                }
            })
            .await;
    }
}

/* ----------TODO----------
 * Get status code, and save to file along with full url
 * Figure out how to parse HTML for keywords, also save response to file (like meg)
 * Add threshold based on number of duplicate responses (200 most likely). This will mean that you
 * don't output anything for a url until its been checked for all the paths (would that take up too
 * much space in RAM?) maybe it should write everything to disk first, and then parse for threshold
 * limit and delete those??? maybe add as seperate flag to parse threshold.
 */
