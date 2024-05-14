//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="people"))]
#[component]
pub fn PersonSimpleCircle(
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
                <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,40a16,16,0,1,1-16,16A16,16,0,0,1,128,64Zm48,56H136v13.58l30.66,46a8,8,0,0,1-13.32,8.88l-25.34-38-25.34,38a8,8,0,1,1-13.32-8.88l30.66-46V120H80a8,8,0,0,1,0-16h96a8,8,0,0,1,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"></path>
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216ZM112,80a16,16,0,1,1,16,16A16,16,0,0,1,112,80Zm72,32a8,8,0,0,1-8,8H136v13.58l30.66,46a8,8,0,0,1-13.32,8.88l-25.34-38-25.34,38a8,8,0,1,1-13.32-8.88l30.66-46V120H80a8,8,0,0,1,0-16h96A8,8,0,0,1,184,112Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,192a92,92,0,1,1,92-92A92.1,92.1,0,0,1,128,220ZM116,80a12,12,0,1,1,12,12A12,12,0,0,1,116,80Zm64,32a4,4,0,0,1-4,4H132v18.79l31.33,47a4,4,0,0,1-6.66,4.44l-28.67-43-28.67,43a4,4,0,1,1-6.66-4.44l31.33-47V116H80a4,4,0,0,1,0-8h96A4,4,0,0,1,180,112Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,192a84,84,0,1,1,84-84A84.09,84.09,0,0,1,128,212ZM108,80a20,20,0,1,1,20,20A20,20,0,0,1,108,80Zm76,40a12,12,0,0,1-12,12H140v.51l26.12,41.05a12,12,0,0,1-3.68,16.56A11.83,11.83,0,0,1,156,192a12,12,0,0,1-10.13-5.56L128,158.35l-17.88,28.09a12,12,0,0,1-20.24-12.88L116,132.51V132H84a12,12,0,0,1,0-24h88A12,12,0,0,1,184,120Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218ZM114,80a14,14,0,1,1,14,14A14,14,0,0,1,114,80Zm68,32a6,6,0,0,1-6,6H134v16.18l31,46.49a6,6,0,1,1-10,6.66l-27-40.51-27,40.51a6,6,0,1,1-10-6.66l31-46.49V118H80a6,6,0,0,1,0-12h96A6,6,0,0,1,182,112Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216ZM112,80a16,16,0,1,1,16,16A16,16,0,0,1,112,80Zm72,32a8,8,0,0,1-8,8H136v13.58l30.66,46a8,8,0,0,1-13.32,8.88l-25.34-38-25.34,38a8,8,0,1,1-13.32-8.88l30.66-46V120H80a8,8,0,0,1,0-16h96A8,8,0,0,1,184,112Z"></path>
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