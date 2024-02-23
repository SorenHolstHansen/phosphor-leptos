//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "office", feature = "editor", feature = "system"))]
#[component]
pub fn FloppyDiskBack(
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
                <path d="M208,28H91.31a19.86,19.86,0,0,0-14.14,5.86L33.86,77.17A19.86,19.86,0,0,0,28,91.31V208a20,20,0,0,0,20,20H208a20,20,0,0,0,20-20V48A20,20,0,0,0,208,28Zm-4,176H52V93L93,52h71V76H100a12,12,0,0,0,0,24h68a20,20,0,0,0,20-20V52h16Zm-76-88a36,36,0,1,0,36,36A36,36,0,0,0,128,116Zm0,48a12,12,0,1,1,12-12A12,12,0,0,1,128,164Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,40H91.31a8,8,0,0,0-5.65,2.34L42.34,85.66A8,8,0,0,0,40,91.31V208a8,8,0,0,0,8,8H208a8,8,0,0,0,8-8V48A8,8,0,0,0,208,40ZM128,184a32,32,0,1,1,32-32A32,32,0,0,1,128,184Z"
        opacity="0.2"
    ></path>
    <path d="M208,32H91.31A15.86,15.86,0,0,0,80,36.69L36.69,80A15.86,15.86,0,0,0,32,91.31V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32Zm0,176H48V91.31L91.31,48H168V80H88a8,8,0,0,0,0,16h80a16,16,0,0,0,16-16V48h24Zm-80-96a40,40,0,1,0,40,40A40,40,0,0,0,128,112Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,128,176Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M208,32H188a4,4,0,0,0-4,4V80a16,16,0,0,1-16,16H88.27A8.17,8.17,0,0,1,80,88.53,8,8,0,0,1,88,80h76a4,4,0,0,0,4-4V36a4,4,0,0,0-4-4H91.31A15.86,15.86,0,0,0,80,36.69L36.69,80A15.86,15.86,0,0,0,32,91.31V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM128,184a32,32,0,1,1,32-32A32,32,0,0,1,128,184Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,34H91.31a13.94,13.94,0,0,0-9.9,4.1L38.1,81.41a13.94,13.94,0,0,0-4.1,9.9V208a14,14,0,0,0,14,14H208a14,14,0,0,0,14-14V48A14,14,0,0,0,208,34Zm2,174a2,2,0,0,1-2,2H48a2,2,0,0,1-2-2V91.31a2,2,0,0,1,.59-1.41L89.9,46.59A2,2,0,0,1,91.31,46H170V80a2,2,0,0,1-2,2H88a6,6,0,0,0,0,12h80a14,14,0,0,0,14-14V46h26a2,2,0,0,1,2,2Zm-82-94a38,38,0,1,0,38,38A38,38,0,0,0,128,114Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,128,178Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,32H91.31A15.86,15.86,0,0,0,80,36.69L36.69,80A15.86,15.86,0,0,0,32,91.31V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32Zm0,176H48V91.31L91.31,48H168V80H88a8,8,0,0,0,0,16h80a16,16,0,0,0,16-16V48h24Zm-80-96a40,40,0,1,0,40,40A40,40,0,0,0,128,112Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,128,176Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,36H91.31a12,12,0,0,0-8.48,3.51L39.52,82.83A11.9,11.9,0,0,0,36,91.31V208a12,12,0,0,0,12,12H208a12,12,0,0,0,12-12V48A12,12,0,0,0,208,36Zm4,172a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V91.31a4,4,0,0,1,1.17-2.82L88.48,45.17A4,4,0,0,1,91.31,44H172V80a4,4,0,0,1-4,4H88a4,4,0,0,0,0,8h80a12,12,0,0,0,12-12V44h28a4,4,0,0,1,4,4Zm-84-92a36,36,0,1,0,36,36A36,36,0,0,0,128,116Zm0,64a28,28,0,1,1,28-28A28,28,0,0,1,128,180Z"></path>
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
