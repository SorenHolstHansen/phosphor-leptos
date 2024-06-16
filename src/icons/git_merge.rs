//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "development"))]
#[component]
pub fn GitMerge(
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
                <path d="M208,112a32.05,32.05,0,0,0-30.69,23l-42.21-6a8,8,0,0,1-4.95-2.71L94.43,84.55A32,32,0,1,0,72,87v82a32,32,0,1,0,16,0V101.63l30,35a24,24,0,0,0,14.83,8.14l44,6.28A32,32,0,1,0,208,112ZM96,200a16,16,0,1,1-16-16A16,16,0,0,1,96,200Zm112-40a16,16,0,1,1,16-16A16,16,0,0,1,208,160Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M104,56A24,24,0,1,1,80,32,24,24,0,0,1,104,56Z" opacity="0.2"></path>
    <path d="M208,112a32.05,32.05,0,0,0-30.69,23l-42.21-6a8,8,0,0,1-4.95-2.71L94.43,84.55A32,32,0,1,0,72,87v82a32,32,0,1,0,16,0V101.63l30,35a24,24,0,0,0,14.83,8.14l44,6.28A32,32,0,1,0,208,112ZM64,56A16,16,0,1,1,80,72,16,16,0,0,1,64,56ZM96,200a16,16,0,1,1-16-16A16,16,0,0,1,96,200Zm112-40a16,16,0,1,1,16-16A16,16,0,0,1,208,160Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,116a28,28,0,0,0-27.62,23.44l-45.85-6.55a12,12,0,0,1-7.41-4.07L87.75,82.9A28,28,0,1,0,76,83.71v88.58a28,28,0,1,0,8,0V90.81L121,134a20,20,0,0,0,12.36,6.78l46.83,6.69A28,28,0,1,0,208,116ZM60,56A20,20,0,1,1,80,76,20,20,0,0,1,60,56Zm40,144a20,20,0,1,1-20-20A20,20,0,0,1,100,200Zm108-36a20,20,0,1,1,20-20A20,20,0,0,1,208,164Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M208,108a36.05,36.05,0,0,0-33.38,22.54l-39-5.57a4,4,0,0,1-2.47-1.35L100.55,85.53A36,36,0,1,0,68,89.94v76.12a36,36,0,1,0,24,0V112.44l23,26.8a28,28,0,0,0,17.3,9.49l41.34,5.91A36,36,0,1,0,208,108ZM80,44A12,12,0,1,1,68,56,12,12,0,0,1,80,44Zm0,168a12,12,0,1,1,12-12A12,12,0,0,1,80,212Zm128-56a12,12,0,1,1,12-12A12,12,0,0,1,208,156Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,114a30,30,0,0,0-29.21,23.19l-44-6.28a10,10,0,0,1-6.18-3.39L91.18,83.83A30,30,0,1,0,74,85.4v85.2a30,30,0,1,0,12,0V96.22l33.52,39.11a22,22,0,0,0,13.6,7.46l45.35,6.48A30,30,0,1,0,208,114ZM62,56A18,18,0,1,1,80,74,18,18,0,0,1,62,56ZM98,200a18,18,0,1,1-18-18A18,18,0,0,1,98,200Zm110-38a18,18,0,1,1,18-18A18,18,0,0,1,208,162Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,112a32.05,32.05,0,0,0-30.69,23l-42.21-6a8,8,0,0,1-4.95-2.71L94.43,84.55A32,32,0,1,0,72,87v82a32,32,0,1,0,16,0V101.63l30,35a24,24,0,0,0,14.83,8.14l44,6.28A32,32,0,1,0,208,112ZM64,56A16,16,0,1,1,80,72,16,16,0,0,1,64,56ZM96,200a16,16,0,1,1-16-16A16,16,0,0,1,96,200Zm112-40a16,16,0,1,1,16-16A16,16,0,0,1,208,160Z"></path>
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
