//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn BugBeetle(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M224,152a12,12,0,0,0,0-24H212V112h12a12,12,0,0,0,0-24H210.45a83.7,83.7,0,0,0-18.78-38.7l16.82-16.81a12,12,0,0,0-17-17l-18,18a83.7,83.7,0,0,0-91.1,0l-18-18a12,12,0,0,0-17,17L64.33,49.3A83.7,83.7,0,0,0,45.55,88H32a12,12,0,0,0,0,24H44v16H32a12,12,0,0,0,0,24H44a83.55,83.55,0,0,0,1.55,16H32a12,12,0,0,0,0,24H54.15a84,84,0,0,0,147.7,0H224a12,12,0,0,0,0-24H210.45A83.55,83.55,0,0,0,212,152ZM128,44a60.1,60.1,0,0,1,57.82,44H70.18A60.1,60.1,0,0,1,128,44Zm12,166.79V140a12,12,0,0,0-24,0v70.79A60.09,60.09,0,0,1,68,152V112H188v40A60.09,60.09,0,0,1,140,210.79Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M200,104v48a72,72,0,0,1-72,72h0a72,72,0,0,1-72-72V104Z" opacity="0.2"/><path d="M208,144h16a8,8,0,0,0,0-16H208V112h16a8,8,0,0,0,0-16H207.6a79.76,79.76,0,0,0-21.44-46.85l19.5-19.49a8,8,0,1,0-11.32-11.32l-20.29,20.3a79.74,79.74,0,0,0-92.1,0L61.66,18.34A8,8,0,0,0,50.34,29.66l19.5,19.49A79.76,79.76,0,0,0,48.4,96H32a8,8,0,0,0,0,16H48v16H32a8,8,0,0,0,0,16H48v8c0,2.7.14,5.37.4,8H32a8,8,0,0,0,0,16H51.68a80,80,0,0,0,152.64,0H224a8,8,0,0,0,0-16H207.6c.26-2.63.4-5.3.4-8ZM128,40a64.07,64.07,0,0,1,63.48,56h-127A64.07,64.07,0,0,1,128,40Zm8,175.48V136a8,8,0,0,0-16,0v79.48A64.07,64.07,0,0,1,64,152V112H192v40A64.07,64.07,0,0,1,136,215.48Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M224,112H208V96h16a8,8,0,0,1,0,16ZM32,96a8,8,0,0,0,0,16H48V96Zm176,56c0,2.7-.14,5.37-.4,8H224a8,8,0,0,1,0,16H204.32a80,80,0,0,1-152.64,0H32a8,8,0,0,1,0-16H48.4c-.26-2.63-.4-5.3-.4-8v-8H32a8,8,0,0,1,0-16H48V112H208v16h16a8,8,0,0,1,0,16H208Zm-72-16a8,8,0,0,0-16,0v64a8,8,0,0,0,16,0ZM69.84,49.15A79.76,79.76,0,0,0,48.4,96H207.6a79.76,79.76,0,0,0-21.44-46.85l19.5-19.49a8,8,0,1,0-11.32-11.32l-20.29,20.3a79.74,79.74,0,0,0-92.1,0L61.66,18.34A8,8,0,0,0,50.34,29.66Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M206,142h18a6,6,0,0,0,0-12H206V110h18a6,6,0,0,0,0-12H205.75a77.81,77.81,0,0,0-22.38-48.88l20.87-20.88a6,6,0,0,0-8.48-8.48L174.27,41.25a77.8,77.8,0,0,0-92.53,0L60.24,19.76a6,6,0,0,0-8.48,8.48L72.64,49.12A77.76,77.76,0,0,0,50.25,98H32a6,6,0,0,0,0,12H50v20H32a6,6,0,0,0,0,12H50v10a78.6,78.6,0,0,0,.66,10H32a6,6,0,0,0,0,12H53.18a78,78,0,0,0,149.64,0H224a6,6,0,0,0,0-12H205.34a78.6,78.6,0,0,0,.66-10ZM128,38a66.07,66.07,0,0,1,65.71,60H62.29A66.07,66.07,0,0,1,128,38Zm6,179.71V136a6,6,0,0,0-12,0v81.71A66.07,66.07,0,0,1,62,152V110H194v42A66.07,66.07,0,0,1,134,217.71Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,144h16a8,8,0,0,0,0-16H208V112h16a8,8,0,0,0,0-16H207.6a79.76,79.76,0,0,0-21.44-46.85l19.5-19.49a8,8,0,1,0-11.32-11.32l-20.29,20.3a79.74,79.74,0,0,0-92.1,0L61.66,18.34A8,8,0,0,0,50.34,29.66l19.5,19.49A79.76,79.76,0,0,0,48.4,96H32a8,8,0,0,0,0,16H48v16H32a8,8,0,0,0,0,16H48v8c0,2.7.14,5.37.4,8H32a8,8,0,0,0,0,16H51.68a80,80,0,0,0,152.64,0H224a8,8,0,0,0,0-16H207.6c.26-2.63.4-5.3.4-8ZM128,40a64.07,64.07,0,0,1,63.48,56h-127A64.07,64.07,0,0,1,128,40Zm8,175.48V136a8,8,0,0,0-16,0v79.48A64.07,64.07,0,0,1,64,152V112H192v40A64.07,64.07,0,0,1,136,215.48Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M204,140h20a4,4,0,0,0,0-8H204V108h20a4,4,0,0,0,0-8H203.89a75.8,75.8,0,0,0-23.37-50.86l22.31-22.31a4,4,0,1,0-5.66-5.66L174.44,43.9a75.77,75.77,0,0,0-92.88,0L58.83,21.17a4,4,0,0,0-5.66,5.66L75.48,49.14A75.8,75.8,0,0,0,52.11,100H32a4,4,0,0,0,0,8H52v24H32a4,4,0,0,0,0,8H52v12a75.41,75.41,0,0,0,1,12H32a4,4,0,0,0,0,8H54.69a76,76,0,0,0,146.62,0H224a4,4,0,0,0,0-8H203a75.41,75.41,0,0,0,1-12ZM128,36a68.08,68.08,0,0,1,67.87,64H60.13A68.08,68.08,0,0,1,128,36Zm4,183.87V136a4,4,0,0,0-8,0v83.87A68.08,68.08,0,0,1,60,152V108H196v44A68.08,68.08,0,0,1,132,219.87Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            class=class
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
