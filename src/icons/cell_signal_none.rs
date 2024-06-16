//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn CellSignalNone(
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
                <path d="M198.12,25.23a16,16,0,0,0-17.44,3.46l-160,160A16,16,0,0,0,32,216H192a16,16,0,0,0,16-16V40A15.94,15.94,0,0,0,198.12,25.23ZM192,200H32L192,40Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M198.12,25.23a16,16,0,0,0-17.43,3.47l-160,160A16,16,0,0,0,32,216H192a16,16,0,0,0,16-16V40A16,16,0,0,0,198.12,25.23ZM192,200H32L192,40Z"></path>
}.into_view(),
IconWeight::Thin => view! { <path d="M44,192v8a4,4,0,0,1-8,0v-8a4,4,0,0,1,8,0Z"></path> }.into_view(),
IconWeight::Bold => view! { <path d="M52,192v8a12,12,0,0,1-24,0v-8a12,12,0,0,1,24,0Z"></path> }.into_view(),
IconWeight::Light => view! { <path d="M46,192v8a6,6,0,0,1-12,0v-8a6,6,0,0,1,12,0Z"></path> }.into_view(),
IconWeight::Regular => view! { <path d="M48,192v8a8,8,0,0,1-16,0v-8a8,8,0,0,1,16,0Z"></path> }.into_view()
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
