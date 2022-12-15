pub use derivatives::*;
pub trait HelloMacro {
    fn hello_macro();
}

pub trait EnumIter {
    fn iter() -> Vec<Self>
    where
        Self: Sized;
}
