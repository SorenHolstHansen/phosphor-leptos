//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media"))]
#[component]
pub fn MusicNotes(
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
                <path d="M215.38,22.54a12,12,0,0,0-10.29-2.18l-128,32A12,12,0,0,0,68,64V167.35A40,40,0,1,0,92,204V121.37l104-26v40A40,40,0,1,0,220,172V32A12,12,0,0,0,215.38,22.54ZM52,220a16,16,0,1,1,16-16A16,16,0,0,1,52,220ZM92,96.63V73.37l104-26V70.63ZM180,188a16,16,0,1,1,16-16A16,16,0,0,1,180,188Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,172a28,28,0,1,1-28-28A28,28,0,0,1,208,172ZM52,176a28,28,0,1,0,28,28A28,28,0,0,0,52,176Z"
        opacity="0.2"
    ></path>
    <path d="M212.92,25.69a8,8,0,0,0-6.86-1.45l-128,32A8,8,0,0,0,72,64V174.08A36,36,0,1,0,88,204V118.25l112-28v51.83A36,36,0,1,0,216,172V32A8,8,0,0,0,212.92,25.69ZM52,224a20,20,0,1,1,20-20A20,20,0,0,1,52,224ZM88,101.75V70.25l112-28v31.5ZM180,192a20,20,0,1,1,20-20A20,20,0,0,1,180,192Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M212.92,25.71a7.89,7.89,0,0,0-6.86-1.46l-128,32A8,8,0,0,0,72,64V174.1A36,36,0,1,0,88,204V110.25l112-28V142.1A36,36,0,1,0,216,172V32A8,8,0,0,0,212.92,25.71Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M211.69,27.27a6,6,0,0,0-5.15-1.09l-128,32A6,6,0,0,0,74,64V178.11A34,34,0,1,0,86,204V116.68l116-29v58.43A34,34,0,1,0,214,172V32A6,6,0,0,0,211.69,27.27ZM52,226a22,22,0,1,1,22-22A22,22,0,0,1,52,226ZM86,104.32V68.68l116-29V75.32ZM180,194a22,22,0,1,1,22-22A22,22,0,0,1,180,194Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M212.92,25.69a8,8,0,0,0-6.86-1.45l-128,32A8,8,0,0,0,72,64V174.08A36,36,0,1,0,88,204V118.25l112-28v51.83A36,36,0,1,0,216,172V32A8,8,0,0,0,212.92,25.69ZM52,224a20,20,0,1,1,20-20A20,20,0,0,1,52,224ZM88,101.75V70.25l112-28v31.5ZM180,192a20,20,0,1,1,20-20A20,20,0,0,1,180,192Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M210.46,28.85a4,4,0,0,0-3.43-.73l-128,32A4,4,0,0,0,76,64V182.87A32,32,0,1,0,84,204V115.12l120-30v65.75A32,32,0,1,0,212,172V32A4,4,0,0,0,210.46,28.85ZM52,228a24,24,0,1,1,24-24A24,24,0,0,1,52,228Zm128-32a24,24,0,1,1,24-24A24,24,0,0,1,180,196ZM84,106.88V67.12l120-30V76.88Z"></path>
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
