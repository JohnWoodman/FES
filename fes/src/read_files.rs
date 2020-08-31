pub mod read_file {

    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::str;

    pub fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
        let file = File::open(path).expect("Unable to open file.");
        let reader = BufReader::new(file);
        let v: Vec<String> = reader.lines().filter_map(Result::ok).collect();
        Ok(v)
    }
}
