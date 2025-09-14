pub trait HelloMacro {
    fn hello_macro();
}

// re-export macro only if feature is on
#[cfg(feature = "derive")]
pub use hello_macro_derive::*;