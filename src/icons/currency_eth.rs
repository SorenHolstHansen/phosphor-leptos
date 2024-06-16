//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "finance"))]
#[component]
pub fn CurrencyEth(
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
                <path d="M222.29,123.06l-88-112a8,8,0,0,0-12.58,0l-88,112a8,8,0,0,0,0,9.88l88,112a8,8,0,0,0,12.58,0l88-112A8,8,0,0,0,222.29,123.06ZM136,155.58V39.13l67.42,85.8Zm-16,0L52.58,124.93,120,39.13Zm0,17.57v43.72l-53.43-68Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M216,128,128,240,40,128l88,40Z" opacity="0.2"></path>
    <path d="M222.29,123.06l-88-112a8,8,0,0,0-12.58,0l-88,112a8,8,0,0,0,0,9.88l88,112a8,8,0,0,0,12.58,0l88-112A8,8,0,0,0,222.29,123.06ZM136,39.13l67.42,85.8L136,155.58ZM120,155.58,52.58,124.93,120,39.13Zm0,17.57v43.72l-53.43-68Zm16,0,53.43-24.29-53.43,68Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M219.15,125.53l-88-112a4,4,0,0,0-6.3,0l-88,112a4,4,0,0,0,0,4.94l88,112a4,4,0,0,0,6.3,0l88-112A4,4,0,0,0,219.15,125.53ZM132,27.57l77.71,98.9L132,161.79Zm-8,134.22L46.29,126.47,124,27.57Zm0,8.79v57.85l-70.72-90Zm8,0,70.72-32.15-70.72,90Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M225.44,120.59l-88-112a12,12,0,0,0-18.88,0l-88,112a12,12,0,0,0,0,14.82l.6.76a3.72,3.72,0,0,0,.44.56l87,110.68a12,12,0,0,0,18.88,0l88-112A12,12,0,0,0,225.44,120.59ZM140,50.7l57.12,72.7-57.12,26Zm-24,98.66-57.12-26L116,50.7Zm0,26.37V205.3l-36.15-46Zm24,0,36.15-16.43L140,205.3Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M220.72,124.29l-88-112a6,6,0,0,0-9.44,0l-88,112a6,6,0,0,0,0,7.42l88,112a6,6,0,0,0,9.44,0l88-112A6,6,0,0,0,220.72,124.29ZM134,33.35l72.56,92.35-72.56,33ZM122,158.68l-72.56-33L122,33.35Zm0,13.18v50.79l-62.08-79Zm12,0,62.08-28.21-62.08,79Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M222.29,123.06l-88-112a8,8,0,0,0-12.58,0l-88,112a8,8,0,0,0,0,9.88l88,112a8,8,0,0,0,12.58,0l88-112A8,8,0,0,0,222.29,123.06ZM136,39.13l67.42,85.8L136,155.58ZM120,155.58,52.58,124.93,120,39.13Zm0,17.57v43.72l-53.43-68Zm16,0,53.43-24.29-53.43,68Z"></path>
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
