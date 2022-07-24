#[macro_export]
macro_rules! hashmap {
    ($($x:expr => $y:expr),+ $(,)?) => {{
        let mut hashmap = ::std::collections::HashMap::new();
        $(
            hashmap.insert($x, $y);
        )+
        hashmap
    }};
    () => {{
        ::std::collections::HashMap::new()
    }};
}
