/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Option(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM200,176H152.94a15.92,15.92,0,0,1-14.31-8.84L103.06,96H56a8,8,0,0,1,0-16h47.06a15.92,15.92,0,0,1,14.31,8.84L152.94,160H200a8,8,0,0,1,0,16Zm0-80H152a8,8,0,0,1,0-16h48a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,72V184H40a8,8,0,0,1-8-8V72Z" opacity="0.2"/><path d="M232,184a8,8,0,0,1-8,8H160.94a15.92,15.92,0,0,1-14.31-8.84L95.06,80H32a8,8,0,0,1,0-16H95.06a15.92,15.92,0,0,1,14.31,8.84L160.94,176H224A8,8,0,0,1,232,184ZM152,80h72a8,8,0,0,0,0-16H152a8,8,0,0,0,0,16Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M228,184a4,4,0,0,1-4,4H160.94a11.94,11.94,0,0,1-10.73-6.63L98.63,78.21A4,4,0,0,0,95.06,76H32a4,4,0,0,1,0-8H95.06a11.94,11.94,0,0,1,10.73,6.63l51.58,103.16a4,4,0,0,0,3.57,2.21H224A4,4,0,0,1,228,184ZM152,76h72a4,4,0,0,0,0-8H152a4,4,0,0,0,0,8Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M236,184a12,12,0,0,1-12,12H160.94a19.89,19.89,0,0,1-17.88-11.06L92.58,84H32a12,12,0,0,1,0-24H95.06a19.89,19.89,0,0,1,17.88,11.06L163.42,172H224A12,12,0,0,1,236,184ZM152,84h72a12,12,0,0,0,0-24H152a12,12,0,0,0,0,24Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M230,184a6,6,0,0,1-6,6H160.94a13.92,13.92,0,0,1-12.52-7.74L96.84,79.11A2,2,0,0,0,95.06,78H32a6,6,0,0,1,0-12H95.06a13.92,13.92,0,0,1,12.52,7.74l51.58,103.15a2,2,0,0,0,1.78,1.11H224A6,6,0,0,1,230,184ZM152,78h72a6,6,0,0,0,0-12H152a6,6,0,0,0,0,12Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M232,184a8,8,0,0,1-8,8H160.94a15.92,15.92,0,0,1-14.31-8.84L95.06,80H32a8,8,0,0,1,0-16H95.06a15.92,15.92,0,0,1,14.31,8.84L160.94,176H224A8,8,0,0,1,232,184ZM152,80h72a8,8,0,0,0,0-16H152a8,8,0,0,0,0,16Z"/> }.into_view()
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