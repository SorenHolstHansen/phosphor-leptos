//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn PaintBrushHousehold(
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
            IconWeight::Bold => view!{ <path d="M233.47,22.53a36,36,0,0,0-50.91,0c-.16.15-.3.31-.45.47L130,81l-6.22-6.21a28,28,0,0,0-39.6,0L7.52,151.51a12,12,0,0,0,0,17l80,80a12,12,0,0,0,17,0l76.7-76.7a28,28,0,0,0,0-39.6L175,126l58-52.08c.16-.15.32-.29.47-.45A36,36,0,0,0,233.47,22.53ZM96,223,85,212l19.51-19.52a12,12,0,0,0-17-17L68,195l-7-7,19.51-19.52a12,12,0,0,0-17-17L44,171,33,160l39-39,63,63ZM216.69,56.28l-60.9,54.65a20,20,0,0,0-.78,29l9.2,9.2a4,4,0,0,1,0,5.66L152,167,89,104l12.2-12.2a4,4,0,0,1,5.66,0L116,101a20,20,0,0,0,29-.78l54.65-60.9a12,12,0,0,1,17,17Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M163.48,131.48l9.21,9.21a16,16,0,0,1,0,22.62L152,184,72,104,92.69,83.31a16,16,0,0,1,22.62,0l9.21,9.21a8,8,0,0,0,11.61-.32L191,31A24,24,0,0,1,225,65l-61.17,54.9A8,8,0,0,0,163.48,131.48Z" opacity="0.2"/><path d="M230.64,25.36a32,32,0,0,0-45.26,0c-.1.1-.2.2-.29.31L130.18,86.85,121,77.64a24,24,0,0,0-33.95,0l-76.69,76.7a8,8,0,0,0,0,11.31l80,80a8,8,0,0,0,11.31,0L178.36,169a24,24,0,0,0,0-33.95l-9.21-9.2,61.18-54.91a2.91,2.91,0,0,0,.31-.3A32,32,0,0,0,230.64,25.36ZM96,228.69,79.32,212l22.34-22.35a8,8,0,0,0-11.31-11.31L68,200.68,55.32,188l22.34-22.35a8,8,0,0,0-11.31-11.31L44,176.68,27.31,160,72,115.31,140.69,184ZM219.46,59.16l-61,54.75a16,16,0,0,0-.62,23.22l9.2,9.21a8,8,0,0,1,0,11.31l-15,15L83.32,104l15-15a8,8,0,0,1,11.31,0l9.21,9.2a16,16,0,0,0,23.22-.62h0l54.75-61a16,16,0,0,1,22.62,22.62Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M230.64,25.36a32,32,0,0,0-45.26,0c-.1.1-.2.2-.29.31L130.18,86.85,121,77.64a24,24,0,0,0-33.95,0l-76.69,76.7a8,8,0,0,0,0,11.31l80,80a8,8,0,0,0,11.31,0L178.36,169a24,24,0,0,0,0-33.95l-9.21-9.2,61.18-54.91a2.91,2.91,0,0,0,.31-.3A32,32,0,0,0,230.64,25.36ZM96,228.69,79.32,212l22.34-22.35a8,8,0,0,0-11.31-11.31L68,200.68,55.32,188l22.34-22.35a8,8,0,0,0-11.31-11.31L44,176.68,27.31,160,72,115.31,140.69,184Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M229.23,26.77a30.05,30.05,0,0,0-42.43,0l-.23.24-54.9,61.18a2,2,0,0,1-1.44.66,1.93,1.93,0,0,1-1.46-.59l-9.21-9.2a22,22,0,0,0-31.11,0L11.76,155.75a6,6,0,0,0,0,8.49l80,80a6,6,0,0,0,8.49,0l76.69-76.69a22,22,0,0,0,0-31.11l-9.2-9.21a2,2,0,0,1-.59-1.46,2,2,0,0,1,.66-1.44L229,69.43l.24-.23A30.05,30.05,0,0,0,229.23,26.77ZM96,231.51,76.49,212l23.76-23.76a6,6,0,0,0-8.49-8.49L68,203.51,52.49,188l23.76-23.76a6,6,0,0,0-8.49-8.49L44,179.51,24.49,160,72,112.48,143.52,184ZM220.85,60.61l-61,54.79a14,14,0,0,0-.55,20.32l9.2,9.2a10,10,0,0,1,0,14.15L152,175.51,80.49,104,96.93,87.55a10,10,0,0,1,14.15,0l9.2,9.2a14,14,0,0,0,20.32-.55h0l54.79-61a18,18,0,0,1,25.46,25.46Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M230.64,25.36a32,32,0,0,0-45.26,0c-.1.1-.2.2-.29.31L130.18,86.85,121,77.64a24,24,0,0,0-33.95,0l-76.69,76.7a8,8,0,0,0,0,11.31l80,80a8,8,0,0,0,11.31,0L178.36,169a24,24,0,0,0,0-33.95l-9.21-9.2,61.18-54.91a2.91,2.91,0,0,0,.31-.3A32,32,0,0,0,230.64,25.36ZM96,228.69,79.32,212l22.34-22.35a8,8,0,0,0-11.31-11.31L68,200.68,55.32,188l22.34-22.35a8,8,0,0,0-11.31-11.31L44,176.68,27.31,160,72,115.31,140.69,184ZM219.46,59.16l-61,54.75a16,16,0,0,0-.62,23.22l9.2,9.21a8,8,0,0,1,0,11.31l-15,15L83.32,104l15-15a8,8,0,0,1,11.31,0l9.21,9.2a16,16,0,0,0,23.22-.62h0l54.75-61a16,16,0,0,1,22.62,22.62Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M227.8,28.2a28,28,0,0,0-39.6,0l-.15.16-54.9,61.17a4,4,0,0,1-5.81.16l-9.2-9.2a20,20,0,0,0-28.28,0L13.17,157.17a4,4,0,0,0,0,5.66l80,80a4,4,0,0,0,5.66,0l76.68-76.69a20,20,0,0,0,0-28.28l-9.2-9.2a4,4,0,0,1-1.17-2.94,4,4,0,0,1,1.33-2.87L227.64,68l.16-.15A28,28,0,0,0,227.8,28.2ZM96,234.34,73.66,212l25.17-25.17a4,4,0,0,0-5.66-5.66L68,206.34,49.66,188l25.17-25.17a4,4,0,0,0-5.66-5.66L44,182.34,21.66,160,72,109.66,146.34,184ZM222.21,62.07,161.12,116.9a12,12,0,0,0-.46,17.41l9.2,9.2a12,12,0,0,1,0,17L152,178.34,77.66,104,95.51,86.14a12,12,0,0,1,17,0l9.2,9.2a12,12,0,0,0,17.41-.46l54.83-61.09a20,20,0,0,1,28.28,28.28Z"/> }.into_view()
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
