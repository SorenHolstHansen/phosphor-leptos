//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn GoogleChromeLogo(
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
            IconWeight::Bold => view!{ <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,24a83.89,83.89,0,0,1,65.9,32H128a52.05,52.05,0,0,0-46.15,28.07L64.18,73.47A83.82,83.82,0,0,1,128,44Zm28,84a28,28,0,1,1-28-28A28,28,0,0,1,156,128ZM44,128a83.41,83.41,0,0,1,6-31.11L83,154c.06.11.14.2.2.3A52,52,0,0,0,128,180q1.19,0,2.34-.06l-17.68,30.63A84.12,84.12,0,0,1,44,128Zm96.05,83.12L173,154c.09-.15.16-.3.24-.46A51.81,51.81,0,0,0,171.78,100h35.4a83.95,83.95,0,0,1-67.13,111.12Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M168,128a40,40,0,1,1-40-40A40,40,0,0,1,168,128Z" opacity="0.2"/><path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,16a88,88,0,0,1,73.72,40H128a48.08,48.08,0,0,0-45.6,33l-23.08-40A87.89,87.89,0,0,1,128,40Zm32,88a32,32,0,1,1-32-32A32,32,0,0,1,160,128Zm-45.28,87A88,88,0,0,1,49.56,88.14L86.43,152c.06.1.13.19.19.28A48,48,0,0,0,137.82,175Zm18,.87L169.57,152c.08-.14.14-.28.22-.42a47.88,47.88,0,0,0-6-55.58H210a88,88,0,0,1-77.29,119.87Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,16a88,88,0,0,1,73.72,40H128a48.08,48.08,0,0,0-45.6,33l-23.08-40A87.89,87.89,0,0,1,128,40ZM40,128a87.44,87.44,0,0,1,9.56-39.86L86.43,152c.06.1.13.19.19.28A48,48,0,0,0,137.82,175l-23.1,40A88.14,88.14,0,0,1,40,128Zm92.69,87.87L169.57,152c.08-.14.14-.28.22-.42a47.88,47.88,0,0,0-6-55.58H210a88,88,0,0,1-77.29,119.87Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,12a90,90,0,0,1,77.33,44H128a46.07,46.07,0,0,0-44.93,36.17L56.91,72.87A89.91,89.91,0,0,1,128,38Zm34,90a34,34,0,1,1-34-34A34,34,0,0,1,162,128ZM38,128A89.4,89.4,0,0,1,49.5,84l38.66,67c.06.1.13.18.19.27A45.94,45.94,0,0,0,142,171.83l-26.17,45.34A90.13,90.13,0,0,1,38,128Zm91.16,90,38.68-67,.21-.41A45.9,45.9,0,0,0,159,94h52.37a90,90,0,0,1-82.16,124Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,16a88,88,0,0,1,73.72,40H128a48.08,48.08,0,0,0-45.6,33l-23.08-40A87.89,87.89,0,0,1,128,40Zm32,88a32,32,0,1,1-32-32A32,32,0,0,1,160,128ZM40,128a87.44,87.44,0,0,1,9.56-39.86L86.43,152c.06.1.13.19.19.28A48,48,0,0,0,137.82,175l-23.1,40A88.14,88.14,0,0,1,40,128Zm92.69,87.87L169.57,152c.08-.14.14-.28.22-.42a47.88,47.88,0,0,0-6-55.58H210a88,88,0,0,1-77.29,119.87Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,8a92.08,92.08,0,0,1,80.78,48H128a44.05,44.05,0,0,0-43.82,40.11L54.51,72.72A91.9,91.9,0,0,1,128,36Zm0,128a36,36,0,1,1,36-36A36,36,0,0,1,128,164ZM36,128A91.52,91.52,0,0,1,49.51,80.05L89.9,150c0,.09.11.17.17.26a43.93,43.93,0,0,0,56.47,17.63l-29.7,51.43A92.13,92.13,0,0,1,36,128Zm92,92c-.77,0-1.53,0-2.29,0l40.39-70a1.21,1.21,0,0,0,.09-.2A43.89,43.89,0,0,0,153.25,92h59.41A92,92,0,0,1,128,220Z"/> }.into_view()
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
