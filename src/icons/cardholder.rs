/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Cardholder(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M208,48H48A24,24,0,0,0,24,72V184a24,24,0,0,0,24,24H208a24,24,0,0,0,24-24V72A24,24,0,0,0,208,48Zm-56.48,76.81a24,24,0,0,1-47,0A16,16,0,0,0,88.81,112H40V96H216v16H167.19A16,16,0,0,0,151.52,124.81ZM48,64H208a8,8,0,0,1,8,8v8H40V72A8,8,0,0,1,48,64Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,72v48H167.19a8,8,0,0,0-7.83,6.4,32,32,0,0,1-62.72,0,8,8,0,0,0-7.83-6.4H32V72A16,16,0,0,1,48,56H208A16,16,0,0,1,224,72Z" opacity="0.2"/><path d="M208,48H48A24,24,0,0,0,24,72V184a24,24,0,0,0,24,24H208a24,24,0,0,0,24-24V72A24,24,0,0,0,208,48ZM40,96H216v16H167.19a16,16,0,0,0-15.67,12.81,24,24,0,0,1-47,0A16,16,0,0,0,88.81,112H40Zm8-32H208a8,8,0,0,1,8,8v8H40V72A8,8,0,0,1,48,64ZM208,192H48a8,8,0,0,1-8-8V128H88.8a40,40,0,0,0,78.39,0H216v56A8,8,0,0,1,208,192Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M208,52H48A20,20,0,0,0,28,72V184a20,20,0,0,0,20,20H208a20,20,0,0,0,20-20V72A20,20,0,0,0,208,52ZM36,92H220v24H167.19a12,12,0,0,0-11.75,9.6,28,28,0,0,1-54.88,0A12,12,0,0,0,88.81,116H36ZM48,60H208a12,12,0,0,1,12,12V84H36V72A12,12,0,0,1,48,60ZM208,196H48a12,12,0,0,1-12-12V124H88.81a4,4,0,0,1,3.91,3.2,36,36,0,0,0,70.56,0,4,4,0,0,1,3.91-3.2H220v60A12,12,0,0,1,208,196Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M208,44H48A28,28,0,0,0,20,72V184a28,28,0,0,0,28,28H208a28,28,0,0,0,28-28V72A28,28,0,0,0,208,44ZM48,68H208a4,4,0,0,1,4,4V88H167.19a20,20,0,0,0-19.59,16,20,20,0,0,1-39.2,0A20,20,0,0,0,88.81,88H44V72A4,4,0,0,1,48,68ZM208,188H48a4,4,0,0,1-4-4V112H85.66a44,44,0,0,0,84.68,0H212v72A4,4,0,0,1,208,188Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M208,50H48A22,22,0,0,0,26,72V184a22,22,0,0,0,22,22H208a22,22,0,0,0,22-22V72A22,22,0,0,0,208,50ZM38,94H218v20H167.19a14,14,0,0,0-13.71,11.21,26,26,0,0,1-51,0A14,14,0,0,0,88.81,114H38ZM48,62H208a10,10,0,0,1,10,10V82H38V72A10,10,0,0,1,48,62ZM208,194H48a10,10,0,0,1-10-10V126H88.81a2,2,0,0,1,2,1.59,38,38,0,0,0,74.48,0,2,2,0,0,1,1.95-1.59H218v58A10,10,0,0,1,208,194Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,48H48A24,24,0,0,0,24,72V184a24,24,0,0,0,24,24H208a24,24,0,0,0,24-24V72A24,24,0,0,0,208,48ZM40,96H216v16H167.19a16,16,0,0,0-15.67,12.81,24,24,0,0,1-47,0A16,16,0,0,0,88.81,112H40Zm8-32H208a8,8,0,0,1,8,8v8H40V72A8,8,0,0,1,48,64ZM208,192H48a8,8,0,0,1-8-8V128H88.8a40,40,0,0,0,78.39,0H216v56A8,8,0,0,1,208,192Z"/> }.into_view()
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