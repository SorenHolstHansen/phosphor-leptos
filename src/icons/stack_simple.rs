//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "office", feature = "editor"))]
#[component]
pub fn StackSimple(
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
                <path d="M10.05,110.42l112,64a12,12,0,0,0,11.9,0l112-64a12,12,0,0,0,0-20.84l-112-64a12,12,0,0,0-11.9,0l-112,64a12,12,0,0,0,0,20.84Zm118-60.6L215.81,100,128,150.18,40.19,100Zm122.42,92.23A12,12,0,0,1,246,158.42l-112,64a12,12,0,0,1-11.9,0l-112-64A12,12,0,1,1,22,137.58l106,60.6,106-60.6A12,12,0,0,1,250.42,142.05Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M240,104,128,168,16,104,128,40Z" opacity="0.2"></path>
    <path d="M12,111l112,64a8,8,0,0,0,7.94,0l112-64a8,8,0,0,0,0-13.9l-112-64a8,8,0,0,0-7.94,0l-112,64A8,8,0,0,0,12,111ZM128,49.21,223.87,104,128,158.79,32.13,104ZM247,140A8,8,0,0,1,244,151L132,215a8,8,0,0,1-7.94,0L12,151A8,8,0,1,1,20,137.05l108,61.74,108-61.74A8,8,0,0,1,247,140Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M12,111l112,64a8,8,0,0,0,7.94,0l112-64a8,8,0,0,0,0-13.9l-112-64a8,8,0,0,0-7.94,0l-112,64A8,8,0,0,0,12,111Z"></path>
    <path d="M236,137.05,128,198.79,20,137.05A8,8,0,1,0,12,151l112,64a8,8,0,0,0,7.94,0l112-64a8,8,0,1,0-7.94-13.9Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M13,109.21l112,64a6,6,0,0,0,6,0l112-64a6,6,0,0,0,0-10.42l-112-64a6,6,0,0,0-6,0l-112,64a6,6,0,0,0,0,10.42Zm115-62.3L227.91,104,128,161.09,28.09,104ZM245.21,141a6,6,0,0,1-2.23,8.19l-112,64a6,6,0,0,1-6,0l-112-64a6,6,0,0,1,6-10.42l109,62.3,109-62.3A6,6,0,0,1,245.21,141Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M12,111l112,64a8,8,0,0,0,7.94,0l112-64a8,8,0,0,0,0-13.9l-112-64a8,8,0,0,0-7.94,0l-112,64A8,8,0,0,0,12,111ZM128,49.21,223.87,104,128,158.79,32.13,104ZM246.94,140A8,8,0,0,1,244,151L132,215a8,8,0,0,1-7.94,0L12,151A8,8,0,0,1,20,137.05l108,61.74,108-61.74A8,8,0,0,1,246.94,140Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M14,107.47l112,64a4,4,0,0,0,4,0l112-64a4,4,0,0,0,0-6.94l-112-64a4,4,0,0,0-4,0l-112,64a4,4,0,0,0,0,6.94ZM128,44.61,231.94,104,128,163.39,24.06,104ZM243.47,142a4,4,0,0,1-1.49,5.45l-112,64a4,4,0,0,1-4,0l-112-64a4,4,0,0,1,4-6.94l110,62.86,110-62.86A4,4,0,0,1,243.47,142Z"></path>
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
