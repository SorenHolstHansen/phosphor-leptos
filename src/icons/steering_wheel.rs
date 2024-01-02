//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn SteeringWheel(
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
                <path d="M144,144a16,16,0,1,1-16-16A16,16,0,0,1,144,144Zm92-16A108,108,0,1,1,128,20,108.12,108.12,0,0,1,236,128Zm-70.45,28h41.63c.79-2.21,1.49-4.47,2.09-6.76a116,116,0,0,0-162.54,0q.9,3.44,2.09,6.76H90.45a20.07,20.07,0,0,1,18.73,13l16.06,42.93c.92,0,1.83.07,2.76.07s1.82,0,2.72-.07l16.1-43A20.09,20.09,0,0,1,165.55,156ZM44.41,119.73a139.85,139.85,0,0,1,167.18,0,84,84,0,0,0-167.18,0Zm53.08,86.51L87.68,180H62.1A84.46,84.46,0,0,0,97.49,206.24ZM193.9,180H168.32l-9.84,26.25A84.35,84.35,0,0,0,193.9,180Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M222.4,145.55A96.6,96.6,0,0,1,218.54,160h-53a8,8,0,0,0-7.5,5.19l-21.9,58.47c-2.69.22-5.41.34-8.15.34s-5.5-.12-8.2-.35L97.94,165.2a8,8,0,0,0-7.49-5.2h-53a96.6,96.6,0,0,1-3.86-14.45,128,128,0,0,1,188.8,0Z"
        opacity="0.2"
    ></path>
    <path d="M128,152a12,12,0,1,1,12-12A12,12,0,0,1,128,152Zm104-24A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128ZM40,128v.33a135.93,135.93,0,0,1,176,0V128a88,88,0,0,0-176,0Zm67.5,85.58L90.45,168H49.63A88.35,88.35,0,0,0,107.5,213.58ZM128,216c.83,0,1.66,0,2.49,0l20.07-53.57a16.07,16.07,0,0,1,15-10.39h47.12c.38-1.31.72-2.64,1-4a120,120,0,0,0-171.4,0c.31,1.34.65,2.67,1,4H90.45a16.08,16.08,0,0,1,15,10.4l20,53.56C126.31,216,127.15,216,128,216Zm78.37-48H165.55l-17.09,45.59A88.34,88.34,0,0,0,206.37,168Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24ZM49.63,168H90.45l17,45.58A88.35,88.35,0,0,1,49.63,168ZM128,156a16,16,0,1,1,16-16A16,16,0,0,1,128,156Zm20.46,57.59L165.55,168h40.82A88.34,88.34,0,0,1,148.46,213.59ZM128,96a136.38,136.38,0,0,0-88,32.33V128a88,88,0,0,1,176,0v.33A136.38,136.38,0,0,0,128,96Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26ZM46.43,166h44a2,2,0,0,1,1.87,1.3l18.33,49A90.3,90.3,0,0,1,46.43,166Zm98.87,50.32,18.37-49a2,2,0,0,1,1.88-1.3h44A90.29,90.29,0,0,1,145.3,216.32ZM214.17,154H165.55a14,14,0,0,0-13.11,9.09l-20.55,54.82c-1.29.06-2.59.09-3.89.09s-2.63,0-3.94-.09l-20.5-54.81A14.06,14.06,0,0,0,90.45,154H41.83c-.65-2.17-1.23-4.37-1.72-6.61a122,122,0,0,1,175.78,0C215.4,149.63,214.82,151.83,214.17,154ZM128,98a134.38,134.38,0,0,0-89.88,34.64C38,131.1,38,129.56,38,128a90,90,0,0,1,180,0c0,1.56,0,3.1-.12,4.64A134.38,134.38,0,0,0,128,98Zm10,42a10,10,0,1,1-10-10A10,10,0,0,1,138,140Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,152a12,12,0,1,1,12-12A12,12,0,0,1,128,152Zm104-24A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128ZM40,128v.33a135.93,135.93,0,0,1,176,0V128a88,88,0,0,0-176,0Zm67.5,85.58L90.45,168H49.63A88.35,88.35,0,0,0,107.5,213.58ZM128,216c.83,0,1.66,0,2.49,0l20.07-53.57a16.07,16.07,0,0,1,15-10.39h47.12c.38-1.31.72-2.64,1-4a120,120,0,0,0-171.4,0c.31,1.34.65,2.67,1,4H90.45a16.08,16.08,0,0,1,15,10.4l20,53.56C126.31,216,127.15,216,128,216Zm78.37-48H165.55l-17.09,45.59A88.34,88.34,0,0,0,206.37,168Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,148a8,8,0,1,1,8-8A8,8,0,0,1,128,148Zm100-20A100,100,0,1,1,128,28,100.11,100.11,0,0,1,228,128ZM36,128q0,4.53.44,8.94a131.94,131.94,0,0,1,183.12,0q.44-4.41.44-8.94a92,92,0,0,0-184,0Zm77.75,90.9L94.2,166.6a4,4,0,0,0-3.75-2.6H43.34A92.31,92.31,0,0,0,113.75,218.9ZM128,220c1.78,0,3.55-.06,5.3-.16l21-56.05A12,12,0,0,1,165.55,156h50.09a91.61,91.61,0,0,0,2.43-9.21,124,124,0,0,0-180.14,0A91.61,91.61,0,0,0,40.36,156H90.45a12,12,0,0,1,11.24,7.8l21,56C124.42,219.94,126.2,220,128,220Zm84.66-56H165.55a4,4,0,0,0-3.75,2.6l-19.6,52.3A92.26,92.26,0,0,0,212.66,164Z"></path>
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
