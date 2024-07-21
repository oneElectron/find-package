macro_rules! run_package_managers {
    ( $package_manager:ident::$function:ident $(,)?) => { // for debugging
        {
            let mut package_list: Vec<Pkg> = vec![];

            let mut $function = $package_manager::$function().await;
            package_list.append(&mut $function);

            package_list
        }
    };

    ( $( $package_manager:ident::$function:ident ),+ $(,)? ) => {
        {
            let mut package_list: Vec<Package> = vec![];


            let ($( $function ),*) = ( $( $package_manager::$function() ),* );

            let ($( mut $function ),*) = tokio::join!($($function),*);

            $(
                    package_list.append(&mut $function);
            )+

            package_list
        }
    };
}
