//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn ShieldWarning(
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
                <path d="M208,36H48A20,20,0,0,0,28,56V114.8c0,92.36,78.1,123,93.75,128.18a19.63,19.63,0,0,0,12.49,0C149.9,237.78,228,207.16,228,114.8V56A20,20,0,0,0,208,36Zm-4,78.8c0,73.55-60.52,99.52-76,105-15.47-5.42-76-31.39-76-104.95V60H204ZM116,132V96a12,12,0,0,1,24,0v36a12,12,0,0,1-24,0Zm-4,40a16,16,0,1,1,16,16A16,16,0,0,1,112,172Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,56v58.77c0,84.18-71.31,112.07-85.54,116.8a7.54,7.54,0,0,1-4.92,0C111.31,226.86,40,199,40,114.79V56a8,8,0,0,1,8-8H208A8,8,0,0,1,216,56Z"
        opacity="0.2"
    ></path>
    <path d="M208,40H48A16,16,0,0,0,32,56v58.77c0,89.62,75.82,119.34,91,124.38a15.44,15.44,0,0,0,10,0c15.2-5.05,91-34.77,91-124.39V56A16,16,0,0,0,208,40Zm0,74.79c0,78.42-66.34,104.62-80,109.18-13.53-4.5-80-30.68-80-109.18V56l160,0ZM120,136V96a8,8,0,0,1,16,0v40a8,8,0,0,1-16,0Zm-4,36a12,12,0,1,1,12,12A12,12,0,0,1,116,172Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M208,40H48A16,16,0,0,0,32,56v58.77c0,89.62,75.82,119.34,91,124.38a15.44,15.44,0,0,0,10,0c15.2-5.05,91-34.77,91-124.39V56A16,16,0,0,0,208,40ZM120,96a8,8,0,0,1,16,0v40a8,8,0,0,1-16,0Zm8,88a12,12,0,1,1,12-12A12,12,0,0,1,128,184Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,42H48A14,14,0,0,0,34,56v58.77c0,88.25,74.68,117.52,89.65,122.49a13.5,13.5,0,0,0,8.7,0c15-5,89.65-34.24,89.65-122.49V56A14,14,0,0,0,208,42Zm2,72.79c0,80-67.84,106.59-81.44,111.1a1.57,1.57,0,0,1-1.13,0C113.84,221.38,46,194.8,46,114.79V56a2,2,0,0,1,2-2H208a2,2,0,0,1,2,2ZM122,136V96a6,6,0,0,1,12,0v40a6,6,0,0,1-12,0Zm16,36a10,10,0,1,1-10-10A10,10,0,0,1,138,172Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,40H48A16,16,0,0,0,32,56v58.77c0,89.62,75.82,119.34,91,124.38a15.44,15.44,0,0,0,10,0c15.2-5.05,91-34.77,91-124.39V56A16,16,0,0,0,208,40Zm0,74.79c0,78.42-66.34,104.62-80,109.18-13.53-4.5-80-30.68-80-109.18V56l160,0ZM120,136V96a8,8,0,0,1,16,0v40a8,8,0,0,1-16,0Zm-4,36a12,12,0,1,1,12,12A12,12,0,0,1,116,172Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,44H48A12,12,0,0,0,36,56v58.77c0,86.88,73.54,115.7,88.28,120.59a11.47,11.47,0,0,0,7.44,0C146.46,230.49,220,201.67,220,114.79V56A12,12,0,0,0,208,44Zm4,70.79c0,81.38-69,108.41-82.81,113a3.51,3.51,0,0,1-2.39,0C113,223.2,44,196.17,44,114.79V56a4,4,0,0,1,4-4H208a4,4,0,0,1,4,4ZM124,136V96a4,4,0,0,1,8,0v40a4,4,0,0,1-8,0Zm12,36a8,8,0,1,1-8-8A8,8,0,0,1,136,172Z"></path>
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
