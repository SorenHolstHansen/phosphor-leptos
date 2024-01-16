//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn RocketLaunch(
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
            IconWeight::Bold => view!{ <path d="M227.85,46.89a20,20,0,0,0-18.74-18.74c-13.13-.77-46.65.42-74.48,28.24L131,60H74.36a19.83,19.83,0,0,0-14.14,5.86L25.87,100.19a20,20,0,0,0,11.35,33.95l37.14,5.18,42.32,42.32,5.19,37.18A19.88,19.88,0,0,0,135.34,235a20.13,20.13,0,0,0,6.37,1,19.9,19.9,0,0,0,14.1-5.87l34.34-34.35A19.85,19.85,0,0,0,196,181.64V125l3.6-3.59C227.43,93.54,228.62,60,227.85,46.89ZM76,84h31L75.75,115.28l-27.23-3.8ZM151.6,73.37A72.27,72.27,0,0,1,204,52a72.17,72.17,0,0,1-21.38,52.41L128,159,97,128ZM172,180l-27.49,27.49-3.8-27.23L172,149Zm-71.67,21.62C91.67,213.65,74.16,228,40,228a12,12,0,0,1-12-12c0-34.15,14.35-51.66,26.39-60.33a12,12,0,0,1,14,19.47c-6,4.31-12.89,12.5-15.42,27.87,15.37-2.53,23.56-9.44,27.87-15.42a12,12,0,1,1,19.47,14Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M72,160l24,24s-8,32-56,32C40,168,72,160,72,160Zm64-88H74.35a8,8,0,0,0-5.65,2.34L34.35,108.69a8,8,0,0,0,4.53,13.57L80,128Zm-8,104,5.74,41.12a8,8,0,0,0,13.57,4.53l34.35-34.35a8,8,0,0,0,2.34-5.65V120Z" opacity="0.2"/><path d="M103.77,185.94C103.38,187.49,93.63,224,40,224a8,8,0,0,1-8-8c0-53.63,36.51-63.38,38.06-63.77a8,8,0,0,1,3.88,15.53c-.9.25-22.42,6.54-25.56,39.86C81.7,204.48,88,183,88.26,182a8,8,0,0,1,15.51,4Zm93-67.4L192,123.31v58.33A15.91,15.91,0,0,1,187.32,193L153,227.3A15.91,15.91,0,0,1,141.7,232a16.11,16.11,0,0,1-5.1-.83,15.94,15.94,0,0,1-10.78-12.92l-5.37-38.49L76.24,135.55l-38.47-5.37A16,16,0,0,1,28.7,103L63,68.68A15.91,15.91,0,0,1,74.36,64h58.33l4.77-4.77c26.68-26.67,58.83-27.82,71.41-27.07a16,16,0,0,1,15,15C224.6,59.71,223.45,91.86,196.78,118.54ZM40,114.34l37.15,5.18L116.69,80H74.36ZM91.32,128,128,164.68l57.45-57.45a76.46,76.46,0,0,0,22.42-59.16,76.64,76.64,0,0,0-59.11,22.47ZM176,139.31l-39.53,39.53L141.67,216,176,181.64Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M103.77,185.94C103.38,187.49,93.63,224,40,224a8,8,0,0,1-8-8c0-53.63,36.51-63.38,38.06-63.77a8,8,0,0,1,3.88,15.53c-.9.25-22.42,6.54-25.56,39.86C81.7,204.48,88,183,88.26,182a8,8,0,0,1,15.51,4Zm93-67.4L192,123.31v58.33A15.91,15.91,0,0,1,187.32,193L153,227.3A15.91,15.91,0,0,1,141.7,232a16.11,16.11,0,0,1-5.1-.83,15.94,15.94,0,0,1-10.78-12.92l-5.37-38.49L76.24,135.55l-38.47-5.37A16,16,0,0,1,28.7,103L63,68.68A15.91,15.91,0,0,1,74.36,64h58.33l4.77-4.77c26.68-26.67,58.82-27.82,71.41-27.07a16,16,0,0,1,15,15C224.6,59.71,223.45,91.86,196.78,118.54ZM116.69,80H74.36L40,114.34l37.15,5.18ZM176,139.31l-39.53,39.53L141.67,216,176,181.64Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M101.83,185.45C101.45,187,92.09,222,40,222a6,6,0,0,1-6-6c0-52.09,35.06-61.45,36.55-61.82a6,6,0,0,1,2.91,11.64c-1,.27-24.84,7.08-27.26,44,37.34-2.47,43.92-27,44-27.26a6,6,0,0,1,11.65,2.91Zm93.53-68.32h0L190,122.48v59.17a13.91,13.91,0,0,1-4.1,9.89l-34.35,34.35A14,14,0,0,1,127.8,218l-5.46-39.15L77.18,133.66,38.05,128.2a14,14,0,0,1-7.94-23.76L64.46,70.09A13.9,13.9,0,0,1,74.35,66h59.17l5.35-5.35c26.11-26.11,57.57-27.23,69.88-26.5a14,14,0,0,1,13.11,13.1C222.59,59.56,221.46,91,195.36,117.13Zm-155.63-.81,38.14,5.32L121.52,78H74.35a2,2,0,0,0-1.41.59L38.6,112.92a2,2,0,0,0-.49,2A1.93,1.93,0,0,0,39.73,116.32ZM178,134.48l-43.65,43.65,5.32,38.16a2,2,0,0,0,3.4,1.11l34.34-34.34a2,2,0,0,0,.59-1.41ZM209.88,48a2,2,0,0,0-1.83-1.83,78.47,78.47,0,0,0-60.69,23L88.49,128,128,167.51l51.76-51.75h0l7.11-7.11h0A78.47,78.47,0,0,0,209.88,48Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M103.77,185.94C103.38,187.49,93.63,224,40,224a8,8,0,0,1-8-8c0-53.63,36.51-63.38,38.06-63.77a8,8,0,0,1,3.88,15.53c-.9.25-22.42,6.54-25.56,39.86C81.7,204.48,88,183,88.26,182a8,8,0,0,1,15.51,4Zm93-67.4L192,123.31v58.33A15.91,15.91,0,0,1,187.32,193L153,227.3A15.91,15.91,0,0,1,141.7,232a16.11,16.11,0,0,1-5.1-.83,15.94,15.94,0,0,1-10.78-12.92l-5.37-38.49L76.24,135.55l-38.47-5.37A16,16,0,0,1,28.7,103L63,68.68A15.91,15.91,0,0,1,74.36,64h58.33l4.77-4.77c26.68-26.67,58.83-27.82,71.41-27.07a16,16,0,0,1,15,15C224.6,59.71,223.45,91.86,196.78,118.54ZM40,114.34l37.15,5.18L116.69,80H74.36ZM91.32,128,128,164.68l57.45-57.45a76.46,76.46,0,0,0,22.42-59.16,76.65,76.65,0,0,0-59.11,22.47ZM176,139.31l-39.53,39.53L141.67,216,176,181.64Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M99.88,185c-.35,1.43-9.33,35-59.88,35a4,4,0,0,1-4-4c0-50.54,33.6-59.53,35-59.88A4,4,0,0,1,73,163.88c-1.12.29-27.23,7.59-28.89,48,40.45-1.66,47.75-27.76,48.05-28.91a4,4,0,0,1,7.75,2ZM194,115.71,188,121.65v60a11.9,11.9,0,0,1-3.52,8.48l-34.34,34.35a12,12,0,0,1-20.37-6.79l-5.55-39.81L78.12,131.77l-39.79-5.55a12,12,0,0,1-6.81-20.37L65.87,71.51A11.9,11.9,0,0,1,74.35,68h60l5.94-5.94c25.53-25.53,56.3-26.62,68.35-25.91a12,12,0,0,1,11.22,11.22C220.57,59.41,219.48,90.18,194,115.71ZM39.44,118.3l39.14,5.46L126.35,76h-52a4,4,0,0,0-2.82,1.18L37.18,111.51a4,4,0,0,0,2.26,6.79ZM180,129.65l-47.77,47.77,5.46,39.15a4,4,0,0,0,6.79,2.25l34.35-34.34a4,4,0,0,0,1.17-2.83Zm8.28-19.59c23.23-23.24,24.23-51.26,23.58-62.22a3.93,3.93,0,0,0-3.71-3.71c-11-.65-39,.34-62.22,23.58l-7.11,7.11h0L85.66,128,128,170.34l53.18-53.17h0Z"/> }.into_view()
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
