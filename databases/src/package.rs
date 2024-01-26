#[derive(Clone, Debug)]
pub(crate) struct Pkg {
    pub(crate) name: String,
    pub(crate) pkg_files: Vec<PkgFile>,
    pub(crate) pm_name: &'static str,
}

#[derive(Clone, Debug)]
pub(crate) struct PkgFile {
    pub(crate) name: String,
    pub(crate) file_type: PMFileType,
}

#[derive(Clone, Debug)]
pub(crate) enum PMFileType {
    Binary,
}
