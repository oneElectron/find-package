use crate::PackageFile;

use std::path::PathBuf;

use sha2::Digest;

pub(crate) fn get_tmp_folder() -> PathBuf {
    let mut tmp = std::env::temp_dir();
    tmp.push(get_random_folder_name());

    while tmp.exists() {
        tmp.pop();

        tmp.push(get_random_folder_name());
    }

    tmp
}

fn get_random_folder_name() -> String {
    let mut hash = sha2::Sha256::new();

    hash.update(rand::random::<[u8; 32]>());

    let result = hash.finalize();

    format!("{:x}", result)
}

pub(crate) fn guess_filetype_from_path(p: &str) -> crate::PackageFileType {
    todo!();
}

pub(crate) fn filter_path_list(paths: Vec<String>) -> Vec<PackageFile> {
    todo!();
}
