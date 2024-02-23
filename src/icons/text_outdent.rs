//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor"))]
#[component]
pub fn TextOutdent(
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
                <path d="M224,128a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16H216A8,8,0,0,1,224,128ZM112,72H216a8,8,0,0,0,0-16H112a8,8,0,0,0,0,16ZM216,184H40a8,8,0,0,0,0,16H216a8,8,0,0,0,0-16ZM72,144a8,8,0,0,0,8-8V56a8,8,0,0,0-13.66-5.66l-40,40a8,8,0,0,0,0,11.32l40,40A8,8,0,0,0,72,144Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M72,56v80L32,96Z" opacity="0.2"></path>
    <path d="M224,128a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16H216A8,8,0,0,1,224,128ZM112,72H216a8,8,0,0,0,0-16H112a8,8,0,0,0,0,16ZM216,184H40a8,8,0,0,0,0,16H216a8,8,0,0,0,0-16ZM66.34,141.66l-40-40a8,8,0,0,1,0-11.32l40-40A8,8,0,0,1,80,56v80a8,8,0,0,1-13.66,5.66ZM64,75.31,43.31,96,64,116.69Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M220,128a4,4,0,0,1-4,4H112a4,4,0,0,1,0-8H216A4,4,0,0,1,220,128ZM112,68H216a4,4,0,0,0,0-8H112a4,4,0,0,0,0,8ZM216,188H40a4,4,0,0,0,0,8H216a4,4,0,0,0,0-8ZM72,140a4,4,0,0,0,2.83-6.83L37.66,96,74.83,58.83a4,4,0,0,0-5.66-5.66l-40,40a4,4,0,0,0,0,5.66l40,40A4,4,0,0,0,72,140Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M228,128a12,12,0,0,1-12,12H120a12,12,0,0,1,0-24h96A12,12,0,0,1,228,128ZM120,76h96a12,12,0,0,0,0-24H120a12,12,0,0,0,0,24Zm96,104H40a12,12,0,0,0,0,24H216a12,12,0,0,0,0-24ZM72,148a12,12,0,0,0,8.49-20.49L49,96,80.49,64.48a12,12,0,0,0-17-17l-40,40a12,12,0,0,0,0,17l40,40A12,12,0,0,0,72,148Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M222,128a6,6,0,0,1-6,6H112a6,6,0,0,1,0-12H216A6,6,0,0,1,222,128ZM112,70H216a6,6,0,0,0,0-12H112a6,6,0,0,0,0,12ZM216,186H40a6,6,0,0,0,0,12H216a6,6,0,0,0,0-12ZM72,142a6,6,0,0,0,4.24-10.24L40.49,96,76.24,60.24a6,6,0,0,0-8.48-8.48l-40,40a6,6,0,0,0,0,8.48l40,40A6,6,0,0,0,72,142Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,128a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16H216A8,8,0,0,1,224,128ZM112,72H216a8,8,0,0,0,0-16H112a8,8,0,0,0,0,16ZM216,184H40a8,8,0,0,0,0,16H216a8,8,0,0,0,0-16ZM72,144a8,8,0,0,0,5.66-13.66L43.31,96,77.66,61.66A8,8,0,0,0,66.34,50.34l-40,40a8,8,0,0,0,0,11.32l40,40A8,8,0,0,0,72,144Z"></path>
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
