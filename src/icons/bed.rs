/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Bed(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M208,72H24V48A8,8,0,0,0,8,48V208a8,8,0,0,0,16,0V176H232v32a8,8,0,0,0,16,0V112A40,40,0,0,0,208,72ZM24,88H96v72H24Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M240,112v56H104V80H208A32,32,0,0,1,240,112Z" opacity="0.2"/><path d="M208,72H24V48A8,8,0,0,0,8,48V208a8,8,0,0,0,16,0V176H232v32a8,8,0,0,0,16,0V112A40,40,0,0,0,208,72ZM24,88H96v72H24Zm88,72V88h96a24,24,0,0,1,24,24v48Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M208,76H20V48a4,4,0,0,0-8,0V208a4,4,0,0,0,8,0V172H236v36a4,4,0,0,0,8,0V112A36,36,0,0,0,208,76ZM20,84h80v80H20Zm88,80V84H208a28,28,0,0,1,28,28v52Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M208,68H28V48A12,12,0,0,0,4,48V208a12,12,0,0,0,24,0V180H228v28a12,12,0,0,0,24,0V112A44.05,44.05,0,0,0,208,68ZM92,156H28V92H92Zm136,0H116V92h92a20,20,0,0,1,20,20Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M208,74H22V48a6,6,0,0,0-12,0V208a6,6,0,0,0,12,0V174H234v34a6,6,0,0,0,12,0V112A38,38,0,0,0,208,74ZM22,86H98v76H22Zm88,76V86h98a26,26,0,0,1,26,26v50Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,72H24V48A8,8,0,0,0,8,48V208a8,8,0,0,0,16,0V176H232v32a8,8,0,0,0,16,0V112A40,40,0,0,0,208,72ZM24,88H96v72H24Zm88,72V88h96a24,24,0,0,1,24,24v48Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg 
            xmlns="http://www.w3.org/2000/svg" 
            width=size.clone()
            height=size
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}