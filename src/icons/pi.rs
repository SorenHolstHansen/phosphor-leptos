//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="finance", feature ="development"))]
#[component]
pub fn Pi(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM172,168a12,12,0,0,0,12-12,8,8,0,0,1,16,0,28,28,0,0,1-56,0V96H112v80a8,8,0,0,1-16,0V96H88a24,24,0,0,0-24,24,8,8,0,0,1-16,0A40,40,0,0,1,88,80H192a8,8,0,0,1,0,16H160v60A12,12,0,0,0,172,168Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M196,200H88V64h80V172A28,28,0,0,0,196,200Z" opacity="0.2"></path>
    <path d="M232,172a36,36,0,0,1-72,0V72H96V200a8,8,0,0,1-16,0V72H72a40,40,0,0,0-40,40,8,8,0,0,1-16,0A56.06,56.06,0,0,1,72,56H224a8,8,0,0,1,0,16H176V172a20,20,0,0,0,40,0,8,8,0,0,1,16,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M228,172a32,32,0,0,1-64,0V68H92V200a4,4,0,0,1-8,0V68H72a44.05,44.05,0,0,0-44,44,4,4,0,0,1-8,0A52.06,52.06,0,0,1,72,60H224a4,4,0,0,1,0,8H172V172a24,24,0,0,0,48,0,4,4,0,0,1,8,0Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M236,172a40,40,0,0,1-80,0V76H100V200a12,12,0,0,1-24,0V76H72a36,36,0,0,0-36,36,12,12,0,0,1-24,0A60.07,60.07,0,0,1,72,52H224a12,12,0,0,1,0,24H180v96a16,16,0,0,0,32,0,12,12,0,0,1,24,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M230,172a34,34,0,0,1-68,0V70H94V200a6,6,0,0,1-12,0V70H72a42,42,0,0,0-42,42,6,6,0,0,1-12,0A54.06,54.06,0,0,1,72,58H224a6,6,0,0,1,0,12H174V172a22,22,0,0,0,44,0,6,6,0,0,1,12,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,172a36,36,0,0,1-72,0V72H96V200a8,8,0,0,1-16,0V72H72a40,40,0,0,0-40,40,8,8,0,0,1-16,0A56.06,56.06,0,0,1,72,56H224a8,8,0,0,1,0,16H176V172a20,20,0,0,0,40,0,8,8,0,0,1,16,0Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().unwrap_or(TextProp::from(""))
            class=move || class.get().unwrap_or(TextProp::from(""))
        >
            {body}
        </svg>
    }
}