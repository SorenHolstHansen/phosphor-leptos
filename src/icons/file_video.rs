//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn FileVideo(
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
                <path d="M150.35,149.82a12,12,0,0,0-11.63-.6L118,159.37A20,20,0,0,0,100,148H48a20,20,0,0,0-20,20v40a20,20,0,0,0,20,20h52a20,20,0,0,0,18.3-12l20.12,10.58A12,12,0,0,0,156,216V160A12,12,0,0,0,150.35,149.82ZM96,204H52V172H96Zm36-7.87-12-6.3v-4.72l12-5.87ZM216.49,79.51l-56-56A12,12,0,0,0,152,20H56A20,20,0,0,0,36,40v76a12,12,0,0,0,24,0V44h76V92a12,12,0,0,0,12,12h48V212h-8a12,12,0,0,0,0,24h12a20,20,0,0,0,20-20V88A12,12,0,0,0,216.49,79.51ZM160,57l23,23H160Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M112,175.67,144,160v56l-32-16.82V208a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V168a8,8,0,0,1,8-8h56a8,8,0,0,1,8,8ZM152,32V88h56Z"
        opacity="0.2"
    ></path>
    <path d="M148.23,153.21a8,8,0,0,0-7.75-.39l-21.22,10.39A16,16,0,0,0,104,152H48a16,16,0,0,0-16,16v40a16,16,0,0,0,16,16h56a16,16,0,0,0,15.44-11.87l20.84,11A8,8,0,0,0,152,216V160A8,8,0,0,0,148.23,153.21ZM104,208H48V168h56v31c0,.13,0,.25,0,.38V208Zm32-5.24-16-8.42V180.66l16-7.83ZM213.66,82.34l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v88a8,8,0,0,0,16,0V40h88V88a8,8,0,0,0,8,8h48V216H176a8,8,0,0,0,0,16h24a16,16,0,0,0,16-16V88A8,8,0,0,0,213.66,82.34ZM160,51.31,188.69,80H160Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M148.23,153.21a8,8,0,0,0-7.75-.39l-21.22,10.39A16,16,0,0,0,104,152H48a16,16,0,0,0-16,16v40a16,16,0,0,0,16,16h56a16,16,0,0,0,15.44-11.87l20.84,11A8,8,0,0,0,152,216V160A8,8,0,0,0,148.23,153.21ZM136,202.76l-16-8.42V180.66l16-7.83ZM213.66,82.34l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v88a8,8,0,0,0,16,0V40h88V88a8,8,0,0,0,8,8h48V216H176a8,8,0,0,0,0,16h24a16,16,0,0,0,16-16V88A8,8,0,0,0,213.66,82.34ZM160,51.31,188.69,80H160Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M147.18,154.91a6,6,0,0,0-5.82-.3l-23.5,11.51A14,14,0,0,0,104,154H48a14,14,0,0,0-14,14v40a14,14,0,0,0,14,14h56a14,14,0,0,0,14-12.92l23.26,12.23A6,6,0,0,0,150,216V160A6,6,0,0,0,147.18,154.91ZM104,210H48a2,2,0,0,1-2-2V168a2,2,0,0,1,2-2h56a2,2,0,0,1,2,2v40A2,2,0,0,1,104,210Zm34-3.93-20-10.52V179.41l20-9.79ZM212.24,83.76l-56-56A6,6,0,0,0,152,26H56A14,14,0,0,0,42,40v88a6,6,0,0,0,12,0V40a2,2,0,0,1,2-2h90V88a6,6,0,0,0,6,6h50V216a2,2,0,0,1-2,2H176a6,6,0,0,0,0,12h24a14,14,0,0,0,14-14V88A6,6,0,0,0,212.24,83.76ZM158,46.48,193.52,82H158Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M148.23,153.21a8,8,0,0,0-7.75-.39l-21.22,10.39A16,16,0,0,0,104,152H48a16,16,0,0,0-16,16v40a16,16,0,0,0,16,16h56a16,16,0,0,0,15.44-11.87l20.84,11A8,8,0,0,0,152,216V160A8,8,0,0,0,148.23,153.21ZM104,208H48V168h56v31c0,.13,0,.25,0,.38V208Zm32-5.24-16-8.42V180.66l16-7.83ZM213.66,82.34l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v88a8,8,0,0,0,16,0V40h88V88a8,8,0,0,0,8,8h48V216H176a8,8,0,0,0,0,16h24a16,16,0,0,0,16-16V88A8,8,0,0,0,213.66,82.34ZM160,51.31,188.69,80H160Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M146.12,156.61a4,4,0,0,0-3.88-.2L116,169.26V168a12,12,0,0,0-12-12H48a12,12,0,0,0-12,12v40a12,12,0,0,0,12,12h56a12,12,0,0,0,12-12v-2.2l26.14,13.74a4,4,0,0,0,3.93-.12A4,4,0,0,0,148,216V160A4,4,0,0,0,146.12,156.61ZM108,208a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V168a4,4,0,0,1,4-4h56a4,4,0,0,1,4,4Zm32,1.38-24-12.62V178.17l24-11.76ZM210.83,85.17l-56-56A4,4,0,0,0,152,28H56A12,12,0,0,0,44,40v88a4,4,0,0,0,8,0V40a4,4,0,0,1,4-4h92V88a4,4,0,0,0,4,4h52V216a4,4,0,0,1-4,4H176a4,4,0,0,0,0,8h24a12,12,0,0,0,12-12V88A4,4,0,0,0,210.83,85.17ZM156,41.65,198.34,84H156Z"></path>
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
