/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn AlignLeftSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M40,56V200a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0ZM224,80H72A16,16,0,0,0,56,96v64a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V96A16,16,0,0,0,224,80Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M232,96v64a8,8,0,0,1-8,8H72a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8H224A8,8,0,0,1,232,96Z" opacity="0.2"/><path d="M40,56V200a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0ZM240,96v64a16,16,0,0,1-16,16H72a16,16,0,0,1-16-16V96A16,16,0,0,1,72,80H224A16,16,0,0,1,240,96Zm-16,64V96H72v64H224Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M36,56V200a4,4,0,0,1-8,0V56a4,4,0,0,1,8,0ZM236,96v64a12,12,0,0,1-12,12H72a12,12,0,0,1-12-12V96A12,12,0,0,1,72,84H224A12,12,0,0,1,236,96Zm-8,0a4,4,0,0,0-4-4H72a4,4,0,0,0-4,4v64a4,4,0,0,0,4,4H224a4,4,0,0,0,4-4Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M44,56V200a12,12,0,0,1-24,0V56a12,12,0,0,1,24,0ZM244,96v64a20,20,0,0,1-20,20H80a20,20,0,0,1-20-20V96A20,20,0,0,1,80,76H224A20,20,0,0,1,244,96Zm-24,4H84v56H220Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M38,56V200a6,6,0,0,1-12,0V56a6,6,0,0,1,12,0ZM238,96v64a14,14,0,0,1-14,14H72a14,14,0,0,1-14-14V96A14,14,0,0,1,72,82H224A14,14,0,0,1,238,96Zm-12,0a2,2,0,0,0-2-2H72a2,2,0,0,0-2,2v64a2,2,0,0,0,2,2H224a2,2,0,0,0,2-2Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M40,56V200a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0ZM240,96v64a16,16,0,0,1-16,16H72a16,16,0,0,1-16-16V96A16,16,0,0,1,72,80H224A16,16,0,0,1,240,96Zm-16,64V96H72v64H224Z"/> }.into_view()
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