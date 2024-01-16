//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Footprints(
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
            IconWeight::Bold => view!{ <path d="M212,180H156a12,12,0,0,0-12,12v12a40,40,0,0,0,80,0V192A12,12,0,0,0,212,180Zm-28,40a16,16,0,0,1-16-16h32A16,16,0,0,1,184,220Zm-84-64H44a12,12,0,0,0-12,12v12a40,40,0,0,0,80,0V168A12,12,0,0,0,100,156ZM72,196a16,16,0,0,1-16-16H88A16,16,0,0,1,72,196ZM72,12c-14.06,0-27.7,12.33-37.41,33.83-12.45,27.57-16.78,67.52,3.14,90.11a12,12,0,0,0,9,4.06H97.25a12,12,0,0,0,9-4.06c19.92-22.59,15.59-62.54,3.14-90.11C99.68,24.33,86,12,72,12ZM91.07,116H52.9c-6.71-12.13-7.67-35.45,3.56-60.3C63.16,40.85,70.28,36,72,36s8.82,4.85,15.53,19.7C98.73,80.55,97.77,103.87,91.07,116Zm67.68,48h50.54a12,12,0,0,0,9-4.06c19.92-22.59,15.59-62.54,3.14-90.11C211.72,48.33,198.08,36,184,36s-27.7,12.33-37.41,33.83c-12.45,27.57-16.78,67.52,3.14,90.11A12,12,0,0,0,158.75,164Zm9.74-84.3C175.2,64.85,182.32,60,184,60s8.82,4.85,15.52,19.7c11.23,24.85,10.27,48.17,3.56,60.3H164.93C158.23,127.87,157.27,104.55,168.49,79.7Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M152,192h56v12a28,28,0,0,1-56,0ZM48,180a28,28,0,0,0,56,0V168H48ZM76,24c-24.52,0-51.46,80-25.26,112h50.52C127.46,104,100.52,24,76,24ZM205.26,160c26.2-32-.74-112-25.26-112s-51.46,80-25.26,112Z" opacity="0.2"/><path d="M208.06,184H152a8,8,0,0,0-8,8v12a36,36,0,0,0,72.05,0V192A8,8,0,0,0,208.06,184Zm-8,20a20,20,0,0,1-40,0v-4h40ZM104,160h-56a8,8,0,0,0-8,8v12A36,36,0,0,0,112,180V168A8,8,0,0,0,104,160Zm-8,20a20,20,0,0,1-40,0v-4H96ZM76,16C64.36,16,53.07,26.31,44.2,45c-13.93,29.38-18.56,73,.29,96a8,8,0,0,0,6.2,2.93h50.55a8,8,0,0,0,6.2-2.93c18.85-23,14.22-66.65.29-96C98.85,26.31,87.57,16,76,16ZM97.15,128H54.78c-11.4-18.1-7.21-52.7,3.89-76.11C65.14,38.22,72.17,32,76,32s10.82,6.22,17.3,19.89C104.36,75.3,108.55,109.9,97.15,128Zm57.61,40h50.55a8,8,0,0,0,6.2-2.93c18.85-23,14.22-66.65.29-96C202.93,50.31,191.64,40,180,40s-22.89,10.31-31.77,29c-13.93,29.38-18.56,73,.29,96A8,8,0,0,0,154.76,168Zm8-92.11C169.22,62.22,176.25,56,180,56s10.81,6.22,17.29,19.89c11.1,23.41,15.29,58,3.89,76.11H158.85C147.45,133.9,151.64,99.3,162.74,75.89Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M216.06,192v12A36,36,0,0,1,144,204V192a8,8,0,0,1,8-8h56A8,8,0,0,1,216.06,192ZM104,160h-56a8,8,0,0,0-8,8v12A36,36,0,0,0,112,180V168A8,8,0,0,0,104,160ZM76,16C64.36,16,53.07,26.31,44.2,45c-13.93,29.38-18.56,73,.29,96a8,8,0,0,0,6.2,2.93h50.55a8,8,0,0,0,6.2-2.93c18.85-23,14.22-66.65.29-96C98.85,26.31,87.57,16,76,16Zm78.8,152h50.55a8,8,0,0,0,6.2-2.93c18.85-23,14.22-66.65.29-96C202.93,50.31,191.64,40,180,40s-22.89,10.31-31.77,29c-13.93,29.38-18.56,73,.29,96A8,8,0,0,0,154.76,168Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M104,162H48a6,6,0,0,0-6,6v12a34,34,0,0,0,68,0V168A6,6,0,0,0,104,162Zm-6,18a22,22,0,0,1-44,0v-6H98ZM76,18C65.2,18,54.56,27.91,46,45.9c-13.66,28.82-18.29,71.53,0,93.9a6,6,0,0,0,4.65,2.2h50.53a6,6,0,0,0,4.65-2.2c18.32-22.37,13.69-65.08,0-93.9C97.41,27.91,86.77,18,76,18ZM98.23,130H53.74c-10.09-15.18-11.69-47.65,3.14-79C64.24,35.51,71.77,30,76,30s11.75,5.51,19.1,21C109.92,82.35,108.32,114.82,98.23,130ZM208,186H152a6,6,0,0,0-6,6v12a34,34,0,0,0,68,0V192A6,6,0,0,0,208,186Zm-6,18a22,22,0,0,1-44,0v-6h44Zm-47.27-38h50.53a6,6,0,0,0,4.65-2.2c18.32-22.37,13.69-65.08,0-93.9C201.44,51.91,190.8,42,180,42s-21.43,9.91-30,27.9c-13.66,28.82-18.29,71.53,0,93.9A6,6,0,0,0,154.75,166Zm6.17-91c7.35-15.53,14.88-21,19.1-21s11.74,5.51,19.1,21c14.83,31.31,13.23,63.78,3.14,79H157.77C147.68,138.82,146.08,106.35,160.92,75Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208.06,184H152a8,8,0,0,0-8,8v12a36,36,0,0,0,72.05,0V192A8,8,0,0,0,208.06,184Zm-8,20a20,20,0,0,1-40,0v-4h40ZM104,160h-56a8,8,0,0,0-8,8v12A36,36,0,0,0,112,180V168A8,8,0,0,0,104,160Zm-8,20a20,20,0,0,1-40,0v-4H96ZM76,16C64.36,16,53.07,26.31,44.2,45c-13.93,29.38-18.56,73,.29,96a8,8,0,0,0,6.2,2.93h50.55a8,8,0,0,0,6.2-2.93c18.85-23,14.22-66.65.29-96C98.85,26.31,87.57,16,76,16ZM97.15,128H54.78c-11.4-18.1-7.21-52.7,3.89-76.11C65.14,38.22,72.17,32,76,32s10.82,6.22,17.3,19.89C104.36,75.3,108.55,109.9,97.15,128Zm57.61,40h50.55a8,8,0,0,0,6.2-2.93c18.85-23,14.22-66.65.29-96C202.93,50.31,191.64,40,180,40s-22.89,10.31-31.77,29c-13.93,29.38-18.56,73,.29,96A8.05,8.05,0,0,0,154.76,168Zm8-92.11C169.22,62.22,176.25,56,180,56s10.82,6.22,17.29,19.89c11.1,23.41,15.29,58,3.9,76.11H158.85C147.45,133.9,151.64,99.3,162.74,75.89Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M208,188H152a4,4,0,0,0-4,4v12a32,32,0,1,0,64,0V192A4,4,0,0,0,208,188Zm-4,16a24,24,0,1,1-48,0v-8h48ZM104,164H48a4,4,0,0,0-4,4v12a32,32,0,1,0,64,0V168A4,4,0,0,0,104,164Zm-4,16a24,24,0,1,1-48,0v-8h48ZM76,20c-9.82,0-20.07,9.75-28.12,26.75-13.39,28.27-18,70.05-.23,91.78a4,4,0,0,0,3.1,1.47h50.51a4,4,0,0,0,3.1-1.47c17.79-21.73,13.16-63.51-.23-91.78C96.08,29.75,85.83,20,76,20ZM99.3,132H52.71c-13.8-19-9.79-56.08,2.4-81.82C61.59,36.5,69.59,28,76,28s14.42,8.5,20.9,22.18C109.09,75.92,113.1,113,99.3,132Zm55.44,32h50.51a4,4,0,0,0,3.1-1.47c17.79-21.73,13.16-63.51-.23-91.78C200.07,53.75,189.82,44,180,44s-20.08,9.75-28.13,26.75c-13.39,28.27-18,70.05-.23,91.78A4,4,0,0,0,154.74,164Zm4.36-89.82C165.58,60.5,173.58,52,180,52s14.41,8.5,20.89,22.18c12.19,25.74,16.2,62.82,2.4,81.82H156.7C142.9,137,146.91,99.92,159.1,74.18Z"/> }.into_view()
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
