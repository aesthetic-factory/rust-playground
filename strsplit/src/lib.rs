#![allow(missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainer: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainer: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainer.find(self.delimiter) {
            let until_delimiter = &self.remainer[..next_delim];
            self.remainer = &self.remainer[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainer.is_empty() {
            // TODO: bug
            None
        } else {
            let rest = self.remainer;
            self.remainer = "";
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d"]);
}

#[test]
fn tail() {
    let haystack = "a b c d";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
