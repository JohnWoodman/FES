pub mod sort_hash {

    use std::collections::HashMap;
    use std::convert::TryInto;
    use std::fs;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::str;
    use walkdir::WalkDir;

    pub fn read_hashes(output_dir: &str, a_thresh: i32, keywords: Vec<&str>, anomaly: bool) {
        let line_break = "=============================================";
        let mut hash_list = Vec::new();
        let mut hash_only = Vec::new();
        let mut keyword_data = HashMap::new();
        for entry in WalkDir::new(output_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.path();
            let mut file_data = Vec::new();
            let metadata = fs::metadata(&f_name).unwrap();
            if metadata.is_file() {
                file_data.push(f_name.to_str().unwrap().to_string());

                let file = File::open(f_name).expect("Unable to open file.");
                let reader = BufReader::new(file);
                let v: Vec<String> = reader.lines().filter_map(Result::ok).collect();
                let mut found = false;
                let mut first = true;
                let mut newline = 0;
                let mut found_status = false;
                let mut found_body = false;
                let mut wrote_status = false;
                for l in v.iter() {
                    if first {
                        file_data.push(l.to_owned());
                        first = false;
                    }

                    if !keywords.is_empty() && found_body {
                        for key in &keywords {
                            if l.to_lowercase().contains(&key.to_lowercase()) {
                                keyword_data.insert(f_name.to_str().unwrap().to_string(), key);
                            }
                        }
                    }

                    if found {
                        file_data.push(l.to_owned());
                        hash_only.push(l.to_owned());
                        found_body = true;
                    }
                    if newline == 2 {
                        found_status = true;
                    }
                    if found_status {
                        file_data.push(l.to_owned()[2..5].to_string());
                        wrote_status = true;
                        found_status = false;
                        newline = 0;
                    }
                    if l.trim() == "Hashed Body:" {
                        found = true;
                    }
                    if l.trim().is_empty() && !wrote_status {
                        newline = newline + 1;
                    }
                }
                hash_list.push(file_data);
            }
        }
        for key in &keywords {
            println!("Keyword: {}", key);
            println!("{}", line_break);
            for (file, word) in &keyword_data {
                if *word == key {
                    println!("{}", file);
                }
            }
        }
        if anomaly {
            hash_list.sort_by(|a, b| a[3].cmp(&b[3]));
            let mut hashes = HashMap::new();
            let mut frequency = Vec::new();
            for hash in hash_list.iter() {
                match hashes.get(&hash[3]) {
                    Some(&val) => {
                        frequency[val] += 1;
                    }
                    _ => {
                        hashes.insert(&hash[3], frequency.len());
                        frequency.push(1);
                    }
                }
            }
            let mut all_hashes = Vec::new();
            let mut k = 0;
            for hash in hash_list.iter() {
                match hashes.get(&hash[3]) {
                    Some(&val) => {
                        let f = frequency[val];
                        all_hashes.push(vec![f, k]);
                        k += 1;
                    }
                    _ => k += 1,
                }
            }
            all_hashes.sort();
            let mut previous_hash = "";
            println!("{}\n\nAnomaly Output (Sorted)\n", line_break);
            for hash in all_hashes {
                if hash[0] <= a_thresh.try_into().unwrap() || a_thresh == 0 {
                    let j = hash[1];
                    if hash_list[j][3] != previous_hash {
                        println!(
                            "{}\n{} ({})\n{}",
                            line_break, hash_list[j][3], hash[0], line_break
                        );
                        previous_hash = &hash_list[j][3];
                    }
                    let data = &hash_list[j];
                    println!("[{}] {} ({})", data[2], data[1], data[0]);
                }
            }
        }
    }
}
