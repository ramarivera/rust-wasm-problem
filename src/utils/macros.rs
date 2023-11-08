#[allow(unused_imports)]
use const_format::concatcp;

#[allow(unused_macros)]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            core::convert::From::from([$(($k, $v),)*])
        }
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        {
            core::convert::From::from([$($v,)*])
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_constants {
    ($name:ident, $value:literal) => {
        paste! {
            pub const [<$name _CHAR>]: char = $value as char;
            pub const [<$name _STR>]: &str = concatcp!($value);
        }
    };
}

#[allow(unused_imports)]
pub(crate) use collection;

#[allow(unused_imports)]
pub(crate) use define_constants;
