mod icons;
pub use icons::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconWeight {
    Bold,
    Duotone,
    Fill,
    Light,
    Regular,
    Thin,
}
