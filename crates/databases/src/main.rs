#![feature(let_chains)]

pub(crate) mod file_list;
pub(crate) mod package_managers;
#[macro_use]
pub(crate) mod macros;

use file_list::FileList;
use package_managers::*;

use find_common::Pkg;

#[tokio::main]
async fn main() {
    let package_list = run_package_managers!(
        homebrew::homebrew_get_package_list,
        pacman::pacman_get_package_list,
    );

    let files = FileList::from(package_list);

    std::fs::write("./files.filedb", files.export()).unwrap();
}
