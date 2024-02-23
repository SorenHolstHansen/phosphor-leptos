//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "objects"))]
#[component]
pub fn Barricade(
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
                <path d="M224,60H32A20,20,0,0,0,12,80v72a20,20,0,0,0,20,20H52v28a12,12,0,0,0,24,0V172H180v28a12,12,0,0,0,24,0V172h20a20,20,0,0,0,20-20V80A20,20,0,0,0,224,60Zm-4,59L185,84h35Zm-43,29L113,84H151l64,64Zm-72,0L41,84H79l64,64ZM36,113l35,35H36Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,80v68L156,72h68A8,8,0,0,1,232,80ZM32,72a8,8,0,0,0-8,8v4l76,76h72L84,72Z"
        opacity="0.2"
    ></path>
    <path d="M224,64H32A16,16,0,0,0,16,80v72a16,16,0,0,0,16,16H56v32a8,8,0,0,0,16,0V168H184v32a8,8,0,0,0,16,0V168h24a16,16,0,0,0,16-16V80A16,16,0,0,0,224,64Zm0,64.69L175.31,80H224ZM80.69,80l72,72H103.31L32,80.69V80ZM32,103.31,80.69,152H32ZM224,152H175.31l-72-72h49.38L224,151.32V152Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M224,64H32A16,16,0,0,0,16,80v72a16,16,0,0,0,16,16H56v32a8,8,0,0,0,16,0V168H184v32a8,8,0,0,0,16,0V168h24a16,16,0,0,0,16-16V80A16,16,0,0,0,224,64ZM32,92l60,60H32Zm192,60H164L92,80h72l60,60v12Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M224,66H32A14,14,0,0,0,18,80v72a14,14,0,0,0,14,14H58v34a6,6,0,0,0,12,0V166H186v34a6,6,0,0,0,12,0V166h26a14,14,0,0,0,14-14V80A14,14,0,0,0,224,66Zm2,14v53.52L170.48,78H224A2,2,0,0,1,226,80ZM32,78H81.52l76,76h-55L30,81.52V80A2,2,0,0,1,32,78Zm-2,74V98.48L85.52,154H32A2,2,0,0,1,30,152Zm194,2H174.48l-76-76h55L226,150.48V152A2,2,0,0,1,224,154Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,64H32A16,16,0,0,0,16,80v72a16,16,0,0,0,16,16H56v32a8,8,0,0,0,16,0V168H184v32a8,8,0,0,0,16,0V168h24a16,16,0,0,0,16-16V80A16,16,0,0,0,224,64Zm0,64.69L175.31,80H224ZM80.69,80l72,72H103.31L32,80.69V80ZM32,103.31,80.69,152H32ZM224,152H175.31l-72-72h49.38L224,151.32V152Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M224,68H32A12,12,0,0,0,20,80v72a12,12,0,0,0,12,12H60v36a4,4,0,0,0,8,0V164H188v36a4,4,0,0,0,8,0V164h28a12,12,0,0,0,12-12V80A12,12,0,0,0,224,68Zm4,12v58.34L165.66,76H224A4,4,0,0,1,228,80ZM32,76H82.34l80,80H101.66L28,82.34V80A4,4,0,0,1,32,76Zm-4,76V93.66L90.34,156H32A4,4,0,0,1,28,152Zm196,4H173.66l-80-80h60.68L228,149.66V152A4,4,0,0,1,224,156Z"></path>
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
