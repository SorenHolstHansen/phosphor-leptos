//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance"))]
#[component]
pub fn NumberFour(
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
                <path d="M104.65,144,144,94.81V144ZM216,40V216a16,16,0,0,1-16,16H56a16,16,0,0,1-16-16V40A16,16,0,0,1,56,24H200A16,16,0,0,1,216,40ZM184,152a8,8,0,0,0-8-8H160V72a8,8,0,0,0-14.25-5l-64,80A8,8,0,0,0,88,160h56v24a8,8,0,0,0,16,0V160h16A8,8,0,0,0,184,152Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,40V216a16,16,0,0,1-16,16H56a16,16,0,0,1-16-16V40A16,16,0,0,1,56,24H200A16,16,0,0,1,216,40Z"
        opacity="0.2"
    ></path>
    <path d="M184,160H168V48a8,8,0,0,0-14.25-5l-96,120A8,8,0,0,0,64,176h88v32a8,8,0,0,0,16,0V176h16a8,8,0,0,0,0-16Zm-32,0H80.64L152,70.81Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M184,164H164V48a4,4,0,0,0-7.12-2.5l-96,120A4,4,0,0,0,64,172h92v36a4,4,0,0,0,8,0V172h20a4,4,0,0,0,0-8Zm-28,0H72.32L156,59.4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M184,156H172V48a12,12,0,0,0-21.37-7.5l-96,120A12,12,0,0,0,64,180h84v28a12,12,0,0,0,24,0V180h12a12,12,0,0,0,0-24Zm-36,0H89l59-73.79Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M184,162H166V48a6,6,0,0,0-10.69-3.75l-96,120A6,6,0,0,0,64,174h90v34a6,6,0,0,0,12,0V174h18a6,6,0,0,0,0-12Zm-30,0H76.48L154,65.1Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M184,160H168V48a8,8,0,0,0-14.25-5l-96,120A8,8,0,0,0,64,176h88v32a8,8,0,0,0,16,0V176h16a8,8,0,0,0,0-16Zm-32,0H80.64L152,70.81Z"></path>
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
