//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn ShirtFolded(
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
                <path d="M200,44H177L164.49,31.51A12,12,0,0,0,156.27,28L156,28H100c-.29,0-.56,0-.83,0a12,12,0,0,0-7.66,3.47L79,44H56A20,20,0,0,0,36,64V216a20,20,0,0,0,20,20H200a20,20,0,0,0,20-20V64A20,20,0,0,0,200,44ZM96,61l1.48-1.48L112.92,86,96,102.48Zm32,3.21L120.89,52h14.22ZM160,61v41.51L143.08,86l15.44-26.47ZM60,68H72v44a20,20,0,0,0,34.08,14.21L116,116.5V212H60ZM196,212H140V116.5l9.92,9.69A20,20,0,0,0,184,112V68h12Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,64V216a8,8,0,0,1-8,8H56a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H80v56a8,8,0,0,0,13.12,6.15L128,88l34.88,30.13A8,8,0,0,0,176,112V56h24A8,8,0,0,1,208,64ZM96,40l32,48,32-48Z"
        opacity="0.2"
    ></path>
    <path d="M200,48H179.31L165.66,34.34A8.08,8.08,0,0,0,160,32H96l-.49,0-.23,0a8,8,0,0,0-4.89,2.29L76.69,48H56A16,16,0,0,0,40,64V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V64A16,16,0,0,0,200,48ZM168,59.31V112L138.57,86.56l22.67-34ZM128,73.58,111,48h34.1Zm-33.24-21,22.67,34L88,112V59.31ZM56,64H72v48a15.85,15.85,0,0,0,9.21,14.49A16.1,16.1,0,0,0,88,128a15.89,15.89,0,0,0,10.2-3.73.52.52,0,0,0,.11-.1L120,105.48V216H56ZM200,216H136V105.48l21.65,18.7a.52.52,0,0,0,.11.1A15.89,15.89,0,0,0,168,128a16.1,16.1,0,0,0,6.83-1.54A15.85,15.85,0,0,0,184,112V64h16Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M201,48H179.35L165.66,34.34A8,8,0,0,0,160,32H96a8,8,0,0,0-5.66,2.34L76.65,48H55A15,15,0,0,0,40,63V217a15,15,0,0,0,15,15h61a4,4,0,0,0,4-4V112.27a8.18,8.18,0,0,1,7.47-8.25,8,8,0,0,1,8.53,8V228a4,4,0,0,0,4,4h61a15,15,0,0,0,15-15V63A15,15,0,0,0,201,48ZM86.54,115.08A4,4,0,0,1,80,112V67.31L95.24,52.07l23.47,35.21ZM128,88h0v0Zm48,24a4,4,0,0,1-2.3,3.63,3.93,3.93,0,0,1-4.21-.51l-32.2-27.82,23.47-35.21L176,67.31Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M200,50H178.48L164.24,35.76A6,6,0,0,0,160,34H96a6,6,0,0,0-4.21,1.76L77.52,50H56A14,14,0,0,0,42,64V216a14,14,0,0,0,14,14H200a14,14,0,0,0,14-14V64A14,14,0,0,0,200,50Zm-30,8.49V112a2,2,0,0,1-3.25,1.56L135.93,86.92l25-37.5ZM128,77.18,107.21,46h41.58ZM95.07,49.42l25,37.5L89.25,113.54A2,2,0,0,1,86,112V58.49ZM54,216V64a2,2,0,0,1,2-2H74v50a13.87,13.87,0,0,0,8.06,12.68A14.11,14.11,0,0,0,88,126,13.87,13.87,0,0,0,97,122.74l.08-.07,25-21.56V218H56A2,2,0,0,1,54,216Zm148,0a2,2,0,0,1-2,2H134V101.11l25,21.56.08.07A13.87,13.87,0,0,0,168,126a14.08,14.08,0,0,0,6-1.35A13.87,13.87,0,0,0,182,112V62h18a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M200,48H179.31L165.66,34.34A8.07,8.07,0,0,0,160.05,32H96a8,8,0,0,0-5.66,2.34L76.69,48H56A16,16,0,0,0,40,64V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V64A16,16,0,0,0,200,48Zm-38.76,4.56L168,59.31V112L138.57,86.56ZM88,59.31l6.76-6.75,22.67,34L88,112ZM120,216H56V64H72v48a15.85,15.85,0,0,0,9.21,14.49A16.1,16.1,0,0,0,88,128a15.89,15.89,0,0,0,10.2-3.73.52.52,0,0,0,.11-.1L120,105.48ZM111,48h34.1L128,73.58ZM200,216H136V105.48l21.65,18.7a.52.52,0,0,0,.11.1A15.89,15.89,0,0,0,168,128a16.1,16.1,0,0,0,6.83-1.54A15.85,15.85,0,0,0,184,112V64h16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M200,52H177.66L162.83,37.17A4,4,0,0,0,160.39,36a2.13,2.13,0,0,0-.39,0H96v0a4,4,0,0,0-2.83,1.15L78.34,52H56A12,12,0,0,0,44,64V216a12,12,0,0,0,12,12H200a12,12,0,0,0,12-12V64A12,12,0,0,0,200,52Zm-28,5.66V112a4,4,0,0,1-2.3,3.63,3.93,3.93,0,0,1-4.21-.51l-32.2-27.82,27.33-41ZM128,80.79,103.47,44h49.06ZM95.38,46.28l27.33,41-32.17,27.8A4,4,0,0,1,84,112V57.66ZM52,216V64a4,4,0,0,1,4-4H76v52a11.89,11.89,0,0,0,6.91,10.87A12.08,12.08,0,0,0,88,124a11.88,11.88,0,0,0,7.65-2.8l.06,0L124,96.74V220H56A4,4,0,0,1,52,216Zm152,0a4,4,0,0,1-4,4H132V96.74l28.32,24.46A11.9,11.9,0,0,0,168,124a12.08,12.08,0,0,0,5.12-1.15A11.89,11.89,0,0,0,180,112V60h20a4,4,0,0,1,4,4Z"></path>
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
