mod db;
mod search;
#[cfg(feature = "package_getter")]
mod package_getter;

#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub pkg_files: Vec<PackageFile>,
    pub pm_name: &'static str,
}

#[derive(Clone, Debug)]
pub struct PackageFile {
    pub name: String,
    pub file_type: PackageFileType,
}

#[derive(Clone, Debug)]
pub enum PackageFileType {
    Binary,
    Library,
    Other,
}
