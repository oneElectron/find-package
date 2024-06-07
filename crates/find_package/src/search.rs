use crate::db::{FileDB, PkgDB};

use find_common::{Pkg, PkgFileType};

pub(crate) fn search_file(name: &str, file_type: Option<PkgFileType>) -> Pkg {
    let database = FileDB::parse_file_db("./file_db.filedb");

    todo!();
}

pub(crate) fn search_pkg(name: &str) -> Pkg {
    let database = PkgDB::parse_pkg_db("./pkg_db.pkgdb");

    todo!();
}
