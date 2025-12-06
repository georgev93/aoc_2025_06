use std::fs::File;
use std::io::{BufRead, BufReader};

// pub fn parse_lines(path: &str) -> Vec<String> {
//     FileParser.parse_lines(path)
// }

pub trait FileParserTrait {
    fn parse_lines(&self) -> Vec<String>;
    fn parse_delimeted(&self) -> Vec<String>;
    fn parse_grid(&self) -> Vec<Vec<char>>;
    fn parse_grid_strings(&self) -> Vec<Vec<String>>;
}

pub struct FileParser {
    file: File,
}

impl FileParser {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap_or_else(|_| {
            panic!("Could not find file \"{path}\"");
        });

        Self { file }
    }
}

impl FileParserTrait for FileParser {
    fn parse_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for line in BufReader::new(&self.file).lines() {
            lines.push(line.unwrap().clone());
        }
        lines
    }

    fn parse_delimeted(&self) -> Vec<String> {
        let mut items: Vec<String> = Vec::new();
        for item in BufReader::new(&self.file).split(b',') {
            items.push(String::from_utf8(item.unwrap().trim_ascii().to_vec()).unwrap());
        }
        items
    }

    fn parse_grid(&self) -> Vec<Vec<char>> {
        let mut ret_vec: Vec<Vec<char>> = Vec::new();
        for line in BufReader::new(&self.file).lines() {
            let unwrapped_line = line.unwrap();
            let line_arr = unwrapped_line.as_bytes();
            let char_vec = line_arr.iter().map(|b| *b as char).collect::<Vec<char>>();

            ret_vec.push(char_vec);
        }
        ret_vec
    }

    fn parse_grid_strings(&self) -> Vec<Vec<String>> {
        let mut ret_vec: Vec<Vec<String>> = Vec::new();
        for line in BufReader::new(&self.file).lines() {
            let unwrapped_line = line.unwrap();

            let line_arr = unwrapped_line
                .trim_ascii()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect();

            ret_vec.push(line_arr);
        }
        ret_vec
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    #[should_panic(expected = "Could not find file \"not a path\"")]
    fn file_opener_bad_file() {
        FileParser::new("not a path").parse_lines();
    }

    #[test]
    fn file_opener() {
        let result_vec = FileParser::new("tests/file_opening_test.txt").parse_lines();

        assert_eq!(result_vec[0], "Here is a file");
        assert_eq!(result_vec[1], "It has stuff");
        assert_eq!(result_vec[2], "and");
        assert_eq!(result_vec[3], "Many Lines");
    }

    #[test]
    fn file_opener_single() {
        let result_vec = FileParser::new("tests/single_line_file.txt").parse_lines();

        assert_eq!(result_vec[0], "This file has one line");
    }

    #[test]
    #[should_panic(expected = "Could not find file \"tests/non_open_permission.txt\"")]
    fn file_permission_issue() {
        FileParser::new("tests/non_open_permission.txt").parse_lines();
    }

    #[test]
    fn single_line_comma() {
        let result_vec = FileParser::new("tests/comma_delimited.txt").parse_delimeted();
        assert_eq!(result_vec[0], "one");
        assert_eq!(result_vec[1], "two");
        assert_eq!(result_vec[2], "three");
        assert_eq!(result_vec[3], "four");
    }

    // #[test]
    // fn grid() {
    //     let result_vec = FileParser::new("tests/grid.txt").parse_grid();
    //     assert_eq!(result_vec[0], vec!['1', '2', '3', '4', '5']);
    //     assert_eq!(result_vec[1], vec!['a', 'b', 'c', 'd', 'e']);
    //     assert_eq!(result_vec[2], vec!['6', '7', '8', '9', '0']);
    // }

    #[test]
    fn grid_strings() {
        let result_vec = FileParser::new("tests/grid_spacing.txt").parse_grid_strings();
        assert_eq!(
            result_vec[0],
            vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string()
            ]
        );
        assert_eq!(
            result_vec[1],
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ]
        );
    }
}
