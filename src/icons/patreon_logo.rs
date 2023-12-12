//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn PatreonLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M170,36a73.68,73.68,0,0,0-50,19.53A20,20,0,0,0,100,36H64A20,20,0,0,0,44,56V208a20,20,0,0,0,20,20h36a20,20,0,0,0,20-20V164.5A74,74,0,1,0,170,36ZM96,204H68V60H96Zm74-44a50,50,0,1,1,50-50A50.06,50.06,0,0,1,170,160Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,104a56,56,0,1,1-56-56A56,56,0,0,1,232,104ZM80,48H64a8,8,0,0,0-8,8V208a8,8,0,0,0,8,8H80a8,8,0,0,0,8-8V56A8,8,0,0,0,80,48Z"
        opacity="0.2"
    ></path>
    <path d="M176,40a64,64,0,1,0,64,64A64.07,64.07,0,0,0,176,40Zm0,112a48,48,0,1,1,48-48A48.05,48.05,0,0,1,176,152ZM80,40H64A16,16,0,0,0,48,56V208a16,16,0,0,0,16,16H80a16,16,0,0,0,16-16V56A16,16,0,0,0,80,40Zm0,168H64V56H80V208Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M240,104a64,64,0,1,1-64-64A64.07,64.07,0,0,1,240,104ZM80,40H64A16,16,0,0,0,48,56V208a16,16,0,0,0,16,16H80a16,16,0,0,0,16-16V56A16,16,0,0,0,80,40Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M176,42a62,62,0,1,0,62,62A62.07,62.07,0,0,0,176,42Zm0,112a50,50,0,1,1,50-50A50.06,50.06,0,0,1,176,154ZM80,42H64A14,14,0,0,0,50,56V208a14,14,0,0,0,14,14H80a14,14,0,0,0,14-14V56A14,14,0,0,0,80,42Zm2,166a2,2,0,0,1-2,2H64a2,2,0,0,1-2-2V56a2,2,0,0,1,2-2H80a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M176,40a64,64,0,1,0,64,64A64.07,64.07,0,0,0,176,40Zm0,112a48,48,0,1,1,48-48A48.05,48.05,0,0,1,176,152ZM80,40H64A16,16,0,0,0,48,56V208a16,16,0,0,0,16,16H80a16,16,0,0,0,16-16V56A16,16,0,0,0,80,40Zm0,168H64V56H80V208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M176,44a60,60,0,1,0,60,60A60.07,60.07,0,0,0,176,44Zm0,112a52,52,0,1,1,52-52A52.06,52.06,0,0,1,176,156ZM80,44H64A12,12,0,0,0,52,56V208a12,12,0,0,0,12,12H80a12,12,0,0,0,12-12V56A12,12,0,0,0,80,44Zm4,164a4,4,0,0,1-4,4H64a4,4,0,0,1-4-4V56a4,4,0,0,1,4-4H80a4,4,0,0,1,4,4Z"></path>
}.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
