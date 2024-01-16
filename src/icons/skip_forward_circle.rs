//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn SkipForwardCircle(
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
            IconWeight::Bold => view!{ <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,192a84,84,0,1,1,84-84A84.09,84.09,0,0,1,128,212ZM160,76a12,12,0,0,0-12,12v18.35L102.36,77.82A12,12,0,0,0,84,88v80a12,12,0,0,0,18.36,10.18L148,149.65V168a12,12,0,0,0,24,0V88A12,12,0,0,0,160,76Zm-52,70.35v-36.7L137.36,128Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M128,32a96,96,0,1,0,96,96A96,96,0,0,0,128,32ZM96,168V88l64,40Z" opacity="0.2"/><path d="M160,80a8,8,0,0,0-8,8v25.57L100.24,81.22A8,8,0,0,0,88,88v80a8,8,0,0,0,12.24,6.78L152,142.43V168a8,8,0,0,0,16,0V88A8,8,0,0,0,160,80Zm-56,73.57V102.43L144.91,128ZM128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm40,144a8,8,0,0,1-16,0V137.83l-51.35,36.68A8,8,0,0,1,88,168V88a8,8,0,0,1,12.65-6.51L152,118.17V88a8,8,0,0,1,16,0Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218ZM160,82a6,6,0,0,0-6,6v29.17L99.18,82.91A6,6,0,0,0,90,88v80a6,6,0,0,0,9.18,5.09L154,138.83V168a6,6,0,0,0,12,0V88A6,6,0,0,0,160,82Zm-58,75.17V98.83L148.68,128Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216ZM160,80a8,8,0,0,0-8,8v25.57L100.24,81.22A8,8,0,0,0,88,88v80a8,8,0,0,0,12.24,6.78L152,142.43V168a8,8,0,0,0,16,0V88A8,8,0,0,0,160,80Zm-56,73.57V102.43L144.91,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,192a92,92,0,1,1,92-92A92.1,92.1,0,0,1,128,220ZM160,84a4,4,0,0,0-4,4v32.78L98.12,84.61A4,4,0,0,0,92,88v80a4,4,0,0,0,2.06,3.5A4.06,4.06,0,0,0,96,172a4,4,0,0,0,2.12-.61L156,135.22V168a4,4,0,0,0,8,0V88A4,4,0,0,0,160,84Zm-60,76.78V95.22L152.45,128Z"/> }.into_view()
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
