//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "arrows"))]
#[component]
pub fn ArrowElbowRight(
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
                <path d="M240,80v72a8,8,0,0,1-13.66,5.66L196,127.31l-70.34,70.35a8,8,0,0,1-11.32,0l-96-96A8,8,0,0,1,29.66,90.34L120,180.69,184.69,116,154.34,85.66A8,8,0,0,1,160,72h72A8,8,0,0,1,240,80Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M232,80v72L160,80Z" opacity="0.2"></path>
    <path d="M232,72H160a8,8,0,0,0-5.66,13.66L184.69,116,120,180.69,29.66,90.34a8,8,0,0,0-11.32,11.32l96,96a8,8,0,0,0,11.32,0L196,127.31l30.34,30.35A8,8,0,0,0,240,152V80A8,8,0,0,0,232,72Zm-8,60.69L179.31,88H224Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M236,80v72a4,4,0,0,1-8,0V89.66L122.83,194.83a4,4,0,0,1-5.66,0l-96-96a4,4,0,0,1,5.66-5.66L120,186.34,222.34,84H160a4,4,0,0,1,0-8h72A4,4,0,0,1,236,80Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,80v72a12,12,0,0,1-24,0V109l-91.51,91.52a12,12,0,0,1-17,0l-96-96a12,12,0,0,1,17-17L120,175l83-83H160a12,12,0,0,1,0-24h72A12,12,0,0,1,244,80Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M238,80v72a6,6,0,0,1-12,0V94.48L124.24,196.24a6,6,0,0,1-8.48,0l-96-96a6,6,0,0,1,8.48-8.48L120,183.51,217.52,86H160a6,6,0,0,1,0-12h72A6,6,0,0,1,238,80Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,80v72a8,8,0,0,1-16,0V99.31l-98.34,98.35a8,8,0,0,1-11.32,0l-96-96A8,8,0,0,1,29.66,90.34L120,180.69,212.69,88H160a8,8,0,0,1,0-16h72A8,8,0,0,1,240,80Z"></path>
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
            id=move || id.get().map(|id| id.get())
            class=move || class.get().map(|cls| cls.get())
        >
            {body}
        </svg>
    }
}
