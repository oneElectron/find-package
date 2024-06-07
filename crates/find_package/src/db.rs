use find_common::{PMFile, Pkg};

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) struct FileDB {
    pub(crate) files: Vec<PMFile>,
}

impl FileDB {
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
pub(crate) struct PkgDB {
    pub(crate) pkgs: Vec<Pkg>,
}

impl PkgDB {
    pub(crate) fn parse_pkg_db<P: AsRef<str>>(path: P) -> Self {
        todo!();
    }
}
