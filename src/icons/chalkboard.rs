//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "objects"))]
#[component]
pub fn Chalkboard(
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
                <path d="M240,192h-8V56a16,16,0,0,0-16-16H40A16,16,0,0,0,24,56V192H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16Zm-24,0H144V176a8,8,0,0,1,8-8h56a8,8,0,0,1,8,8Zm0-48a8,8,0,0,1-16,0V72H56V184a8,8,0,0,1-16,0V64a8,8,0,0,1,8-8H208a8,8,0,0,1,8,8Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M192,80v88H120v32H64V80Z" opacity="0.2"></path>
    <path d="M240,192h-8V56a16,16,0,0,0-16-16H40A16,16,0,0,0,24,56V192H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM40,56H216V192H200V168a8,8,0,0,0-8-8H120a8,8,0,0,0-8,8v24H72V88H184v48a8,8,0,0,0,16,0V80a8,8,0,0,0-8-8H64a8,8,0,0,0-8,8V192H40ZM184,192H128V176h56Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M240,196H228V56a12,12,0,0,0-12-12H40A12,12,0,0,0,28,56V196H16a4,4,0,0,0,0,8H240a4,4,0,0,0,0-8ZM36,56a4,4,0,0,1,4-4H216a4,4,0,0,1,4,4V196H196V168a4,4,0,0,0-4-4H120a4,4,0,0,0-4,4v28H68V84H188v52a4,4,0,0,0,8,0V80a4,4,0,0,0-4-4H64a4,4,0,0,0-4,4V196H36ZM188,196H124V172h64Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M240,188h-4V56a20,20,0,0,0-20-20H40A20,20,0,0,0,20,56V188H16a12,12,0,0,0,0,24H240a12,12,0,0,0,0-24ZM44,60H212V188H196V160a12,12,0,0,0-12-12H120a12,12,0,0,0-12,12v28H84V100h88v20a12,12,0,0,0,24,0V88a12,12,0,0,0-12-12H72A12,12,0,0,0,60,88V188H44ZM172,188H132V172h40Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M240,194H230V56a14,14,0,0,0-14-14H40A14,14,0,0,0,26,56V194H16a6,6,0,0,0,0,12H240a6,6,0,0,0,0-12ZM38,56a2,2,0,0,1,2-2H216a2,2,0,0,1,2,2V194H198V168a6,6,0,0,0-6-6H120a6,6,0,0,0-6,6v26H70V86H186v50a6,6,0,0,0,12,0V80a6,6,0,0,0-6-6H64a6,6,0,0,0-6,6V194H38ZM186,194H126V174h60Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,192h-8V56a16,16,0,0,0-16-16H40A16,16,0,0,0,24,56V192H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM40,56H216V192H200V168a8,8,0,0,0-8-8H120a8,8,0,0,0-8,8v24H72V88H184v48a8,8,0,0,0,16,0V80a8,8,0,0,0-8-8H64a8,8,0,0,0-8,8V192H40ZM184,192H128V176h56Z"></path>
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
