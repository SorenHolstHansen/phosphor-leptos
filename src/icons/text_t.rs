//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn TextT(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M212,56V88a12,12,0,0,1-24,0V68H140V188h20a12,12,0,0,1,0,24H96a12,12,0,0,1,0-24h20V68H68V88a12,12,0,0,1-24,0V56A12,12,0,0,1,56,44H200A12,12,0,0,1,212,56Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M200,56V192a8,8,0,0,1-8,8H64a8,8,0,0,1-8-8V56Z" opacity="0.2"/><path d="M208,56V88a8,8,0,0,1-16,0V64H136V192h24a8,8,0,0,1,0,16H96a8,8,0,0,1,0-16h24V64H64V88a8,8,0,0,1-16,0V56a8,8,0,0,1,8-8H200A8,8,0,0,1,208,56Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM184,96a8,8,0,0,1-16,0V88H136v88h16a8,8,0,0,1,0,16H104a8,8,0,0,1,0-16h16V88H88v8a8,8,0,0,1-16,0V80a8,8,0,0,1,8-8h96a8,8,0,0,1,8,8Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M206,56V88a6,6,0,0,1-12,0V62H134V194h26a6,6,0,0,1,0,12H96a6,6,0,0,1,0-12h26V62H62V88a6,6,0,0,1-12,0V56a6,6,0,0,1,6-6H200A6,6,0,0,1,206,56Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,56V88a8,8,0,0,1-16,0V64H136V192h24a8,8,0,0,1,0,16H96a8,8,0,0,1,0-16h24V64H64V88a8,8,0,0,1-16,0V56a8,8,0,0,1,8-8H200A8,8,0,0,1,208,56Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M204,56V88a4,4,0,0,1-8,0V60H132V196h28a4,4,0,0,1,0,8H96a4,4,0,0,1,0-8h28V60H60V88a4,4,0,0,1-8,0V56a4,4,0,0,1,4-4H200A4,4,0,0,1,204,56Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
