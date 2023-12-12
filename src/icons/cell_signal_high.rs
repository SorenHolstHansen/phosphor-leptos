use crate::IconWeight;
/// GENERATED FILE
use leptos::*;

#[component]
pub fn CellSignalHigh(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M172,72V200a12,12,0,0,1-24,0V72a12,12,0,0,1,24,0Zm-52,28a12,12,0,0,0-12,12v88a12,12,0,0,0,24,0V112A12,12,0,0,0,120,100ZM80,140a12,12,0,0,0-12,12v48a12,12,0,0,0,24,0V152A12,12,0,0,0,80,140ZM40,180a12,12,0,0,0-12,12v8a12,12,0,0,0,24,0v-8A12,12,0,0,0,40,180Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M160,60.7V208H32a8,8,0,0,1-5.66-13.66Z" opacity="0.2"/><path d="M198.12,25.23a16,16,0,0,0-17.44,3.46l-160,160A16,16,0,0,0,32,216H192a16,16,0,0,0,16-16V40A15.94,15.94,0,0,0,198.12,25.23ZM152,80V200H32Zm40,120H168V64l24-24Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M198.12,25.23a16,16,0,0,0-17.44,3.46l-160,160A16,16,0,0,0,32,216H192a16,16,0,0,0,16-16V40A15.94,15.94,0,0,0,198.12,25.23ZM192,200H168V64l24-24Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M166,72V200a6,6,0,0,1-12,0V72a6,6,0,0,1,12,0Zm-46,34a6,6,0,0,0-6,6v88a6,6,0,0,0,12,0V112A6,6,0,0,0,120,106ZM80,146a6,6,0,0,0-6,6v48a6,6,0,0,0,12,0V152A6,6,0,0,0,80,146ZM40,186a6,6,0,0,0-6,6v8a6,6,0,0,0,12,0v-8A6,6,0,0,0,40,186Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M168,72V200a8,8,0,0,1-16,0V72a8,8,0,0,1,16,0Zm-48,32a8,8,0,0,0-8,8v88a8,8,0,0,0,16,0V112A8,8,0,0,0,120,104ZM80,144a8,8,0,0,0-8,8v48a8,8,0,0,0,16,0V152A8,8,0,0,0,80,144ZM40,184a8,8,0,0,0-8,8v8a8,8,0,0,0,16,0v-8A8,8,0,0,0,40,184Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M164,72V200a4,4,0,0,1-8,0V72a4,4,0,0,1,8,0Zm-44,36a4,4,0,0,0-4,4v88a4,4,0,0,0,8,0V112A4,4,0,0,0,120,108ZM80,148a4,4,0,0,0-4,4v48a4,4,0,0,0,8,0V152A4,4,0,0,0,80,148ZM40,188a4,4,0,0,0-4,4v8a4,4,0,0,0,8,0v-8A4,4,0,0,0,40,188Z"/> }.into_view()
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
