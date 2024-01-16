//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn MicrosoftWordLogo(
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
            IconWeight::Bold => view!{ <path d="M200,20H72A20,20,0,0,0,52,40V56H36A20,20,0,0,0,16,76V180a20,20,0,0,0,20,20H52v16a20,20,0,0,0,20,20H200a20,20,0,0,0,20-20V40A20,20,0,0,0,200,20Zm-32,88h28v40H168ZM76,44H196V84H168V76a20,20,0,0,0-20-20H76ZM40,80H144v96H40ZM76,212V200h72a20,20,0,0,0,20-20v-8h28v40ZM64.36,154.91l-12-48a12,12,0,1,1,23.28-5.82l4.13,16.53,1.5-3a12,12,0,0,1,21.46,0l1.5,3,4.13-16.53a12,12,0,0,1,23.28,5.82l-12,48a12,12,0,0,1-10.33,9A11.62,11.62,0,0,1,108,164a12,12,0,0,1-10.73-6.63L92,146.83l-5.27,10.54a12,12,0,0,1-22.37-2.46Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M152,80v96a8,8,0,0,1-8,8H40a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H144A8,8,0,0,1,152,80Z" opacity="0.2"/><path d="M200,24H72A16,16,0,0,0,56,40V64H40A16,16,0,0,0,24,80v96a16,16,0,0,0,16,16H56v24a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24Zm-40,80h40v48H160ZM72,40H200V88H160V80a16,16,0,0,0-16-16H72ZM40,80H144v79.83c0,.06,0,.11,0,.17s0,.11,0,.17V176H40ZM72,216V192h72a16,16,0,0,0,16-16v-8h40v48Zm-3.76-62.06-12-48a8,8,0,1,1,15.52-3.88l6.76,27,6.32-12.66a8,8,0,0,1,14.32,0l6.32,12.66,6.76-27a8,8,0,0,1,15.52,3.88l-12,48a8,8,0,0,1-6.89,6,8.46,8.46,0,0,1-.87.05,8,8,0,0,1-7.16-4.42L92,137.89l-8.84,17.69a8,8,0,0,1-14.92-1.64Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M200,24H72A16,16,0,0,0,56,40V64H40A16,16,0,0,0,24,80v96a16,16,0,0,0,16,16H56v24a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24ZM68.24,153.94l-12-48a8,8,0,1,1,15.52-3.88l6.76,27,6.32-12.66a8,8,0,0,1,14.32,0l6.32,12.66,6.76-27a8,8,0,0,1,15.52,3.88l-12,48a8,8,0,0,1-6.89,6,8.46,8.46,0,0,1-.87.05,8,8,0,0,1-7.16-4.42L92,137.89l-8.84,17.69a8,8,0,0,1-14.92-1.64ZM200,216H72V192h72a16,16,0,0,0,16-16v-8h40Zm0-64H160V104h40Zm0-64H160V80a16,16,0,0,0-16-16H72V40H200Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M200,26H72A14,14,0,0,0,58,40V66H40A14,14,0,0,0,26,80v96a14,14,0,0,0,14,14H58v26a14,14,0,0,0,14,14H200a14,14,0,0,0,14-14V40A14,14,0,0,0,200,26Zm-42,76h44v52H158ZM70,40a2,2,0,0,1,2-2H200a2,2,0,0,1,2,2V90H158V80a14,14,0,0,0-14-14H70ZM38,176V80a2,2,0,0,1,2-2H144a2,2,0,0,1,2,2v96a2,2,0,0,1-2,2H40A2,2,0,0,1,38,176Zm162,42H72a2,2,0,0,1-2-2V190h74a14,14,0,0,0,14-14V166h44v50A2,2,0,0,1,200,218ZM70.18,153.46l-12-48a6,6,0,1,1,11.64-2.92l8.07,32.27,8.74-17.49a6,6,0,0,1,10.74,0l8.74,17.49,8.07-32.27a6,6,0,1,1,11.64,2.92l-12,48a6,6,0,0,1-5.17,4.5,4.63,4.63,0,0,1-.65,0,6,6,0,0,1-5.37-3.32L92,133.42,81.37,154.68a6,6,0,0,1-11.19-1.22Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M200,24H72A16,16,0,0,0,56,40V64H40A16,16,0,0,0,24,80v96a16,16,0,0,0,16,16H56v24a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24Zm-40,80h40v48H160ZM72,40H200V88H160V80a16,16,0,0,0-16-16H72ZM40,80H144v79.83c0,.06,0,.11,0,.17s0,.11,0,.17V176H40ZM72,216V192h72a16,16,0,0,0,16-16v-8h40v48Zm-3.76-62.06-12-48a8,8,0,1,1,15.52-3.88l6.76,27,6.32-12.66a8,8,0,0,1,14.32,0l6.32,12.66,6.76-27a8,8,0,0,1,15.52,3.88l-12,48a8,8,0,0,1-6.89,6,8.46,8.46,0,0,1-.87.05,8,8,0,0,1-7.16-4.42L92,137.89l-8.84,17.69a8,8,0,0,1-14.92-1.64Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M200,28H72A12,12,0,0,0,60,40V68H40A12,12,0,0,0,28,80v96a12,12,0,0,0,12,12H60v28a12,12,0,0,0,12,12H200a12,12,0,0,0,12-12V40A12,12,0,0,0,200,28Zm-44,72h48v56H156ZM68,40a4,4,0,0,1,4-4H200a4,4,0,0,1,4,4V92H156V80a12,12,0,0,0-12-12H68ZM36,176V80a4,4,0,0,1,4-4H144a4,4,0,0,1,4,4v96a4,4,0,0,1-4,4H40A4,4,0,0,1,36,176Zm164,44H72a4,4,0,0,1-4-4V188h76a12,12,0,0,0,12-12V164h48v52A4,4,0,0,1,200,220ZM72.12,153l-12-48A4,4,0,1,1,67.88,103l9.38,37.51,11.16-22.33a4,4,0,0,1,7.16,0l11.16,22.33L116.12,103a4,4,0,0,1,7.76,1.94l-12,48a4,4,0,0,1-3.44,3l-.44,0a4,4,0,0,1-3.58-2.21L92,128.94,79.58,153.79a4,4,0,0,1-7.46-.82Z"/> }.into_view()
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
