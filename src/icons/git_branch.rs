//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn GitBranch(
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
            IconWeight::Bold => view!{ <path d="M228,64a36,36,0,1,0-48,33.94V104a12,12,0,0,1-12,12H96a35.88,35.88,0,0,0-12,2.06V97.94a36,36,0,1,0-24,0v60.12a36,36,0,1,0,24,0V152a12,12,0,0,1,12-12h72a36,36,0,0,0,36-36V97.94A36.07,36.07,0,0,0,228,64ZM72,52A12,12,0,1,1,60,64,12,12,0,0,1,72,52Zm0,152a12,12,0,1,1,12-12A12,12,0,0,1,72,204ZM192,76a12,12,0,1,1,12-12A12,12,0,0,1,192,76Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,64a24,24,0,1,1-24-24A24,24,0,0,1,216,64Z" opacity="0.2"/><path d="M224,64a32,32,0,1,0-40,31v9a16,16,0,0,1-16,16H96a31.71,31.71,0,0,0-16,4.31V95a32,32,0,1,0-16,0v66a32,32,0,1,0,16,0v-9a16,16,0,0,1,16-16h72a32,32,0,0,0,32-32V95A32.06,32.06,0,0,0,224,64ZM56,64A16,16,0,1,1,72,80,16,16,0,0,1,56,64ZM88,192a16,16,0,1,1-16-16A16,16,0,0,1,88,192ZM192,80a16,16,0,1,1,16-16A16,16,0,0,1,192,80Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M224,64a32,32,0,1,0-40,31v9a16,16,0,0,1-16,16H96a31.71,31.71,0,0,0-16,4.31V95a32,32,0,1,0-16,0v66a32,32,0,1,0,16,0v-9a16,16,0,0,1,16-16h72a32,32,0,0,0,32-32V95A32.06,32.06,0,0,0,224,64ZM56,64A16,16,0,1,1,72,80,16,16,0,0,1,56,64ZM88,192a16,16,0,1,1-16-16A16,16,0,0,1,88,192Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M222,64a30,30,0,1,0-36,29.4V104a18,18,0,0,1-18,18H96a29.82,29.82,0,0,0-18,6V93.4a30,30,0,1,0-12,0v69.2a30,30,0,1,0,12,0V152a18,18,0,0,1,18-18h72a30,30,0,0,0,30-30V93.4A30.05,30.05,0,0,0,222,64ZM54,64A18,18,0,1,1,72,82,18,18,0,0,1,54,64ZM90,192a18,18,0,1,1-18-18A18,18,0,0,1,90,192ZM192,82a18,18,0,1,1,18-18A18,18,0,0,1,192,82Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M224,64a32,32,0,1,0-40,31v9a16,16,0,0,1-16,16H96a31.71,31.71,0,0,0-16,4.31V95a32,32,0,1,0-16,0v66a32,32,0,1,0,16,0v-9a16,16,0,0,1,16-16h72a32,32,0,0,0,32-32V95A32.06,32.06,0,0,0,224,64ZM56,64A16,16,0,1,1,72,80,16,16,0,0,1,56,64ZM88,192a16,16,0,1,1-16-16A16,16,0,0,1,88,192ZM192,80a16,16,0,1,1,16-16A16,16,0,0,1,192,80Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M220,64a28,28,0,1,0-32,27.71V104a20,20,0,0,1-20,20H96a27.9,27.9,0,0,0-20,8.43V91.71a28,28,0,1,0-8,0v72.58a28,28,0,1,0,8,0V152a20,20,0,0,1,20-20h72a28,28,0,0,0,28-28V91.71A28,28,0,0,0,220,64ZM52,64A20,20,0,1,1,72,84,20,20,0,0,1,52,64ZM92,192a20,20,0,1,1-20-20A20,20,0,0,1,92,192ZM192,84a20,20,0,1,1,20-20A20,20,0,0,1,192,84Z"/> }.into_view()
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
