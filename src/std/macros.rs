#[macro_export]
macro_rules! format {
    ($fmt:expr, $($args:tt)*) => {
        {
            use collections::String;
            let mut x = String::new();
            write!(&mut x, $fmt, $($args)*).unwrap();
            x
        }
    }
}
