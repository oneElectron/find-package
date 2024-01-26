
/// Input a list of (package manager name, "package manager bin name")
#[macro_export]
macro_rules! impl_enum_repo {
( $( ($package_manager_name: ident, $package_manager_bin_name: literal) ),+ $(,)? ) => {
    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Debug)]
    #[repr(u64)]
    pub enum Repo {
        $($package_manager_name,)+
        unknown,
    }

    use find_in_path::FindInPath;

    impl From<&str> for Repo {
        fn from(value: &str) -> Self {
            #[allow(unreachable_patterns)]
            match value {
                $(
                    stringify!($package_manager_name) | $package_manager_bin_name => Self::$package_manager_name,
                )+
                _ => Self::unknown,
            }
        }
    }

    impl Repo {
        pub fn to_str(&self) -> &str {
            match self {
                $(Self::$package_manager_name => stringify!($package_manager_name),)+
                Self::unknown => panic!("Cannot be unknown"),
            }
        }

        pub fn bin_name(&self) -> &str {
            match self {
                $(Self::$package_manager_name => $package_manager_bin_name,)+
                Self::unknown => panic!("Cannot be unknown"),
            }
        }
    }

    pub fn parse_filters(filter_list_str: &str) -> u64 {
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
                println!("Warning: invalid filter");
            }
        }

        output
    }

    pub fn find_installed_package_managers() -> u64 {
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
}
}