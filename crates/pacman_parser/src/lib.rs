use std::{collections::HashMap, path::PathBuf};

pub fn parse_pacman_desc_file(contents: &str) -> HashMap<String, Vec<String>> {
    let mut output = HashMap::new();
    for group in contents.split("\n\n") {
        let mut k = String::new();
        let mut v = vec![];

        for line in group.lines() {
            if line.starts_with("%") && k.is_empty() {
                k = line
                    .trim()
                    .trim_start_matches("%")
                    .trim_end_matches("%")
                    .to_owned();
            } else {
                v.push(line.trim().to_owned());
            }
        }

        output.insert(k, v);
    }

    output
}

pub fn parse_pacman_files_file(contents: &str) -> Vec<String> {
    let mut output = vec![];

    for line in contents.lines() {
        if !line.starts_with("%") {
            output.push(line.trim().to_owned());
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    const DESC_FILE: &str = include_str!("../../../data/pacman/desc_example");
    const FILES_FILE: &str = include_str!("../../../data/pacman/files_example");

    #[test]
    fn parse_desc_file() {
        let c = parse_pacman_desc_file(DESC_FILE);

        assert_eq!(c.get("NAME").unwrap()[0].as_str(), "binutils");
    }

    #[test]
    fn parse_files_file() {
        let c = parse_pacman_files_file(FILES_FILE);

        assert_eq!(c[0].as_str(), "etc/");
    }
}
