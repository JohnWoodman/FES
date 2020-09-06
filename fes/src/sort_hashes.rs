pub mod sort_hash {

    use map_vec::set;
    use std::collections::HashMap;
    use std::convert::TryInto;
    use std::fs;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::str;
    use walkdir::WalkDir;

    pub fn read_hashes(output_dir: &str, a_thresh: i32) {
        let line_break = "=============================================";
        let mut hash_list = Vec::new();
        let mut hash_only = Vec::new();
        for entry in WalkDir::new(output_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.path();
            let mut file_data = Vec::new();
            let metadata = fs::metadata(&f_name).unwrap();
            let mut keyword_data = Vec::new();
            if metadata.is_file() {
                file_data.push(f_name.to_str().unwrap().to_string());

                let file = File::open(f_name).expect("Unable to open file.");
                let reader = BufReader::new(file);
                let v: Vec<String> = reader.lines().filter_map(Result::ok).collect();
                let mut found = false;
                let mut first = true;
                let mut newline = 0;
                let mut found_status = false;
                for l in v.iter() {
                    if !keywords.is_empty() {
                        for key in keywords {
                            if l.contains(key) {
                                keyword_data.push(f_name);
                            }
                        }
                    }
                    if first {
                        file_data.push(l.to_owned());
                        first = false;
                    }
                    if found {
                        file_data.push(l.to_owned());
                        hash_only.push(l.to_owned());
                        break;
                    }
                    if newline == 2 {
                        found_status = true;
                    }
                    if found_status {
                        file_data.push(l.to_owned()[2..5].to_string());
                        found_status = false;
                        newline = 0;
                    }
                    if l.trim() == "Hashed Body:" {
                        found = true;
                    }
                    if l.trim().is_empty() {
                        newline = newline + 1;
                    }
                }
                hash_list.push(file_data);
            }
        }
        //count_vec.sort_by(|a, b| a.1.cmp(b.1));
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
            //println!("{} {} {} {}",hash[0],hash[1],hash[2],hash[3]);
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
                //println!("{} {} {}", hash_list[j][0],hash_list[j][1], hash_list[j][2]);
            }
        }

        /*
        let mut hash_frequencies = HashMap::new();

        for hash in hash_only.iter() {
            let count = hash_only.iter().filter(|&n| n == hash).count();
            if count <= a_thresh.try_into().unwrap() || a_thresh == 0 {
                hash_frequencies.insert(hash, count);
            }
        }

        let mut count_vec: Vec<_> = hash_frequencies.iter().collect();
        count_vec.sort_by(|a, b| a.1.cmp(b.1));

        println!("");
        println!("=============================================");
        println!("");
        println!("Anomaly Output (Sorted)");
        println!("");
        for fin_hash in count_vec.iter() {
            println!("=============================================");
            println!("{} ({})", fin_hash.0, fin_hash.1);
            println!("=============================================");
            for data in hash_list.iter() {
                if **fin_hash.0 == data[3] {
                    println!("[{}] {} ({})", data[2], data[1], data[0]);
                }
            }
            println!("");
        }
        */
    }
}
