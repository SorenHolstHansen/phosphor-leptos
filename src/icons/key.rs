//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Key(
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
            IconWeight::Bold => view! {
                <path d="M160,12A84.05,84.05,0,0,0,79.38,119.65L23.51,175.51A12,12,0,0,0,20,184v40a12,12,0,0,0,12,12H72a12,12,0,0,0,12-12V212H96a12,12,0,0,0,12-12V188h12a12,12,0,0,0,8.49-3.51l7.86-7.87A84,84,0,1,0,160,12Zm0,144a59.58,59.58,0,0,1-22.1-4.2,12,12,0,0,0-13.22,2.55L115,164H96a12,12,0,0,0-12,12v12H72a12,12,0,0,0-12,12v12H44V189l57.65-57.65a12,12,0,0,0,2.55-13.21A60,60,0,1,1,160,156Zm36-80a16,16,0,1,1-16-16A16,16,0,0,1,196,76Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,96a72,72,0,0,1-98.83,66.83h0L120,176H96v24H72v24H32V184l61.17-61.17h0A72,72,0,1,1,232,96Z"
        opacity="0.2"
    ></path>
    <path d="M160,16A80.07,80.07,0,0,0,83.91,120.78L26.34,178.34A8,8,0,0,0,24,184v40a8,8,0,0,0,8,8H72a8,8,0,0,0,8-8V208H96a8,8,0,0,0,8-8V184h16a8,8,0,0,0,5.66-2.34l9.56-9.57A80,80,0,1,0,160,16Zm0,144a63.7,63.7,0,0,1-23.65-4.51,8,8,0,0,0-8.84,1.68L116.69,168H96a8,8,0,0,0-8,8v16H72a8,8,0,0,0-8,8v16H40V187.31l58.83-58.82a8,8,0,0,0,1.68-8.84A64,64,0,1,1,160,160Zm32-84a12,12,0,1,1-12-12A12,12,0,0,1,192,76Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M160,16A80.07,80.07,0,0,0,83.91,120.78L26.34,178.34A8,8,0,0,0,24,184v40a8,8,0,0,0,8,8H72a8,8,0,0,0,8-8V208H96a8,8,0,0,0,8-8V184h16a8,8,0,0,0,5.66-2.34l9.56-9.57A80,80,0,1,0,160,16Zm20,76a16,16,0,1,1,16-16A16,16,0,0,1,180,92Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M160,18A78.05,78.05,0,0,0,86.2,121.31L27.76,179.76A6,6,0,0,0,26,184v40a6,6,0,0,0,6,6H72a6,6,0,0,0,6-6V206H96a6,6,0,0,0,6-6V182h18a6,6,0,0,0,4.24-1.76l10.45-10.44A78,78,0,1,0,160,18Zm0,144a65.63,65.63,0,0,1-24.43-4.67,6,6,0,0,0-6.64,1.26L117.51,170H96a6,6,0,0,0-6,6v18H72a6,6,0,0,0-6,6v18H38V186.49l59.41-59.42a6,6,0,0,0,1.26-6.64A66,66,0,1,1,160,162Zm30-86a10,10,0,1,1-10-10A10,10,0,0,1,190,76Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M160,16A80.07,80.07,0,0,0,83.91,120.78L26.34,178.34A8,8,0,0,0,24,184v40a8,8,0,0,0,8,8H72a8,8,0,0,0,8-8V208H96a8,8,0,0,0,8-8V184h16a8,8,0,0,0,5.66-2.34l9.56-9.57A80,80,0,1,0,160,16Zm0,144a63.7,63.7,0,0,1-23.65-4.51,8,8,0,0,0-8.84,1.68L116.69,168H96a8,8,0,0,0-8,8v16H72a8,8,0,0,0-8,8v16H40V187.31l58.83-58.82a8,8,0,0,0,1.68-8.84A64,64,0,1,1,160,160Zm32-84a12,12,0,1,1-12-12A12,12,0,0,1,192,76Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M160,20A76,76,0,0,0,88.51,121.84L29.17,181.17A4,4,0,0,0,28,184v40a4,4,0,0,0,4,4H72a4,4,0,0,0,4-4V204H96a4,4,0,0,0,4-4V180h20a4,4,0,0,0,2.83-1.17l11.33-11.34A76,76,0,1,0,160,20Zm0,144a67.52,67.52,0,0,1-25.21-4.83,4,4,0,0,0-4.45.83l-12,12H96a4,4,0,0,0-4,4v20H72a4,4,0,0,0-4,4v20H36V185.66l60-60a4,4,0,0,0,.83-4.45A68,68,0,1,1,160,164Zm28-88a8,8,0,1,1-8-8A8,8,0,0,1,188,76Z"></path>
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
