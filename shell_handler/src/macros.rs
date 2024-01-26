/// Input a list of (package manager name, "package manager bin name")
#[macro_export]
macro_rules! impl_enum_repo {
    ( $( ($package_manager_name: ident, $package_manager_bin_name: literal) ),+ $(,)? ) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug)]
        #[repr(u64)]
        enum Repo {
            $($package_manager_name,)+
            unknown
        }

        impl Repo {
            fn from_str(input: &str) -> Self {
                #[allow(unreachable_patterns)]
                match input {
                    $(
                        stringify!($package_manager_name) | $package_manager_bin_name => Self::$package_manager_name,
                    )+
                    _ => Self::unknown,
                }
            }

            fn to_str(&self) -> &str {
                match self {
                    $(Self::$package_manager_name => stringify!($package_manager_name),)+
                    Self::unknown => panic!("Cannot be unknown"),
                }
            }

            fn bin_name(&self) -> &str {
                match self {
                    $(Self::$package_manager_name => $package_manager_bin_name,)+
                    Self::unknown => panic!("Cannot be unknown"),
                }
            }
        }

        fn parse_filters(filter_list_str: &str) -> u64 {
            let mut output: u64 = 0;

            for filter in filter_list_str.split(',') {
                let mut valid: bool = false;
                $(
                    if filter == stringify!($package_manager_name) || filter == $package_manager_bin_name {
                        output = output | 0b1 << Repo::$package_manager_name as u64;
                        valid = true;
                    }
                )+
                if !valid {
                    println!("{YELLOW}Warning{RESET}: invalid filter");
                }
            }

            output
        }

        fn find_installed_package_managers() -> u64 {
            let mut output: u64 = 0;

            let ($($package_manager_name,)+) = ($( std::thread::spawn(|| {
                Repo::$package_manager_name.bin_name().find_in_path()
            }), )+);

            let ($($package_manager_name,)+) = ($($package_manager_name.join(),)+);

            $(
                if let Ok(s) = $package_manager_name {
                    if let Some(_) = s {
                        output = output | 1 << Repo::$package_manager_name as u64;
                    }
                }
            )+

            output
        }
    };
}

#[macro_export]
macro_rules! print_suggestions {
    ( $results: ident, $filter_package_managers: ident, $( $package_manager_name: ident ),+ $(,)? ) => {
        {
            $(
                for result in &$results {
                    if ($filter_package_managers == 0
                        || 1 << result.repo as u64 & $filter_package_managers >= 1)
                        && result.repo as u64 == Repo::$package_manager_name as u64
                    {
                        println!("  * {}: {}", result.repo.to_str(), result.pkg_name);
                    }
                }
            )+
        }
    };
}
