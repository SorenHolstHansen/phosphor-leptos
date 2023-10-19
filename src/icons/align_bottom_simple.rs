/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn AlignBottomSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M208,232a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,232ZM96,208h64a16,16,0,0,0,16-16V40a16,16,0,0,0-16-16H96A16,16,0,0,0,80,40V192A16,16,0,0,0,96,208Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M168,40V192a8,8,0,0,1-8,8H96a8,8,0,0,1-8-8V40a8,8,0,0,1,8-8h64A8,8,0,0,1,168,40Z" opacity="0.2"/><path d="M208,232a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,232ZM80,192V40A16,16,0,0,1,96,24h64a16,16,0,0,1,16,16V192a16,16,0,0,1-16,16H96A16,16,0,0,1,80,192Zm16,0h64V40H96Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M204,232a4,4,0,0,1-4,4H56a4,4,0,0,1,0-8H200A4,4,0,0,1,204,232ZM84,192V40A12,12,0,0,1,96,28h64a12,12,0,0,1,12,12V192a12,12,0,0,1-12,12H96A12,12,0,0,1,84,192Zm8,0a4,4,0,0,0,4,4h64a4,4,0,0,0,4-4V40a4,4,0,0,0-4-4H96a4,4,0,0,0-4,4Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M212,232a12,12,0,0,1-12,12H56a12,12,0,0,1,0-24H200A12,12,0,0,1,212,232ZM76,184V40A20,20,0,0,1,96,20h64a20,20,0,0,1,20,20V184a20,20,0,0,1-20,20H96A20,20,0,0,1,76,184Zm24-4h56V44H100Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M206,232a6,6,0,0,1-6,6H56a6,6,0,0,1,0-12H200A6,6,0,0,1,206,232ZM82,192V40A14,14,0,0,1,96,26h64a14,14,0,0,1,14,14V192a14,14,0,0,1-14,14H96A14,14,0,0,1,82,192Zm12,0a2,2,0,0,0,2,2h64a2,2,0,0,0,2-2V40a2,2,0,0,0-2-2H96a2,2,0,0,0-2,2Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,232a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,232ZM80,192V40A16,16,0,0,1,96,24h64a16,16,0,0,1,16,16V192a16,16,0,0,1-16,16H96A16,16,0,0,1,80,192Zm16,0h64V40H96Z"/> }.into_view()
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