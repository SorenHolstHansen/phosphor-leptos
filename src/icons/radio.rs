//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Radio(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M160,180a36,36,0,1,0-36-36A36,36,0,0,0,160,180Zm0-48a12,12,0,1,1-12,12A12,12,0,0,1,160,132Zm56-64H113.76l81.69-24.5a12,12,0,0,0-6.9-23l-160,48A12,12,0,0,0,20,80V200a20,20,0,0,0,20,20H216a20,20,0,0,0,20-20V88A20,20,0,0,0,216,68Zm-4,128H44V92H212ZM60,124a12,12,0,0,1,12-12H96a12,12,0,0,1,0,24H72A12,12,0,0,1,60,124Zm0,40a12,12,0,0,1,12-12H96a12,12,0,0,1,0,24H72A12,12,0,0,1,60,164Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,80H32V200a8,8,0,0,0,8,8H216a8,8,0,0,0,8-8V88A8,8,0,0,0,216,80Zm-56,96a32,32,0,1,1,32-32A32,32,0,0,1,160,176Z"
        opacity="0.2"
    ></path>
    <path d="M104,176a8,8,0,0,1-8,8H64a8,8,0,0,1,0-16H96A8,8,0,0,1,104,176Zm-8-40H64a8,8,0,0,0,0,16H96a8,8,0,0,0,0-16Zm0-32H64a8,8,0,0,0,0,16H96a8,8,0,0,0,0-16ZM232,88V200a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V80a8,8,0,0,1,5.7-7.66l160-48a8,8,0,0,1,4.6,15.33L86.51,72H216A16,16,0,0,1,232,88ZM216,200V88H40V200H216Zm-16-56a40,40,0,1,1-40-40A40,40,0,0,1,200,144Zm-16,0a24,24,0,1,0-24,24A24,24,0,0,0,184,144Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,72H86.51L194.3,39.67a8,8,0,0,0-4.6-15.33l-160,48A8,8,0,0,0,24,80V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V88A16,16,0,0,0,216,72ZM96,184H56a8,8,0,0,1,0-16H96a8,8,0,0,1,0,16Zm0-32H56a8,8,0,0,1,0-16H96a8,8,0,0,1,0,16Zm0-32H56a8,8,0,0,1,0-16H96a8,8,0,0,1,0,16Zm72,52a28,28,0,1,1,28-28A28,28,0,0,1,168,172Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M102,112a6,6,0,0,1-6,6H64a6,6,0,0,1,0-12H96A6,6,0,0,1,102,112Zm-6,26H64a6,6,0,0,0,0,12H96a6,6,0,0,0,0-12Zm0,32H64a6,6,0,0,0,0,12H96a6,6,0,0,0,0-12ZM230,88V200a14,14,0,0,1-14,14H40a14,14,0,0,1-14-14V80a6,6,0,0,1,4.28-5.75l160-48a6,6,0,0,1,3.44,11.5L72.88,74H216A14,14,0,0,1,230,88Zm-12,0a2,2,0,0,0-2-2H38V200a2,2,0,0,0,2,2H216a2,2,0,0,0,2-2Zm-20,56a38,38,0,1,1-38-38A38,38,0,0,1,198,144Zm-12,0a26,26,0,1,0-26,26A26,26,0,0,0,186,144Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M104,176a8,8,0,0,1-8,8H64a8,8,0,0,1,0-16H96A8,8,0,0,1,104,176Zm-8-40H64a8,8,0,0,0,0,16H96a8,8,0,0,0,0-16Zm0-32H64a8,8,0,0,0,0,16H96a8,8,0,0,0,0-16ZM232,88V200a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V80a8,8,0,0,1,5.7-7.66l160-48a8,8,0,0,1,4.6,15.33L86.51,72H216A16,16,0,0,1,232,88ZM216,200V88H40V200H216Zm-16-56a40,40,0,1,1-40-40A40,40,0,0,1,200,144Zm-16,0a24,24,0,1,0-24,24A24,24,0,0,0,184,144Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M100,176a4,4,0,0,1-4,4H64a4,4,0,0,1,0-8H96A4,4,0,0,1,100,176Zm-4-36H64a4,4,0,0,0,0,8H96a4,4,0,0,0,0-8ZM228,88V200a12,12,0,0,1-12,12H40a12,12,0,0,1-12-12V80a4,4,0,0,1,2.85-3.81l160-48a4,4,0,0,1,2.3,7.66L59.25,76H216A12,12,0,0,1,228,88Zm-8,0a4,4,0,0,0-4-4H36V200a4,4,0,0,0,4,4H216a4,4,0,0,0,4-4Zm-24,56a36,36,0,1,1-36-36A36,36,0,0,1,196,144Zm-8,0a28,28,0,1,0-28,28A28,28,0,0,0,188,144ZM96,108H64a4,4,0,0,0,0,8H96a4,4,0,0,0,0-8Z"></path>
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
