use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn CurrencyRub(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M148,156a64,64,0,0,0,0-128H88A12,12,0,0,0,76,40v92H56a12,12,0,0,0,0,24H76v16H56a12,12,0,0,0,0,24H76v20a12,12,0,0,0,24,0V196h44a12,12,0,0,0,0-24H100V156ZM100,52h48a40,40,0,0,1,0,80H100Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M200,92a52,52,0,0,1-52,52H88V40h60A52,52,0,0,1,200,92Z" opacity="0.2"/><path d="M148,152a60,60,0,0,0,0-120H88a8,8,0,0,0-8,8v96H56a8,8,0,0,0,0,16H80v16H56a8,8,0,0,0,0,16H80v32a8,8,0,0,0,16,0V184h48a8,8,0,0,0,0-16H96V152ZM96,48h52a44,44,0,0,1,0,88H96Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M176,108a28,28,0,0,1-28,28H112V80h36A28,28,0,0,1,176,108Zm56,20A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-40-20a44.05,44.05,0,0,0-44-44H104a8,8,0,0,0-8,8v64H80a8,8,0,0,0,0,16H96v16H80a8,8,0,0,0,0,16H96v16a8,8,0,0,0,16,0V184h40a8,8,0,0,0,0-16H112V152h36A44.05,44.05,0,0,0,192,108Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M148,150a58,58,0,0,0,0-116H88a6,6,0,0,0-6,6v98H56a6,6,0,0,0,0,12H82v20H56a6,6,0,0,0,0,12H82v34a6,6,0,0,0,12,0V182h50a6,6,0,0,0,0-12H94V150ZM94,46h54a46,46,0,0,1,0,92H94Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M148,152a60,60,0,0,0,0-120H88a8,8,0,0,0-8,8v96H56a8,8,0,0,0,0,16H80v16H56a8,8,0,0,0,0,16H80v32a8,8,0,0,0,16,0V184h48a8,8,0,0,0,0-16H96V152ZM96,48h52a44,44,0,0,1,0,88H96Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M148,36H88a4,4,0,0,0-4,4V140H56a4,4,0,0,0,0,8H84v24H56a4,4,0,0,0,0,8H84v36a4,4,0,0,0,8,0V180h52a4,4,0,0,0,0-8H92V148h56a56,56,0,0,0,0-112Zm0,104H92V44h56a48,48,0,0,1,0,96Z"/> }.into_view()
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
