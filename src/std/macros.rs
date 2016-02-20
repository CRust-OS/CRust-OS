use collections::String;
#[macro_export]
macro_rules! format {
    ($fmt:expr, $($args:tt)*) => {
        {
            use core::fmt::Write;
            let mut x = String::new();
            write!(&mut x, $fmt, $($args)*).unwrap();
            x
        }
    }
}
