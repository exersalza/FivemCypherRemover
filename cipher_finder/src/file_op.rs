use std::io::Read;
use std::{fs, path::PathBuf};

use crate::utils::{check_regex, CIPHER_REGEX, SIMPLE_URL_REGEX};

pub struct ScannedFile {
    path: PathBuf,
    findings: Vec<(i32, f32)>, // line number, confidence
}

impl ScannedFile {
    /// Creates new ScannedFile object and Scans the file in creation
    pub fn new(path: PathBuf) -> std::io::Result<ScannedFile> {
        let mut ret = Self {
            path,
            findings: vec![],
        };
        ret.scan_file()?; // let the caller handle any errors.

        Ok(ret)
    }

    /// gets the file contents, converts it to an utf8 but lossy
    fn get_file_contents(&self) -> std::io::Result<Vec<String>> {
        let mut file = fs::File::open(&self.path)?;
        let mut buf = vec![];

        file.read_to_end(&mut buf)?;

        let cont = String::from_utf8_lossy(&buf);

        Ok(cont.split('\n').map(str::to_string).collect())
    }

    /// Scans file
    fn scan_file(&mut self) -> std::io::Result<Vec<(Vec<String>, usize)>> {
        let contents = self.get_file_contents()?;
        let mut ret = vec![];

        for (ln, line) in contents.into_iter().enumerate() {
            if line.contains('\n') {
                continue;
            }

            let line = line.as_str();

            ret.push((check_regex(&CIPHER_REGEX, line), ln));
            ret.push((check_regex(&SIMPLE_URL_REGEX, line), ln));
        }

        Ok(ret)
    }

    /// Add infected lines to the lister
    fn add_infected(&mut self, ln: i32, confidence: f32) {
        let _ = &self.findings.push((ln, confidence));
    }

    /// getter for the vec of infected lines
    pub fn get_infected(&self) -> Vec<(i32, f32)> {
        self.findings.to_owned()
    }
}
