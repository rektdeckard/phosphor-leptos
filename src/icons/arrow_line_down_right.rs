/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArrowLineDownRight(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M224,40a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,40ZM195.06,96.61a8,8,0,0,0-8.72,1.73L144,140.69,85.66,82.34A8,8,0,0,0,74.34,93.66L132.69,152,90.34,194.34A8,8,0,0,0,96,208h96a8,8,0,0,0,8-8V104A8,8,0,0,0,195.06,96.61Z"/> }.into_view(),
IconWeight::Duotone => view!{ <g opacity="0.2"><polygon points="192 104 192 200 96 200 192 104"/></g><path d="M224,40a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,40Zm-24,64v96a8,8,0,0,1-8,8H96a8,8,0,0,1-5.66-13.66L132.69,152,74.34,93.66A8,8,0,0,1,85.66,82.34L144,140.69l42.34-42.35A8,8,0,0,1,200,104Zm-16,19.31-34.34,34.35h0L115.31,192H184Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M220,40a4,4,0,0,1-4,4H40a4,4,0,0,1,0-8H216A4,4,0,0,1,220,40Zm-28,60a4,4,0,0,0-4,4v86.34L82.83,85.17a4,4,0,0,0-5.66,5.66L182.34,196H96a4,4,0,0,0,0,8h96a4,4,0,0,0,4-4V104A4,4,0,0,0,192,100Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M228,40a12,12,0,0,1-12,12H40a12,12,0,0,1,0-24H216A12,12,0,0,1,228,40ZM192,92a12,12,0,0,0-12,12v67L88.49,79.51a12,12,0,0,0-17,17L163,188H96a12,12,0,0,0,0,24h96a12,12,0,0,0,12-12V104A12,12,0,0,0,192,92Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M222,40a6,6,0,0,1-6,6H40a6,6,0,0,1,0-12H216A6,6,0,0,1,222,40ZM192,98a6,6,0,0,0-6,6v81.52L84.24,83.76a6,6,0,0,0-8.48,8.48L177.52,194H96a6,6,0,0,0,0,12h96a6,6,0,0,0,6-6V104A6,6,0,0,0,192,98Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M224,40a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,40ZM192,96a8,8,0,0,0-8,8v76.69L85.66,82.34A8,8,0,0,0,74.34,93.66L172.69,192H96a8,8,0,0,0,0,16h96a8,8,0,0,0,8-8V104A8,8,0,0,0,192,96Z"/> }.into_view()
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