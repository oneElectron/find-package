mod file_list;
mod package_managers;
#[macro_use]
mod macros;

use file_list::FileList;
use package_managers::*;

use crate::Package;

async fn run() {
    let package_list = run_package_managers!(
        homebrew::homebrew_get_package_list,
        pacman::pacman_get_package_list,
    );

    let files = FileList::from(package_list);

    std::fs::write("./files.filedb", files.export()).unwrap();
}
