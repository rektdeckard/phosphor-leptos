/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ProhibitInset(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm37.66,141.66a8,8,0,0,1-11.32,0l-64-64a8,8,0,0,1,11.32-11.32l64,64A8,8,0,0,1,165.66,165.66Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"/><path d="M165.66,154.34a8,8,0,0,1-11.32,11.32l-64-64a8,8,0,0,1,11.32-11.32ZM232,128A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-16,0a88,88,0,1,0-88,88A88.1,88.1,0,0,0,216,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M162.83,157.17a4,4,0,0,1-5.66,5.66l-64-64a4,4,0,0,1,5.66-5.66ZM228,128A100,100,0,1,1,128,28,100.11,100.11,0,0,1,228,128Zm-8,0a92,92,0,1,0-92,92A92.1,92.1,0,0,0,220,128Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M168.49,151.51a12,12,0,0,1-17,17l-64-64a12,12,0,0,1,17-17ZM236,128A108,108,0,1,1,128,20,108.12,108.12,0,0,1,236,128Zm-24,0a84,84,0,1,0-84,84A84.09,84.09,0,0,0,212,128Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M164.24,155.76a6,6,0,1,1-8.48,8.48l-64-64a6,6,0,0,1,8.48-8.48ZM230,128A102,102,0,1,1,128,26,102.12,102.12,0,0,1,230,128Zm-12,0a90,90,0,1,0-90,90A90.1,90.1,0,0,0,218,128Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M165.66,154.34a8,8,0,0,1-11.32,11.32l-64-64a8,8,0,0,1,11.32-11.32ZM232,128A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-16,0a88,88,0,1,0-88,88A88.1,88.1,0,0,0,216,128Z"/> }.into_view()
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