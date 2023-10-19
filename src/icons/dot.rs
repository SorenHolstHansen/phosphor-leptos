/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Dot(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M128,80a48,48,0,1,0,48,48A48,48,0,0,0,128,80Zm0,60a12,12,0,1,1,12-12A12,12,0,0,1,128,140Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M176,128a48,48,0,1,1-48-48A48,48,0,0,1,176,128Z" opacity="0.2"/><path d="M140,128a12,12,0,1,1-12-12A12,12,0,0,1,140,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M136,128a8,8,0,1,1-8-8A8,8,0,0,1,136,128Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M144,128a16,16,0,1,1-16-16A16,16,0,0,1,144,128Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M138,128a10,10,0,1,1-10-10A10,10,0,0,1,138,128Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M140,128a12,12,0,1,1-12-12A12,12,0,0,1,140,128Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg 
            xmlns="http://www.w3.org/2000/svg" 
            width=size.clone()
            height=size
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}