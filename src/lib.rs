#![doc = r" Phosphor is a flexible icon family for interfaces, diagrams,"]
#![doc = r" presentations — whatever, really."]
#![doc = r" You can explore the available icons at [phosphoricons.com](https://phosphoricons.com)."]
#![doc = r""]
#![doc = r" ```"]
#![doc = r" use leptos::*;"]
#![doc = r" use phosphor_leptos::{Icon, IconWeight, HORSE, HEART, CUBE};"]
#![doc = r""]
#![doc = r" #[component]"]
#![doc = r" fn MyComponent() -> impl IntoView {"]
#![doc = r"     view! {"]
#![doc = r"         <Icon icon=HORSE />"]
#![doc = r##"         <Icon icon=HEART color="#AE2983" weight=IconWeight::Fill size="32px" />"##]
#![doc = r#"         <Icon icon=CUBE color="teal" weight=IconWeight::Duotone />"#]
#![doc = r"     }"]
#![doc = r" }"]
#![doc = r" ```"]
use leptos::*;
mod icons;
pub use icons::*;
#[doc = r" An icon's weight or style."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconWeight {
    Bold,
    Duotone,
    Fill,
    Light,
    Regular,
    Thin,
}
#[doc = r" The SVG path data for all weights of a particular icon."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconWeightData([&'static str; 6usize]);
impl IconWeightData {
    #[doc = r" Retrieve the SVG paths for the given weight."]
    #[doc = r""]
    #[doc = r" The returned string slice contains raw path elements."]
    #[doc = r" To render them manually, you'll need to provide them to"]
    #[doc = r" an SVG component's `inner_html` property."]
    #[doc = r""]
    #[doc = r" ```"]
    #[doc = r" # use leptos::*;"]
    #[doc = r" # #[component]"]
    #[doc = r" # fn MyComponent() -> impl IntoView {"]
    #[doc = r" use phosphor_leptos::{ACORN, IconWeight};"]
    #[doc = r""]
    #[doc = r" let raw_html = ACORN.get(IconWeight::Regular);"]
    #[doc = r" view! {"]
    #[doc = r"     <svg inner_html=raw_html />"]
    #[doc = r" }"]
    #[doc = r" # }"]
    #[doc = r" ```"]
    pub fn get(&self, weight: IconWeight) -> &'static str {
        match weight {
            IconWeight::Bold => self.0[0usize],
            IconWeight::Duotone => self.0[1usize],
            IconWeight::Fill => self.0[2usize],
            IconWeight::Light => self.0[3usize],
            IconWeight::Regular => self.0[4usize],
            IconWeight::Thin => self.0[5usize],
        }
    }
}
#[doc = r" A convenient alias for passing around references to [IconWeightData]."]
#[doc = r""]
#[doc = r" While [IconWeightData] is `Copy`, it's not particularly beneficial to pass"]
#[doc = r" all those bytes around (48 bytes on WASM, 96 bytes on 64 bit systems)."]
pub type IconData = &'static IconWeightData;
#[doc = r" A thin wrapper around `<svg />` for displaying Phosphor icons."]
#[doc = r""]
#[doc = r" ```"]
#[doc = r" use leptos::*;"]
#[doc = r" use phosphor_leptos::{Icon, IconWeight, HORSE, HEART, CUBE};"]
#[doc = r""]
#[doc = r" #[component]"]
#[doc = r" fn MyComponent() -> impl IntoView {"]
#[doc = r"     view! {"]
#[doc = r"         <Icon icon=HORSE />"]
#[doc = r##"         <Icon icon=HEART color="#AE2983" weight=IconWeight::Fill size="32px" />"##]
#[doc = r#"         <Icon icon=CUBE color="teal" weight=IconWeight::Duotone />"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
#[component]
pub fn Icon(
    #[doc = r" The icon data to display."] icon: IconData,
    #[doc = r#" Icon weight/style. This can also be used, for example, to "toggle" an icon's state:"#]
    #[doc = r" a rating component could use Stars with [IconWeight::Regular] to denote an empty star,"]
    #[doc = r" and [IconWeight::Fill] to denote a filled star."]
    # [prop (into , default = MaybeSignal :: Static (IconWeight :: Regular))]
    weight: MaybeSignal<IconWeight>,
    #[doc = r" Icon height & width. As with standard React elements,"]
    #[doc = r" this can be a number, or a string with units in"]
    #[doc = r" `px`, `%`, `em`, `rem`, `pt`, `cm`, `mm`, `in`."]
    # [prop (into , default = TextProp :: from ("1em"))]
    size: TextProp,
    #[doc = r" Icon stroke/fill color."]
    #[doc = r""]
    #[doc = r" This can be any CSS color string, including"]
    #[doc = r" `hex`, `rgb`, `rgba`, `hsl`, `hsla`, named colors,"]
    #[doc = r" or the special `currentColor` variable."]
    #[doc = r""]
    # [prop (into , default = TextProp :: from ("currentColor"))]
    color: TextProp,
    #[doc = r" Flip the icon horizontally."]
    #[doc = r""]
    #[doc = r" This can be useful in RTL languages where normal"]
    #[doc = r" icon orientation is not appropriate."]
    # [prop (into , default = MaybeSignal :: Static (false))]
    mirrored: MaybeSignal<bool>,
    #[doc = r" The HTML ID of the underlying SVG element."]
    #[prop(into, optional)]
    id: MaybeProp<TextProp>,
    #[doc = r" The CSS class property of the underlying SVG element."]
    #[prop(into, optional)]
    class: MaybeProp<TextProp>,
) -> impl IntoView {
    let html = move || icon.get(weight.get());
    let transform = move || mirrored.get().then_some("scale(-1, 1)");
    let height = size.clone();
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().map(|id| id.get())
            class=move || class.get().map(|cls| cls.get())
            inner_html=html
        ></svg>
    }
}
