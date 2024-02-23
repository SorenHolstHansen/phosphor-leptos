//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance", feature = "office"))]
#[component]
pub fn TrendDown(
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
                <path d="M240,136v64a8,8,0,0,1-8,8H168a8,8,0,0,1-5.66-13.66L188.69,168,136,115.31l-34.34,34.35a8,8,0,0,1-11.32,0l-72-72A8,8,0,0,1,29.66,66.34L96,132.69l34.34-34.35a8,8,0,0,1,11.32,0L200,156.69l26.34-26.35A8,8,0,0,1,240,136Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M232,136v64H168Z" opacity="0.2"></path>
    <path d="M235.06,128.61a8,8,0,0,0-8.72,1.73L200,156.69,141.66,98.34a8,8,0,0,0-11.32,0L96,132.69,29.66,66.34A8,8,0,0,0,18.34,77.66l72,72a8,8,0,0,0,11.32,0L136,115.31,188.69,168l-26.35,26.34A8,8,0,0,0,168,208h64a8,8,0,0,0,8-8V136A8,8,0,0,0,235.06,128.61ZM224,192H187.31L224,155.31Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M236,136v64a4,4,0,0,1-4,4H168a4,4,0,0,1,0-8h54.34L136,109.66,98.83,146.83a4,4,0,0,1-5.66,0l-72-72a4,4,0,0,1,5.66-5.66L96,138.34l37.17-37.17a4,4,0,0,1,5.66,0L228,190.34V136a4,4,0,0,1,8,0Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,136v64a12,12,0,0,1-12,12H168a12,12,0,0,1,0-24h35l-67-67-31.51,31.52a12,12,0,0,1-17,0l-72-72a12,12,0,0,1,17-17L96,127l31.51-31.52a12,12,0,0,1,17,0L220,171V136a12,12,0,0,1,24,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M238,136v64a6,6,0,0,1-6,6H168a6,6,0,0,1,0-12h49.52L136,112.49l-35.76,35.75a6,6,0,0,1-8.48,0l-72-72a6,6,0,0,1,8.48-8.48L96,135.51l35.76-35.75a6,6,0,0,1,8.48,0L226,185.52V136a6,6,0,0,1,12,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,136v64a8,8,0,0,1-8,8H168a8,8,0,0,1,0-16h44.69L136,115.31l-34.34,34.35a8,8,0,0,1-11.32,0l-72-72A8,8,0,0,1,29.66,66.34L96,132.69l34.34-34.35a8,8,0,0,1,11.32,0L224,180.69V136a8,8,0,0,1,16,0Z"></path>
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
