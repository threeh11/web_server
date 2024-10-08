use std::{fs, io};
use std::path::Path;

pub struct FileManager<'a> {
    path_file: &'a Path
}

impl<'a> FileManager<'a> {
    pub fn new_by_file(path_file: &'a Path) -> Self {
        Self {
            path_file
        }
    }

    pub fn exists_file(&self) -> bool {
        self.path_file.exists()
            && self.path_file.is_file()
            && fs::metadata(self.path_file).is_ok()
    }

    pub fn ok_permission_read(&self) -> bool {
        self.path_file.is_file()
    }

}