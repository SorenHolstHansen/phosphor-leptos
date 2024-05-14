mod icons;
pub use icons::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconWeight {
    Fill, Duotone, Thin, Bold, Light, Regular
}
