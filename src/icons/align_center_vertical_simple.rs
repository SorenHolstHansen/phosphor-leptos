//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn AlignCenterVerticalSimple(
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
            IconWeight::Bold => view!{ <path d="M208,116H180V48a20,20,0,0,0-20-20H96A20,20,0,0,0,76,48v68H48a12,12,0,0,0,0,24H76v68a20,20,0,0,0,20,20h64a20,20,0,0,0,20-20V140h28a12,12,0,0,0,0-24Zm-52,88H100V52h56Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M168,48V208a8,8,0,0,1-8,8H96a8,8,0,0,1-8-8V48a8,8,0,0,1,8-8h64A8,8,0,0,1,168,48Z" opacity="0.2"/><path d="M208,120H176V48a16,16,0,0,0-16-16H96A16,16,0,0,0,80,48v72H48a8,8,0,0,0,0,16H80v72a16,16,0,0,0,16,16h64a16,16,0,0,0,16-16V136h32a8,8,0,0,0,0-16Zm-48,88H96V48h64Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M216,128a8,8,0,0,1-8,8H176v72a16,16,0,0,1-16,16H96a16,16,0,0,1-16-16V136H48a8,8,0,0,1,0-16H80V48A16,16,0,0,1,96,32h64a16,16,0,0,1,16,16v72h32A8,8,0,0,1,216,128Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M208,122H174V48a14,14,0,0,0-14-14H96A14,14,0,0,0,82,48v74H48a6,6,0,0,0,0,12H82v74a14,14,0,0,0,14,14h64a14,14,0,0,0,14-14V134h34a6,6,0,0,0,0-12Zm-46,86a2,2,0,0,1-2,2H96a2,2,0,0,1-2-2V48a2,2,0,0,1,2-2h64a2,2,0,0,1,2,2Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,120H176V48a16,16,0,0,0-16-16H96A16,16,0,0,0,80,48v72H48a8,8,0,0,0,0,16H80v72a16,16,0,0,0,16,16h64a16,16,0,0,0,16-16V136h32a8,8,0,0,0,0-16Zm-48,88H96V48h64Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M208,124H172V48a12,12,0,0,0-12-12H96A12,12,0,0,0,84,48v76H48a4,4,0,0,0,0,8H84v76a12,12,0,0,0,12,12h64a12,12,0,0,0,12-12V132h36a4,4,0,0,0,0-8Zm-44,84a4,4,0,0,1-4,4H96a4,4,0,0,1-4-4V48a4,4,0,0,1,4-4h64a4,4,0,0,1,4,4Z"/> }.into_view()
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
