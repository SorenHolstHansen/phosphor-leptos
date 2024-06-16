//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "development"))]
#[component]
pub fn LineVertical(
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
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM136,192a8,8,0,0,1-16,0V64a8,8,0,0,1,16,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,48V208a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32H208A16,16,0,0,1,224,48Z"
        opacity="0.2"
    ></path>
    <path d="M136,24V232a8,8,0,0,1-16,0V24a8,8,0,0,1,16,0Z"></path>
}.into_view(),
IconWeight::Thin => view! { <path d="M132,24V232a4,4,0,0,1-8,0V24a4,4,0,0,1,8,0Z"></path> }.into_view(),
IconWeight::Bold => view! { <path d="M140,24V232a12,12,0,0,1-24,0V24a12,12,0,0,1,24,0Z"></path> }.into_view(),
IconWeight::Light => view! { <path d="M134,24V232a6,6,0,0,1-12,0V24a6,6,0,0,1,12,0Z"></path> }.into_view(),
IconWeight::Regular => view! { <path d="M136,24V232a8,8,0,0,1-16,0V24a8,8,0,0,1,16,0Z"></path> }.into_view()
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
