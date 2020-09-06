pub mod fes_request {

    use crate::write_files;
    use futures::{stream, StreamExt};
    use reqwest::header;
    use std::str;
    use std::time::Duration;
    use tokio;
    use url::Url;
    #[tokio::main]
    pub async fn get_request(
        urls: Vec<&str>,
        paths: Vec<&str>,
        parallel_requests: usize,
        output_dir: &str,
        hash_write: bool,
        allowed_status: Vec<&str>,
        disallowed_status: Vec<&str>,
    ) {
        let mut url_test = Vec::new();
        for path in paths {
            for url in &urls {
                let mut full_url = String::new();
                full_url.push_str(url);
                full_url.push_str(path);
                url_test.push(Url::parse(&full_url).unwrap());
            }
        }

        let bodies = stream::iter(url_test)
            .map(|url| {
                let custom_redirect = reqwest::redirect::Policy::none();
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::USER_AGENT,
                    header::HeaderValue::from_static(
                        "Mozilla/5.0 (compatible, fes/0.1; +https://github.com/JohnWoodman/fes)",
                    ),
                );

                let client = reqwest::Client::builder()
                    .redirect(custom_redirect)
                    .default_headers(headers)
                    .build()
                    .unwrap();

                tokio::spawn(async move {
                    let resp = client.get(url).timeout(Duration::from_secs(3)).send().await;
                    resp
                })
            })
            .buffer_unordered(parallel_requests);

        bodies
            .for_each(|b| async {
                match b {
                    Ok(Ok(b)) => {
                        let mut vec = Vec::new();
                        let url = b.url().as_str().to_string();
                        let headers = &b.headers();
                        let status = b.status().as_str().to_string();
                        vec.push(url);
                        vec.push(status);
                        for (key, value) in headers.iter() {
                            let pair = format!("{}: {}", key.as_str(), value.to_str().unwrap());
                            vec.push(pair);
                        }
                        match b.text().await {
                            Ok(text_output) => {
                                let body_test = text_output;
                                write_files::write_file::write_results(
                                    &vec,
                                    body_test.to_string(),
                                    output_dir,
                                    hash_write,
                                    &allowed_status,
                                    &disallowed_status,
                                );
                            }
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    Ok(Err(e)) => println!("Got an error: {}", e),
                    Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
                }
            })
            .await;
    }
}
