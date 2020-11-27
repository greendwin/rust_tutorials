#[macro_export]
macro_rules! vec_str {
    [] => {
        Vec::<String>::new()
    };
    [$($s:expr),+] => {
        vec![$($s.to_owned()),+]
    };
    [$($s:expr,)+] => {
        vec![$($s.to_owned()),+]
    };
}
