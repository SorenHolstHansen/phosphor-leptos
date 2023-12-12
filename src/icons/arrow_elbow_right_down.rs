use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn ArrowElbowRightDown(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M232.49,168.49l-48,48a12,12,0,0,1-17,0l-48-48a12,12,0,1,1,17-17L164,179V76H32a12,12,0,0,1,0-24H176a12,12,0,0,1,12,12V179l27.51-27.52a12,12,0,0,1,17,17Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,160l-48,48-48-48Z" opacity="0.2"/><path d="M231.39,156.94A8,8,0,0,0,224,152H184V64a8,8,0,0,0-8-8H32a8,8,0,0,0,0,16H168v80H128a8,8,0,0,0-5.66,13.66l48,48a8,8,0,0,0,11.32,0l48-48A8,8,0,0,0,231.39,156.94ZM176,196.69,147.31,168h57.38Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M229.66,165.66l-48,48a8,8,0,0,1-11.32,0l-48-48A8,8,0,0,1,128,152h40V72H32a8,8,0,0,1,0-16H176a8,8,0,0,1,8,8v88h40a8,8,0,0,1,5.66,13.66Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M228.24,164.24l-48,48a6,6,0,0,1-8.48,0l-48-48a6,6,0,1,1,8.48-8.48L170,193.51V70H32a6,6,0,0,1,0-12H176a6,6,0,0,1,6,6V193.51l37.76-37.75a6,6,0,0,1,8.48,8.48Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M229.66,165.66l-48,48a8,8,0,0,1-11.32,0l-48-48a8,8,0,0,1,11.32-11.32L168,188.69V72H32a8,8,0,0,1,0-16H176a8,8,0,0,1,8,8V188.69l34.34-34.35a8,8,0,0,1,11.32,11.32Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M226.83,162.83l-48,48a4,4,0,0,1-5.66,0l-48-48a4,4,0,1,1,5.66-5.66L172,198.34V68H32a4,4,0,0,1,0-8H176a4,4,0,0,1,4,4V198.34l41.17-41.17a4,4,0,0,1,5.66,5.66Z"/> }.into_view()
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
