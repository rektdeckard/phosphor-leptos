/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn CaretDoubleUp(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M213.66,202.34A8,8,0,0,1,208,216H48a8,8,0,0,1-5.66-13.66L108.69,136H48a8,8,0,0,1-5.66-13.66l80-80a8,8,0,0,1,11.32,0l80,80A8,8,0,0,1,208,136H147.31Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,208H48l80-80Z" opacity="0.2"/><path d="M133.66,122.34a8,8,0,0,0-11.32,0l-80,80A8,8,0,0,0,48,216H208a8,8,0,0,0,5.66-13.66ZM67.31,200,128,139.31,188.69,200Zm-25-66.34a8,8,0,0,1,0-11.32l80-80a8,8,0,0,1,11.32,0l80,80a8,8,0,0,1-11.32,11.32L128,59.31,53.66,133.66A8,8,0,0,1,42.34,133.66Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M210.83,205.17a4,4,0,0,1-5.66,5.66L128,133.66,50.83,210.83a4,4,0,0,1-5.66-5.66l80-80a4,4,0,0,1,5.66,0Zm-160-74.34L128,53.66l77.17,77.17a4,4,0,0,0,5.66-5.66l-80-80a4,4,0,0,0-5.66,0l-80,80a4,4,0,0,0,5.66,5.66Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M216.49,199.51a12,12,0,0,1-17,17L128,145,56.49,216.49a12,12,0,0,1-17-17l80-80a12,12,0,0,1,17,0Zm-160-63L128,65l71.51,71.52a12,12,0,0,0,17-17l-80-80a12,12,0,0,0-17,0l-80,80a12,12,0,0,0,17,17Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M212.24,203.76a6,6,0,1,1-8.48,8.48L128,136.49,52.24,212.24a6,6,0,0,1-8.48-8.48l80-80a6,6,0,0,1,8.48,0Zm-160-71.52L128,56.49l75.76,75.75a6,6,0,0,0,8.48-8.48l-80-80a6,6,0,0,0-8.48,0l-80,80a6,6,0,0,0,8.48,8.48Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M213.66,202.34a8,8,0,0,1-11.32,11.32L128,139.31,53.66,213.66a8,8,0,0,1-11.32-11.32l80-80a8,8,0,0,1,11.32,0Zm-160-68.68L128,59.31l74.34,74.35a8,8,0,0,0,11.32-11.32l-80-80a8,8,0,0,0-11.32,0l-80,80a8,8,0,0,0,11.32,11.32Z"/> }.into_view()
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