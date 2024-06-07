#[derive(Clone, Debug)]
pub struct Pkg {
    pub name: String,
    pub pkg_files: Vec<PkgFile>,
    pub pm_name: &'static str,
}

#[derive(Clone, Debug)]
pub struct PkgFile {
    pub name: String,
    pub file_type: PkgFileType,
}

#[derive(Clone, Debug)]
pub enum PkgFileType {
    Binary,
    Library,
    Other,
}

#[derive(Clone, Debug)]
pub struct PMFile {
    pub file_name: String,
    pub file_type: PkgFileType,
}
