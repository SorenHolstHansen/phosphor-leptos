//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "games", feature = "objects"))]
#[component]
pub fn Ranking(
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
                <path d="M240,200h-8V144a16,16,0,0,0-16-16H176V56a16,16,0,0,0-16-16H96A16,16,0,0,0,80,56V88H40a16,16,0,0,0-16,16v96H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM80,200H40V104H80Zm60-64a8,8,0,0,1-16,0V107.1l-1.47.49a8,8,0,0,1-5.06-15.18l12-4A8,8,0,0,1,140,96Zm76,64H176V144h40Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M40,96H88V208H32V104A8,8,0,0,1,40,96Zm176,40H168v72h56V144A8,8,0,0,0,216,136Z"
        opacity="0.2"
    ></path>
    <path d="M112.41,102.53a8,8,0,0,1,5.06-10.12l12-4A8,8,0,0,1,140,96v40a8,8,0,0,1-16,0V107.1l-1.47.49A8,8,0,0,1,112.41,102.53ZM248,208a8,8,0,0,1-8,8H16a8,8,0,0,1,0-16h8V104A16,16,0,0,1,40,88H80V56A16,16,0,0,1,96,40h64a16,16,0,0,1,16,16v72h40a16,16,0,0,1,16,16v56h8A8,8,0,0,1,248,208Zm-72-64v56h40V144ZM96,200h64V56H96Zm-56,0H80V104H40Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M116.21,101.26a4,4,0,0,1,2.53-5.05l12-4A4,4,0,0,1,136,96v40a4,4,0,0,1-8,0V101.55l-6.74,2.24A4,4,0,0,1,116.21,101.26ZM244,208a4,4,0,0,1-4,4H16a4,4,0,0,1,0-8H28V104A12,12,0,0,1,40,92H84V56A12,12,0,0,1,96,44h64a12,12,0,0,1,12,12v76h44a12,12,0,0,1,12,12v60h12A4,4,0,0,1,244,208Zm-72-68v64h48V144a4,4,0,0,0-4-4ZM92,204h72V56a4,4,0,0,0-4-4H96a4,4,0,0,0-4,4Zm-56,0H84V100H40a4,4,0,0,0-4,4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M108.62,103.79a12,12,0,0,1,7.59-15.17l12-4A12,12,0,0,1,144,96v40a12,12,0,0,1-24,0V112h0A12,12,0,0,1,108.62,103.79ZM252,208a12,12,0,0,1-12,12H16a12,12,0,0,1,0-24h4V104A20,20,0,0,1,40,84H76V56A20,20,0,0,1,96,36h64a20,20,0,0,1,20,20v68h36a20,20,0,0,1,20,20v52h4A12,12,0,0,1,252,208Zm-72-60v48h32V148Zm-80,48h56V60H100Zm-56,0H76V108H44Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M114.31,101.9a6,6,0,0,1,3.79-7.59l12-4A6,6,0,0,1,138,96v40a6,6,0,0,1-12,0V104.32l-4.1,1.37A6,6,0,0,1,114.31,101.9ZM246,208a6,6,0,0,1-6,6H16a6,6,0,0,1,0-12H26V104A14,14,0,0,1,40,90H82V56A14,14,0,0,1,96,42h64a14,14,0,0,1,14,14v74h42a14,14,0,0,1,14,14v58h10A6,6,0,0,1,246,208Zm-72-66v60h44V144a2,2,0,0,0-2-2ZM94,202h68V56a2,2,0,0,0-2-2H96a2,2,0,0,0-2,2Zm-56,0H82V102H40a2,2,0,0,0-2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M112.41,102.53a8,8,0,0,1,5.06-10.12l12-4A8,8,0,0,1,140,96v40a8,8,0,0,1-16,0V107.1l-1.47.49A8,8,0,0,1,112.41,102.53ZM248,208a8,8,0,0,1-8,8H16a8,8,0,0,1,0-16h8V104A16,16,0,0,1,40,88H80V56A16,16,0,0,1,96,40h64a16,16,0,0,1,16,16v72h40a16,16,0,0,1,16,16v56h8A8,8,0,0,1,248,208Zm-72-64v56h40V144ZM96,200h64V56H96Zm-56,0H80V104H40Z"></path>
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
