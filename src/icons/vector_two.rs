//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "arrows", feature = "development", feature = "design"))]
#[component]
pub fn VectorTwo(
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
            IconWeight::Fill => view! {
                <path d="M229.66,197.66l-32,32A8,8,0,0,1,184,224V200H80a8,8,0,0,1-8-8V80H48a8,8,0,0,1-5.66-13.66l32-32a8,8,0,0,1,11.32,0l32,32A8,8,0,0,1,112,80H88V184h96V160a8,8,0,0,1,13.66-5.66l32,32A8,8,0,0,1,229.66,197.66Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,56V192H80V40H208A16,16,0,0,1,224,56Z" opacity="0.2"></path>
    <path d="M229.66,197.66l-32,32a8,8,0,0,1-11.32-11.32L204.69,200H80a8,8,0,0,1-8-8V59.31L53.66,77.66A8,8,0,0,1,42.34,66.34l32-32a8,8,0,0,1,11.32,0l32,32a8,8,0,0,1-11.32,11.32L88,59.31V184H204.69l-18.35-18.34a8,8,0,0,1,11.32-11.32l32,32A8,8,0,0,1,229.66,197.66Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M226.83,194.83l-32,32a4,4,0,0,1-5.66-5.66L214.34,196H80a4,4,0,0,1-4-4V49.66L50.83,74.83a4,4,0,0,1-5.66-5.66l32-32a4,4,0,0,1,5.66,0l32,32a4,4,0,0,1-5.66,5.66L84,49.66V188H214.34l-25.17-25.17a4,4,0,0,1,5.66-5.66l32,32A4,4,0,0,1,226.83,194.83Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M232.49,200.49l-32,32a12,12,0,0,1-17-17L195,204H80a12,12,0,0,1-12-12V69L56.49,80.49a12,12,0,1,1-17-17l32-32a12,12,0,0,1,17,0l32,32a12,12,0,0,1-17,17L92,69V180H195l-11.52-11.51a12,12,0,0,1,17-17l32,32A12,12,0,0,1,232.49,200.49Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M228.24,196.24l-32,32a6,6,0,0,1-8.48-8.48L209.51,198H80a6,6,0,0,1-6-6V54.49L52.24,76.24a6,6,0,0,1-8.48-8.48l32-32a6,6,0,0,1,8.48,0l32,32a6,6,0,1,1-8.48,8.48L86,54.49V186H209.51l-21.75-21.76a6,6,0,0,1,8.48-8.48l32,32A6,6,0,0,1,228.24,196.24Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M229.66,197.66l-32,32a8,8,0,0,1-11.32-11.32L204.69,200H80a8,8,0,0,1-8-8V59.31L53.66,77.66A8,8,0,0,1,42.34,66.34l32-32a8,8,0,0,1,11.32,0l32,32a8,8,0,0,1-11.32,11.32L88,59.31V184H204.69l-18.35-18.34a8,8,0,0,1,11.32-11.32l32,32A8,8,0,0,1,229.66,197.66Z"></path>
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
            id=move || id.get().map(|id| id.get())
            class=move || class.get().map(|cls| cls.get())
        >
            {body}
        </svg>
    }
}
