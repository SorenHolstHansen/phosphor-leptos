//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="commerce", feature ="development", feature ="objects"))]
#[component]
pub fn TagSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M246.66,123.56,201,55.13A15.94,15.94,0,0,0,187.72,48H40A16,16,0,0,0,24,64V192a16,16,0,0,0,16,16H187.72A16,16,0,0,0,201,200.88h0l45.63-68.44A8,8,0,0,0,246.66,123.56Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M240,128l-45.62,68.44a8,8,0,0,1-6.66,3.56H40a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H187.72a8,8,0,0,1,6.66,3.56Z"
        opacity="0.2"
    ></path>
    <path d="M246.66,123.56,201,55.13A15.94,15.94,0,0,0,187.72,48H40A16,16,0,0,0,24,64V192a16,16,0,0,0,16,16H187.72A16,16,0,0,0,201,200.88l45.63-68.44A8,8,0,0,0,246.66,123.56ZM187.72,192H40V64H187.72l42.66,64Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M243.33,125.78,197.7,57.34a12,12,0,0,0-10-5.34H40A12,12,0,0,0,28,64V192a12,12,0,0,0,12,12H187.72a12,12,0,0,0,10-5.34l45.63-68.44A4,4,0,0,0,243.33,125.78Zm-52.28,68.44a4,4,0,0,1-3.33,1.78H40a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4H187.72a4,4,0,0,1,3.33,1.78L235.19,128Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M250,121.34,204.36,52.91A20,20,0,0,0,187.72,44H40A20,20,0,0,0,20,64V192a20,20,0,0,0,20,20H187.72a20,20,0,0,0,16.64-8.91L250,134.66A12,12,0,0,0,250,121.34ZM185.58,188H44V68H185.58l40,60Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M245,124.67,199.37,56.23A14,14,0,0,0,187.72,50H40A14,14,0,0,0,26,64V192a14,14,0,0,0,14,14H187.72a14,14,0,0,0,11.65-6.23L245,131.33A6,6,0,0,0,245,124.67Zm-55.61,68.44a2,2,0,0,1-1.66.89H40a2,2,0,0,1-2-2V64a2,2,0,0,1,2-2H187.72a2,2,0,0,1,1.66.89L232.79,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M246.66,123.56,201,55.13A15.94,15.94,0,0,0,187.72,48H40A16,16,0,0,0,24,64V192a16,16,0,0,0,16,16H187.72A16,16,0,0,0,201,200.88l45.63-68.44A8,8,0,0,0,246.66,123.56ZM187.72,192H40V64H187.72l42.66,64Z"></path>
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