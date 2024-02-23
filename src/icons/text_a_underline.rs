//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor"))]
#[component]
pub fn TextAUnderline(
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
                <path d="M58.89,178.86a12,12,0,0,0,16-5.75L90.44,140h75.12l15.58,33.11a12,12,0,0,0,21.72-10.22l-64-136a12,12,0,0,0-21.72,0l-64,136A12,12,0,0,0,58.89,178.86ZM128,60.18,154.27,116H101.73ZM228,216a12,12,0,0,1-12,12H40a12,12,0,0,1,0-24H216A12,12,0,0,1,228,216Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M173.18,128H82.82L128,32Z" opacity="0.2"></path>
    <path d="M60.59,175.24a8,8,0,0,0,10.65-3.83L87.9,136h80.2l16.66,35.41a8,8,0,1,0,14.48-6.82l-64-136a8,8,0,0,0-14.48,0l-64,136A8,8,0,0,0,60.59,175.24ZM128,50.79,160.57,120H95.43ZM224,216a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,216Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M148.73,120H107.27L128,75.09ZM216,32V224a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V32a8,8,0,0,1,8-8H208A8,8,0,0,1,216,32ZM76.65,167.26a8,8,0,0,0,10.61-3.91L99.89,136h56.22l12.63,27.35a8,8,0,0,0,14.52-6.7l-48-104a8,8,0,0,0-14.52,0l-48,104A8,8,0,0,0,76.65,167.26ZM200,192a8,8,0,0,0-8-8H64a8,8,0,0,0,0,16H192A8,8,0,0,0,200,192Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M61.45,173.43a6,6,0,0,0,8-2.88L86.63,134h82.74l17.2,36.55A6,6,0,0,0,192,174a5.92,5.92,0,0,0,2.55-.57,6,6,0,0,0,2.88-8l-64-136a6,6,0,0,0-10.86,0l-64,136A6,6,0,0,0,61.45,173.43ZM128,46.09,163.72,122H92.28ZM222,216a6,6,0,0,1-6,6H40a6,6,0,0,1,0-12H216A6,6,0,0,1,222,216Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M60.59,175.24a8,8,0,0,0,10.65-3.83L87.9,136h80.2l16.66,35.41a8,8,0,1,0,14.48-6.82l-64-136a8,8,0,0,0-14.48,0l-64,136A8,8,0,0,0,60.59,175.24ZM128,50.79,160.57,120H95.43ZM224,216a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M62.3,171.62a4,4,0,0,0,5.32-1.92L85.36,132h85.28l17.74,37.7a4,4,0,1,0,7.24-3.4l-64-136a4,4,0,0,0-7.24,0l-64,136A4,4,0,0,0,62.3,171.62ZM128,41.39,166.87,124H89.13ZM220,216a4,4,0,0,1-4,4H40a4,4,0,0,1,0-8H216A4,4,0,0,1,220,216Z"></path>
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
