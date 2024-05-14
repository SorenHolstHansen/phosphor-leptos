//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects"))]
#[component]
pub fn HairDryer(
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
                <path d="M177.42,206.62,209,137.07A64,64,0,0,0,168,24a8.4,8.4,0,0,0-1.32.11L29.37,47A16,16,0,0,0,16,62.78v50.44A16,16,0,0,0,29.37,129L128,145.44V200a16,16,0,0,0,16,16,40,40,0,0,0,40,40h16a8,8,0,0,0,0-16H184a24,24,0,0,1-24-24h2.85A16,16,0,0,0,177.42,206.62ZM192,88a24,24,0,1,1-24-24A24,24,0,0,1,192,88Zm-25.32,63.89A8.4,8.4,0,0,0,168,152a63.9,63.9,0,0,0,17.82-2.54l-23,50.54H144V148.11Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M168,32,30.68,54.89A8,8,0,0,0,24,62.78v50.44a8,8,0,0,0,6.68,7.89L168,144a56,56,0,0,0,0-112Zm0,80a24,24,0,1,1,24-24A24,24,0,0,1,168,112Z"
        opacity="0.2"
    ></path>
    <path d="M200,88a32,32,0,1,0-32,32A32,32,0,0,0,200,88Zm-32,16a16,16,0,1,1,16-16A16,16,0,0,1,168,104Zm9.42,102.62L209,137.07A64,64,0,0,0,168,24a8.4,8.4,0,0,0-1.32.11L29.37,47A16,16,0,0,0,16,62.78v50.44A16,16,0,0,0,29.37,129L128,145.44V200a16,16,0,0,0,16,16,40,40,0,0,0,40,40h16a8,8,0,0,0,0-16H184a24,24,0,0,1-24-24h2.85A16,16,0,0,0,177.42,206.62ZM32,62.78,168.64,40a48,48,0,0,1,0,96L32,113.23Zm134.68,89.11A8.4,8.4,0,0,0,168,152a63.9,63.9,0,0,0,17.82-2.54l-23,50.54H144V148.11Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M196,88a28,28,0,1,0-28,28A28,28,0,0,0,196,88Zm-28,20a20,20,0,1,1,20-20A20,20,0,0,1,168,108Zm-5.15,104a12,12,0,0,0,10.92-7l32-70.39A60,60,0,0,0,168,28a4.89,4.89,0,0,0-.66.05L30,50.94A12,12,0,0,0,20,62.78v50.44a12,12,0,0,0,10,11.84l102,17V200a12,12,0,0,0,12,12h4v4a36,36,0,0,0,36,36h16a4,4,0,0,0,0-8H184a28,28,0,0,1-28-28v-4ZM28,113.22V62.78a4,4,0,0,1,3.34-4L168.33,36a52,52,0,0,1,0,104l-137-22.83A4,4,0,0,1,28,113.22ZM140,200V143.39L167.34,148a4.89,4.89,0,0,0,.66.05,59.62,59.62,0,0,0,25.46-5.69l-27,59.34a4,4,0,0,1-3.64,2.35H144A4,4,0,0,1,140,200Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M200,88a32,32,0,1,0-32,32A32,32,0,0,0,200,88Zm-32,8a8,8,0,1,1,8-8A8,8,0,0,1,168,96Zm13.06,112.28,31.25-68.76A68,68,0,0,0,168,20a11.81,11.81,0,0,0-2,.16L28.71,43.05A19.94,19.94,0,0,0,12,62.78v50.44A19.94,19.94,0,0,0,28.71,133L124,148.83V200a20,20,0,0,0,16.69,19.7A44.06,44.06,0,0,0,184,256h16a12,12,0,0,0,0-24H184a20,20,0,0,1-18.41-12.19A20.09,20.09,0,0,0,181.06,208.28ZM36,66.17,168.93,44a44,44,0,0,1,0,88L36,109.83Zm130,89.67a11.81,11.81,0,0,0,2,.16,68.08,68.08,0,0,0,10.85-.88L160.27,196H148V152.83Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M198,88a30,30,0,1,0-30,30A30,30,0,0,0,198,88Zm-30,18a18,18,0,1,1,18-18A18,18,0,0,1,168,106Zm-5.15,108a14,14,0,0,0,12.74-8.21l31.8-69.94A62,62,0,0,0,168,26a6.61,6.61,0,0,0-1,.08L29.7,49A14,14,0,0,0,18,62.78v50.44A14,14,0,0,0,29.7,127L130,143.75V200a14,14,0,0,0,14,14h2v2a38,38,0,0,0,38,38h16a6,6,0,0,0,0-12H184a26,26,0,0,1-26-26v-2ZM30,113.22V62.78a2,2,0,0,1,1.67-2L168.48,38a50,50,0,0,1,0,100L31.67,115.2A2,2,0,0,1,30,113.22ZM142,200V145.75l25,4.17a6.61,6.61,0,0,0,1,.08,61.75,61.75,0,0,0,21.53-3.86l-24.86,54.69a2,2,0,0,1-1.82,1.17H144A2,2,0,0,1,142,200Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M200,88a32,32,0,1,0-32,32A32,32,0,0,0,200,88Zm-32,16a16,16,0,1,1,16-16A16,16,0,0,1,168,104Zm9.42,102.62L209,137.07A64,64,0,0,0,168,24a8.4,8.4,0,0,0-1.32.11L29.37,47A16,16,0,0,0,16,62.78v50.44A16,16,0,0,0,29.37,129L128,145.44V200a16,16,0,0,0,16,16,40,40,0,0,0,40,40h16a8,8,0,0,0,0-16H184a24,24,0,0,1-24-24h2.85A16,16,0,0,0,177.42,206.62ZM32,62.78,168.64,40a48,48,0,0,1,0,96L32,113.23Zm134.68,89.11A8.4,8.4,0,0,0,168,152a63.9,63.9,0,0,0,17.82-2.54l-23,50.54H144V148.11Z"></path>
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