pub mod write_file {

    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use hyper::http::StatusCode;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::LineWriter;
    use std::path::Path;
    use std::str;

    pub fn write_results(
        output_data: &Vec<String>,
        output_body: String,
        output_dir: &str,
        hash_write: bool,
        allowed_status: &Vec<&str>,
        disallowed_status: &Vec<&str>,
    ) {
        if !allowed_status.is_empty() {
            if !allowed_status.iter().any(|&i| i == output_data[1]) {
                return;
            }
        }
        if !disallowed_status.is_empty() {
            if disallowed_status.iter().any(|&i| i == output_data[1]) {
                return;
            }
        }

        let unique_url = output_data[0].as_str();
        let start_index = unique_url.find('/').unwrap() + 2;
        let temp_dir = &unique_url[start_index..];
        let end_index = temp_dir.find('/').unwrap();
        let endpoint = format!("{}", &temp_dir[end_index..]);
        let host = format!("{}", &temp_dir[..end_index]);
        let main_dir = output_dir;
        let final_dir = format!("{}/{}", main_dir, &temp_dir[..end_index]);

        let mut hasher = Sha256::new();
        hasher.input_str(unique_url);
        let test_hash = hasher.result_str();
        fs::create_dir_all(&final_dir).unwrap();

        let path = Path::new(&final_dir).join(test_hash);
        let display = path.display();

        let file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let mut file = LineWriter::new(file);

        for (pos, i) in output_data.iter().enumerate() {
            if pos == 1 {
                file.write_all(b"\n> GET ").expect("didn't work");
                file.write_all(endpoint.as_bytes()).expect("nope");
                file.write_all(b" HTTP/1.1\n").expect("nope");
                file.write_all(b"> Host: ").expect("nope");
                file.write_all(host.as_bytes()).expect("nope");
                file.write_all(b"\n> User-Agent: Mozilla/5.0 (compatible; fes/0.1; +https://github.com/JohnWoodman/fes)\n\n").expect("nope");
                let status = StatusCode::from_bytes(i.as_bytes()).unwrap();
                let full_status = format!("< {} {}\n", i, status.canonical_reason().unwrap());
                file.write_all(full_status.as_bytes())
                    .expect("Coudlnt' write");
            } else if pos == 0 {
                file.write_all(i.as_bytes()).expect("Unable to write data");
                file.write_all(b"\n").expect("Unable to write new line");
            } else {
                file.write_all(b"< ").expect("Unable to write data");
                file.write_all(i.as_bytes()).expect("Unable to write data");
                file.write_all(b"\n").expect("Unable to write new line");
            }
        }

        let mut hasher = Sha256::new();
        hasher.input_str(output_body.as_str());
        let body_hash = hasher.result_str();
        //hash_list.push(body_hash.as_str());
        file.write_all(b"\n").expect("Unable to write new line");
        file.write_all(b"Hashed Body:\n")
            .expect("Unable to write to file");
        file.write_all(body_hash.as_bytes())
            .expect("Unable to write new line");
        file.write_all(b"\n").expect("Unable to write new line");

        if !hash_write {
            file.write_all(b"\n").expect("Unable to write new line");
            file.write_all(output_body.as_bytes())
                .expect("Unable to write new line");
        }
    }
}
