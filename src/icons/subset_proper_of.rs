//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance", feature = "development"))]
#[component]
pub fn SubsetProperOf(
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
            IconWeight::Fill => view! {
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM128,176h48a8,8,0,0,1,0,16H128a64,64,0,0,1,0-128h48a8,8,0,0,1,0,16H128a48,48,0,0,0,0,96Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M200,48V208H128a80,80,0,0,1-80-80h0a80,80,0,0,1,80-80Z" opacity="0.2"></path>
    <path d="M208,208a8,8,0,0,1-8,8H128a88,88,0,0,1,0-176h72a8,8,0,0,1,0,16H128a72,72,0,0,0,0,144h72A8,8,0,0,1,208,208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M204,208a4,4,0,0,1-4,4H128a84,84,0,0,1,0-168h72a4,4,0,0,1,0,8H128a76,76,0,0,0,0,152h72A4,4,0,0,1,204,208Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M212,208a12,12,0,0,1-12,12H128a92,92,0,0,1,0-184h72a12,12,0,0,1,0,24H128a68,68,0,0,0,0,136h72A12,12,0,0,1,212,208Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M206,208a6,6,0,0,1-6,6H128a86,86,0,0,1,0-172h72a6,6,0,0,1,0,12H128a74,74,0,0,0,0,148h72A6,6,0,0,1,206,208Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,208a8,8,0,0,1-8,8H128a88,88,0,0,1,0-176h72a8,8,0,0,1,0,16H128a72,72,0,0,0,0,144h72A8,8,0,0,1,208,208Z"></path>
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
            id=move || id.get().map(|id| id.get())
            class=move || class.get().map(|cls| cls.get())
        >
            {body}
        </svg>
    }
}
