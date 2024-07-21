use crate::{PackageFile, Package};

use std::path::PathBuf;


#[derive(Clone, Debug)]
pub(crate) struct FileDatabase {
    pub(crate) files: Vec<PackageFile>,
}

impl FileDatabase {
    pub(crate) fn default_file_db_path() -> PathBuf {
        let path = PathBuf::from(std::env::var("HOME").unwrap());
        let path = path.join(".config/find_package/bin_database.filedb");

        path
    }

    pub(crate) fn parse_file_db<P: AsRef<str>>(path: P) -> Self {
        todo!();
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PackageDatabase {
    pub(crate) pkgs: Vec<Package>,
}

impl PackageDatabase {
    pub(crate) fn parse_pkg_db<P: AsRef<str>>(path: P) -> Self {
        todo!();
    }
}
