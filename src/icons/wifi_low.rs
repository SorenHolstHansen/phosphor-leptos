//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn WifiLow(
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
            IconWeight::Bold => view!{ <path d="M144,204a16,16,0,1,1-16-16A16,16,0,0,1,144,204Zm31.06-48.7a80,80,0,0,0-94.12,0,12,12,0,1,0,14.13,19.4,56,56,0,0,1,65.86,0,12,12,0,1,0,14.13-19.4Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M171.68,167.88l-37.53,45.24a8,8,0,0,1-12.3,0L84.32,167.88a68,68,0,0,1,87.36,0Z" opacity="0.2"/><path d="M247.89,80.91a15.93,15.93,0,0,0-6.17-10.81A186.67,186.67,0,0,0,128,32,186.67,186.67,0,0,0,14.28,70.1,15.93,15.93,0,0,0,8.11,80.91,15.65,15.65,0,0,0,11.65,92.8l104,125.43A15.93,15.93,0,0,0,128,224h0a15.93,15.93,0,0,0,12.31-5.77h0l104-125.43A15.65,15.65,0,0,0,247.89,80.91ZM128,208l-32.1-38.7a60,60,0,0,1,64.2,0Zm42.37-51.08a75.89,75.89,0,0,0-84.74,0L24.09,82.74A170.76,170.76,0,0,1,128,48,170.76,170.76,0,0,1,231.91,82.74Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M247.89,80.91a15.93,15.93,0,0,0-6.17-10.81A186.67,186.67,0,0,0,128,32,186.67,186.67,0,0,0,14.28,70.1,15.93,15.93,0,0,0,8.11,80.91,15.65,15.65,0,0,0,11.65,92.8l104,125.43A15.93,15.93,0,0,0,128,224h0a15.93,15.93,0,0,0,12.31-5.77h0l104-125.43A15.65,15.65,0,0,0,247.89,80.91Zm-77.52,76a75.89,75.89,0,0,0-84.74,0L24.09,82.74A170.76,170.76,0,0,1,128,48,170.76,170.76,0,0,1,231.91,82.74Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M138,204a10,10,0,1,1-10-10A10,10,0,0,1,138,204Zm33.53-43.85a74,74,0,0,0-87.06,0,6,6,0,0,0,7.06,9.7,62,62,0,0,1,72.94,0,6,6,0,0,0,8.38-1.32A6,6,0,0,0,171.53,160.15Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M140,204a12,12,0,1,1-12-12A12,12,0,0,1,140,204Zm32.71-45.47a76.05,76.05,0,0,0-89.42,0,8,8,0,0,0,9.42,12.94,60,60,0,0,1,70.58,0,8,8,0,1,0,9.42-12.94Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M136,204a8,8,0,1,1-8-8A8,8,0,0,1,136,204Zm34.35-42.23a72,72,0,0,0-84.7,0,4,4,0,1,0,4.71,6.46,64,64,0,0,1,75.28,0,4,4,0,1,0,4.71-6.46Z"/> }.into_view()
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
