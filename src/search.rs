use crate::db::{FileDatabase, PackageDatabase};

use crate::{Package, PackageFileType};

pub(crate) fn search_file(name: &str, file_type: Option<PackageFileType>) -> Package {
    let database = FileDatabase::parse_file_db("./file_db.filedb");

    todo!();
}

pub(crate) fn search_pkg(name: &str) -> Package {
    let database = PackageDatabase::parse_pkg_db("./pkg_db.pkgdb");

    todo!();
}
