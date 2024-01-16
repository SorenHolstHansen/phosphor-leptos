//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn BluetoothX(
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
            IconWeight::Bold => view!{ <path d="M191.2,166.4,140,128l11.61-8.71a12,12,0,1,0-14.4-19.2L132,104V56l5.21,3.91a12,12,0,1,0,14.4-19.2L127.2,22.4A12,12,0,0,0,108,32v72L63.2,70.4A12,12,0,0,0,48.8,89.6L100,128,48.8,166.4a12,12,0,1,0,14.4,19.2L108,152v72a12,12,0,0,0,19.2,9.6l64-48a12,12,0,0,0,0-19.2ZM132,200V152l32,24ZM240.49,95.51a12,12,0,0,1-17,17L208,97l-15.51,15.52a12,12,0,1,1-17-17L191,80,175.52,64.49a12,12,0,1,1,17-17L208,63l15.51-15.52a12,12,0,0,1,17,17L225,80Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M120,128l64,48-64,48Zm0-96v96l55.47-41.6a8,8,0,0,0,0-12.8Z" opacity="0.2"/><path d="M188.8,169.6,133.33,128l23.47-17.6a8,8,0,0,0-9.6-12.8L128,112V48l19.2,14.4a8,8,0,1,0,9.6-12.8l-32-24A8,8,0,0,0,112,32v80L60.8,73.6a8,8,0,0,0-9.6,12.8L106.67,128,51.2,169.6a8,8,0,1,0,9.6,12.8L112,144v80a8,8,0,0,0,12.8,6.4l64-48a8,8,0,0,0,0-12.8ZM128,208V144l42.67,32ZM237.66,98.34a8,8,0,0,1-11.32,11.32L208,91.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L196.69,80,178.34,61.66a8,8,0,0,1,11.32-11.32L208,68.69l18.34-18.35a8,8,0,0,1,11.32,11.32L219.31,80Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M192,176a8,8,0,0,1-3.2,6.4l-64,48A8,8,0,0,1,120,232a7.9,7.9,0,0,1-4.11-1.14,8.3,8.3,0,0,1-3.9-7.18V144L60.76,182.4a8,8,0,0,1-11.16-1.55,8.26,8.26,0,0,1,1.8-11.43L106.66,128,51.38,86.57a8.19,8.19,0,0,1-2.13-10.93,8,8,0,0,1,11.51-2L112,112V32.24a8.21,8.21,0,0,1,2.83-6.34,8,8,0,0,1,10-.3l33.62,25.2A4,4,0,0,1,160,54v52a4,4,0,0,1-1.6,3.2L133.34,128l55.5,41.6A8,8,0,0,1,192,176Zm45.47-77.87L219.37,80l18.11-18.11a8.21,8.21,0,0,0,.41-11.37,8,8,0,0,0-11.49-.18L208.05,68.69,189.93,50.58a8.23,8.23,0,0,0-11.38-.41,8,8,0,0,0-.18,11.49L196.73,80,178.58,98.13a8.2,8.2,0,0,0-.6,11.1,8,8,0,0,0,11.71.43l18.36-18.35,18.35,18.35a8,8,0,0,0,11.72-.43A8.21,8.21,0,0,0,237.51,98.13Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M187.6,171.2,130,128l25.6-19.2a6,6,0,1,0-7.2-9.6L126,116V44l22.4,16.8a6,6,0,1,0,7.2-9.6l-32-24A6,6,0,0,0,114,32v84L59.6,75.2a6,6,0,0,0-7.2,9.6L110,128,52.4,171.2a6,6,0,0,0,7.2,9.6L114,140v84a6,6,0,0,0,9.6,4.8l64-48a6,6,0,0,0,0-9.6ZM126,212V140l48,36ZM236.24,99.76a6,6,0,1,1-8.48,8.48L208,88.49l-19.76,19.75a6,6,0,0,1-8.48-8.48L199.51,80,179.76,60.24a6,6,0,0,1,8.48-8.48L208,71.51l19.76-19.75a6,6,0,0,1,8.48,8.48L216.49,80Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M188.8,169.6,133.33,128l23.47-17.6a8,8,0,0,0-9.6-12.8L128,112V48l19.2,14.4a8,8,0,1,0,9.6-12.8l-32-24A8,8,0,0,0,112,32v80L60.8,73.6a8,8,0,0,0-9.6,12.8L106.67,128,51.2,169.6a8,8,0,1,0,9.6,12.8L112,144v80a8,8,0,0,0,12.8,6.4l64-48a8,8,0,0,0,0-12.8ZM128,208V144l42.67,32ZM237.66,98.34a8,8,0,0,1-11.32,11.32L208,91.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L196.69,80,178.34,61.66a8,8,0,0,1,11.32-11.32L208,68.69l18.34-18.35a8,8,0,0,1,11.32,11.32L219.31,80Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M186.4,172.8,126.67,128l27.73-20.8a4,4,0,1,0-4.8-6.4L124,120V40l25.6,19.2a4,4,0,0,0,4.8-6.4l-32-24A4,4,0,0,0,116,32v88L58.4,76.8a4,4,0,0,0-4.8,6.4L113.33,128,53.6,172.8a4,4,0,0,0,4.8,6.4L116,136v88a4,4,0,0,0,2.21,3.58A4.05,4.05,0,0,0,120,228a4,4,0,0,0,2.4-.8l64-48a4,4,0,0,0,0-6.4ZM124,216V136l53.33,40ZM234.83,101.17a4,4,0,0,1-5.66,5.66L208,85.66l-21.17,21.17a4,4,0,0,1-5.66-5.66L202.34,80,181.17,58.83a4,4,0,0,1,5.66-5.66L208,74.34l21.17-21.17a4,4,0,1,1,5.66,5.66L213.66,80Z"/> }.into_view()
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
