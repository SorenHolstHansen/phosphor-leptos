//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn DeviceMobileCamera(
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
            IconWeight::Bold => view!{ <path d="M176,12H80A28,28,0,0,0,52,40V216a28,28,0,0,0,28,28h96a28,28,0,0,0,28-28V40A28,28,0,0,0,176,12Zm4,204a4,4,0,0,1-4,4H80a4,4,0,0,1-4-4V40a4,4,0,0,1,4-4h96a4,4,0,0,1,4,4ZM144,68a16,16,0,1,1-16-16A16,16,0,0,1,144,68Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M192,40V216a16,16,0,0,1-16,16H80a16,16,0,0,1-16-16V40A16,16,0,0,1,80,24h96A16,16,0,0,1,192,40Z" opacity="0.2"/><path d="M176,16H80A24,24,0,0,0,56,40V216a24,24,0,0,0,24,24h96a24,24,0,0,0,24-24V40A24,24,0,0,0,176,16Zm8,200a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V40a8,8,0,0,1,8-8h96a8,8,0,0,1,8,8ZM140,60a12,12,0,1,1-12-12A12,12,0,0,1,140,60Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M176,16H80A24,24,0,0,0,56,40V216a24,24,0,0,0,24,24h96a24,24,0,0,0,24-24V40A24,24,0,0,0,176,16ZM128,72a12,12,0,1,1,12-12A12,12,0,0,1,128,72Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M176,18H80A22,22,0,0,0,58,40V216a22,22,0,0,0,22,22h96a22,22,0,0,0,22-22V40A22,22,0,0,0,176,18Zm10,198a10,10,0,0,1-10,10H80a10,10,0,0,1-10-10V40A10,10,0,0,1,80,30h96a10,10,0,0,1,10,10ZM138,60a10,10,0,1,1-10-10A10,10,0,0,1,138,60Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M176,16H80A24,24,0,0,0,56,40V216a24,24,0,0,0,24,24h96a24,24,0,0,0,24-24V40A24,24,0,0,0,176,16Zm8,200a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V40a8,8,0,0,1,8-8h96a8,8,0,0,1,8,8ZM140,60a12,12,0,1,1-12-12A12,12,0,0,1,140,60Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M176,20H80A20,20,0,0,0,60,40V216a20,20,0,0,0,20,20h96a20,20,0,0,0,20-20V40A20,20,0,0,0,176,20Zm12,196a12,12,0,0,1-12,12H80a12,12,0,0,1-12-12V40A12,12,0,0,1,80,28h96a12,12,0,0,1,12,12ZM136,60a8,8,0,1,1-8-8A8,8,0,0,1,136,60Z"/> }.into_view()
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
