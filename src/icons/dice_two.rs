//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn DiceTwo(
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
            IconWeight::Bold => view!{ <path d="M192,28H64A36,36,0,0,0,28,64V192a36,36,0,0,0,36,36H192a36,36,0,0,0,36-36V64A36,36,0,0,0,192,28Zm12,164a12,12,0,0,1-12,12H64a12,12,0,0,1-12-12V64A12,12,0,0,1,64,52H192a12,12,0,0,1,12,12Zm-84-88a16,16,0,1,1-16-16A16,16,0,0,1,120,104Zm48,48a16,16,0,1,1-16-16A16,16,0,0,1,168,152Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,64V192a24,24,0,0,1-24,24H64a24,24,0,0,1-24-24V64A24,24,0,0,1,64,40H192A24,24,0,0,1,216,64Z" opacity="0.2"/><path d="M192,32H64A32,32,0,0,0,32,64V192a32,32,0,0,0,32,32H192a32,32,0,0,0,32-32V64A32,32,0,0,0,192,32Zm16,160a16,16,0,0,1-16,16H64a16,16,0,0,1-16-16V64A16,16,0,0,1,64,48H192a16,16,0,0,1,16,16Zm-88-84a12,12,0,1,1-12-12A12,12,0,0,1,120,108Zm40,40a12,12,0,1,1-12-12A12,12,0,0,1,160,148Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M192,32H64A32,32,0,0,0,32,64V192a32,32,0,0,0,32,32H192a32,32,0,0,0,32-32V64A32,32,0,0,0,192,32Zm-84,88a12,12,0,1,1,12-12A12,12,0,0,1,108,120Zm40,40a12,12,0,1,1,12-12A12,12,0,0,1,148,160Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M192,34H64A30,30,0,0,0,34,64V192a30,30,0,0,0,30,30H192a30,30,0,0,0,30-30V64A30,30,0,0,0,192,34Zm18,158a18,18,0,0,1-18,18H64a18,18,0,0,1-18-18V64A18,18,0,0,1,64,46H192a18,18,0,0,1,18,18Zm-92-84a10,10,0,1,1-10-10A10,10,0,0,1,118,108Zm40,40a10,10,0,1,1-10-10A10,10,0,0,1,158,148Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M192,32H64A32,32,0,0,0,32,64V192a32,32,0,0,0,32,32H192a32,32,0,0,0,32-32V64A32,32,0,0,0,192,32Zm16,160a16,16,0,0,1-16,16H64a16,16,0,0,1-16-16V64A16,16,0,0,1,64,48H192a16,16,0,0,1,16,16Zm-88-84a12,12,0,1,1-12-12A12,12,0,0,1,120,108Zm40,40a12,12,0,1,1-12-12A12,12,0,0,1,160,148Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M192,36H64A28,28,0,0,0,36,64V192a28,28,0,0,0,28,28H192a28,28,0,0,0,28-28V64A28,28,0,0,0,192,36Zm20,156a20,20,0,0,1-20,20H64a20,20,0,0,1-20-20V64A20,20,0,0,1,64,44H192a20,20,0,0,1,20,20Zm-96-84a8,8,0,1,1-8-8A8,8,0,0,1,116,108Zm40,40a8,8,0,1,1-8-8A8,8,0,0,1,156,148Z"/> }.into_view()
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
