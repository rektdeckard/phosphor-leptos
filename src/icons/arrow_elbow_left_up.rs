/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArrowElbowLeftUp(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M232,192a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V104H32a8,8,0,0,1-5.66-13.66l48-48a8,8,0,0,1,11.32,0l48,48A8,8,0,0,1,128,104H88v80H224A8,8,0,0,1,232,192Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M128,96H32L80,48Z" opacity="0.2"/><path d="M224,184H88V104h40a8,8,0,0,0,5.66-13.66l-48-48a8,8,0,0,0-11.32,0l-48,48A8,8,0,0,0,32,104H72v88a8,8,0,0,0,8,8H224a8,8,0,0,0,0-16ZM80,59.31,108.69,88H51.31Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M228,192a4,4,0,0,1-4,4H80a4,4,0,0,1-4-4V57.66L34.83,98.83a4,4,0,0,1-5.66-5.66l48-48a4,4,0,0,1,5.66,0l48,48a4,4,0,0,1-5.66,5.66L84,57.66V188H224A4,4,0,0,1,228,192Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M236,192a12,12,0,0,1-12,12H80a12,12,0,0,1-12-12V77L40.49,104.49a12,12,0,0,1-17-17l48-48a12,12,0,0,1,17,0l48,48a12,12,0,0,1-17,17L92,77V180H224A12,12,0,0,1,236,192Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M230,192a6,6,0,0,1-6,6H80a6,6,0,0,1-6-6V62.49L36.24,100.24a6,6,0,0,1-8.48-8.48l48-48a6,6,0,0,1,8.48,0l48,48a6,6,0,1,1-8.48,8.48L86,62.49V186H224A6,6,0,0,1,230,192Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M232,192a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V67.31L37.66,101.66A8,8,0,0,1,26.34,90.34l48-48a8,8,0,0,1,11.32,0l48,48a8,8,0,0,1-11.32,11.32L88,67.31V184H224A8,8,0,0,1,232,192Z"/> }.into_view()
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