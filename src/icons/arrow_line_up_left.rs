/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArrowLineUpLeft(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M56,152V56a8,8,0,0,1,8-8h96a8,8,0,0,1,5.66,13.66L123.31,104l58.35,58.34a8,8,0,0,1-11.32,11.32L112,115.31,69.66,157.66A8,8,0,0,1,56,152Zm160,56H40a8,8,0,0,0,0,16H216a8,8,0,0,0,0-16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M160,56,64,152V56Z" opacity="0.2"/><path d="M224,216a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,216ZM56,152V56a8,8,0,0,1,8-8h96a8,8,0,0,1,5.66,13.66L123.31,104l58.35,58.34a8,8,0,0,1-11.32,11.32L112,115.31,69.66,157.66A8,8,0,0,1,56,152Zm16-19.31,34.34-34.35h0L140.69,64H72Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M220,216a4,4,0,0,1-4,4H40a4,4,0,0,1,0-8H216A4,4,0,0,1,220,216ZM64,156a4,4,0,0,0,4-4V65.66L173.17,170.83a4,4,0,0,0,5.66-5.66L73.66,60H160a4,4,0,0,0,0-8H64a4,4,0,0,0-4,4v96A4,4,0,0,0,64,156Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M228,216a12,12,0,0,1-12,12H40a12,12,0,0,1,0-24H216A12,12,0,0,1,228,216ZM64,164a12,12,0,0,0,12-12V85l91.51,91.52a12,12,0,0,0,17-17L93,68h67a12,12,0,0,0,0-24H64A12,12,0,0,0,52,56v96A12,12,0,0,0,64,164Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M222,216a6,6,0,0,1-6,6H40a6,6,0,0,1,0-12H216A6,6,0,0,1,222,216ZM64,158a6,6,0,0,0,6-6V70.49L171.76,172.24a6,6,0,0,0,8.48-8.48L78.49,62H160a6,6,0,0,0,0-12H64a6,6,0,0,0-6,6v96A6,6,0,0,0,64,158Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M224,216a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,216ZM64,160a8,8,0,0,0,8-8V75.31l98.34,98.35a8,8,0,0,0,11.32-11.32L83.31,64H160a8,8,0,0,0,0-16H64a8,8,0,0,0-8,8v96A8,8,0,0,0,64,160Z"/> }.into_view()
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