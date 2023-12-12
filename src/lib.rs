mod icons;
pub use icons::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IconWeight {
    Bold,
    Duotone,
    Fill,
    Light,
    Regular,
    Thin,
}
