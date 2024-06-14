//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce"))]
#[component]
pub fn TrademarkRegistered(
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
                <path d="M152,112a16,16,0,0,1-16,16H112V96h24A16,16,0,0,1,152,112Zm80,16A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-16,0a88,88,0,1,0-88,88A88.1,88.1,0,0,0,216,128Zm-16,0a72,72,0,1,1-72-72A72.08,72.08,0,0,1,200,128Zm-33.34,35.56-15.57-23.35A32,32,0,0,0,136,80H104a8,8,0,0,0-8,8v80a8,8,0,0,0,16,0V144h22.39l19,28.44a8,8,0,0,0,13.32-8.88Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"></path>
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm23.09-75.79A32,32,0,0,0,136,80H104a8,8,0,0,0-8,8v80a8,8,0,0,0,16,0V144h22.39l19,28.44a8,8,0,0,0,13.32-8.88ZM112,96h24a16,16,0,0,1,0,32H112Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,192a92,92,0,1,1,92-92A92.1,92.1,0,0,1,128,220Zm17.12-81.53A28,28,0,0,0,136,84H104a4,4,0,0,0-4,4v80a4,4,0,0,0,8,0V140h28.52l20.15,30.23a4,4,0,0,0,6.66-4.44ZM108,92h28a20,20,0,0,1,0,40H108Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,192a84,84,0,1,1,84-84A84.09,84.09,0,0,1,128,212Zm28.7-70.58A36,36,0,0,0,136,76H104A12,12,0,0,0,92,88v80a12,12,0,0,0,24,0V148h16.24L150,174.66a12,12,0,0,0,20-13.32ZM116,100h20a12,12,0,0,1,0,24H116Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218Zm20.16-78.58A30,30,0,0,0,136,82H104a6,6,0,0,0-6,6v80a6,6,0,0,0,12,0V142h25.46L155,171.33a6,6,0,1,0,10-6.66ZM110,94h26a18,18,0,0,1,0,36H110Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm23.09-75.79A32,32,0,0,0,136,80H104a8,8,0,0,0-8,8v80a8,8,0,0,0,16,0V144h22.39l19,28.44a8,8,0,0,0,13.32-8.88ZM112,96h24a16,16,0,0,1,0,32H112Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size
            height=height
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=id
            class=class
        >
            {body}
        </svg>
    }
}
