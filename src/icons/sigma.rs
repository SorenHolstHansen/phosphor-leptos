//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Sigma(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M180,72V60H89l48.4,60.5a12,12,0,0,1,0,15L89,196h91V184a12,12,0,0,1,24,0v24a12,12,0,0,1-12,12H64a12,12,0,0,1-9.37-19.5l58-72.5-58-72.5A12,12,0,0,1,64,36H192a12,12,0,0,1,12,12V72a12,12,0,0,1-24,0Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M192,48V208H64l64-80L64,48Z" opacity="0.2"/><path d="M184,72V56H80.65l53.6,67a8,8,0,0,1,0,10l-53.6,67H184V184a8,8,0,0,1,16,0v24a8,8,0,0,1-8,8H64a8,8,0,0,1-6.25-13l60-75-60-75A8,8,0,0,1,64,40H192a8,8,0,0,1,8,8V72a8,8,0,0,1-16,0Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M200,24H56A16,16,0,0,0,40,40V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24ZM184,88a8,8,0,0,1-16,0V72H96l38.4,51.2a8,8,0,0,1,0,9.6L96,184h72V168a8,8,0,0,1,16,0v24a8,8,0,0,1-8,8H80a8,8,0,0,1-6.4-12.8L118,128,73.6,68.8A8,8,0,0,1,80,56h96a8,8,0,0,1,8,8Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M186,72V54H76.48l56.21,70.25a6,6,0,0,1,0,7.5L76.48,202H186V184a6,6,0,0,1,12,0v24a6,6,0,0,1-6,6H64a6,6,0,0,1-4.69-9.75l61-76.25-61-76.25A6,6,0,0,1,64,42H192a6,6,0,0,1,6,6V72a6,6,0,0,1-12,0Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M184,72V56H80.65l53.6,67a8,8,0,0,1,0,10l-53.6,67H184V184a8,8,0,0,1,16,0v24a8,8,0,0,1-8,8H64a8,8,0,0,1-6.25-13l60-75-60-75A8,8,0,0,1,64,40H192a8,8,0,0,1,8,8V72a8,8,0,0,1-16,0Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M188,72V52H72.32l58.8,73.5a4,4,0,0,1,0,5L72.32,204H188V184a4,4,0,0,1,8,0v24a4,4,0,0,1-4,4H64a4,4,0,0,1-3.12-6.5l62-77.5-62-77.5A4,4,0,0,1,64,44H192a4,4,0,0,1,4,4V72a4,4,0,0,1-8,0Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            class=class
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
