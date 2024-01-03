//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Parallelogram(
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
                <path d="M248.78,45.14A19.92,19.92,0,0,0,232,36H88.81A20,20,0,0,0,70.57,47.79l-64.8,144A20,20,0,0,0,24,220H167.19a20,20,0,0,0,18.24-11.79l64.8-144A19.9,19.9,0,0,0,248.78,45.14ZM164.6,196H30.2L91.4,60H225.8Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M239.29,59.28l-64.8,144a8,8,0,0,1-7.3,4.72H24a8,8,0,0,1-7.3-11.28l64.8-144A8,8,0,0,1,88.81,48H232A8,8,0,0,1,239.29,59.28Z"
        opacity="0.2"
    ></path>
    <path d="M245.43,47.31A15.94,15.94,0,0,0,232,40H88.81a16,16,0,0,0-14.59,9.43l-64.8,144A16,16,0,0,0,24,216H167.19a16,16,0,0,0,14.59-9.43l64.8-144A16,16,0,0,0,245.43,47.31ZM167.19,200H24L88.81,56H232Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M246.58,62.57l-64.8,144A16,16,0,0,1,167.19,216H24A16,16,0,0,1,9.42,193.43l64.8-144A16,16,0,0,1,88.81,40H232a16,16,0,0,1,14.59,22.57Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M243.75,48.4A14,14,0,0,0,232,42H88.81A14,14,0,0,0,76,50.25l-64.8,144A14,14,0,0,0,24,214H167.19A14,14,0,0,0,180,205.75l64.8-144A14,14,0,0,0,243.75,48.4Zm-9.93,8.42-64.8,144a2,2,0,0,1-1.83,1.18H24a2,2,0,0,1-1.83-2.82L87,55.18A2,2,0,0,1,88.81,54H232a2,2,0,0,1,1.83,2.82Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M245.43,47.31A15.94,15.94,0,0,0,232,40H88.81a16,16,0,0,0-14.59,9.43l-64.8,144A16,16,0,0,0,24,216H167.19a16,16,0,0,0,14.59-9.43l64.8-144A16,16,0,0,0,245.43,47.31ZM167.19,200H24L88.81,56H232Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M242.07,49.48A12,12,0,0,0,232,44H88.81a12,12,0,0,0-11,7.08l-64.8,144A12,12,0,0,0,24,212H167.19a12,12,0,0,0,10.95-7.08l64.8-144A12,12,0,0,0,242.07,49.48Zm-6.43,8.16-64.8,144a4,4,0,0,1-3.65,2.36H24a4,4,0,0,1-3.65-5.64l64.8-144A4,4,0,0,1,88.81,52H232a4,4,0,0,1,3.65,5.64Z"></path>
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
