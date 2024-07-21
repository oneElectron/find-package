use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) struct FileList {
    list: HashMap<String, Vec<BasicPkg>>,
}

impl FileList {
    pub(crate) fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, file_name: &str, pm: BasicPkg) {
        if self.list.contains_key(file_name) {
            self.list.get_mut(file_name).unwrap().push(pm);
        } else {
            self.list.insert(file_name.to_string(), vec![pm]);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn add_from(&mut self, file_list: Self) {
        for file in file_list.list {
            for pm in file.1 {
                self.add(&file.0, pm);
            }
        }
    }

    pub(crate) fn export(&self) -> String {
        let mut output = String::new();

        self.list.clone().into_iter();

        let mut sorted_list: Vec<(String, Vec<BasicPkg>)> = self.list.clone().into_iter().collect();

        sorted_list.sort_by(|a, b| {
            a.0.to_ascii_lowercase()
                .partial_cmp(&b.0.to_ascii_lowercase())
                .unwrap()
        });

        for file in sorted_list {
            output.push_str(&file.0.to_ascii_lowercase());
            output.push('|');

            for (i, pkg) in file.1.iter().enumerate() {
                if i != 0 {
                    output.push(',');
                }

                output.push_str(pkg.pm_name);
                output.push(':');
                output.push_str(&pkg.name);
            }

            output.push('\n');
        }

        output
    }
}

impl From<Vec<crate::Package>> for FileList {
    fn from(pkgs: Vec<crate::Package>) -> Self {
        let mut output = FileList::new();

        for pkg in pkgs {
            for file in pkg.pkg_files {
                output.add(
                    &file.name,
                    BasicPkg {
                        name: pkg.name.clone(),
                        pm_name: pkg.pm_name,
                    },
                )
            }
        }

        output
    }
}

#[derive(Clone, Debug)]
pub(crate) struct BasicPkg {
    name: String,
    pm_name: &'static str,
}

#[cfg(test)]
mod tests {}
