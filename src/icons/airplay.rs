/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Airplay(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M182.07,210.79A8,8,0,0,1,176,224H80a8,8,0,0,1-6.07-13.21l48-56a8,8,0,0,1,12.15,0ZM208,40H48A24,24,0,0,0,24,64V176a24,24,0,0,0,24,24H60.26a4,4,0,0,0,3-1.4l46.48-54.22a24,24,0,0,1,36.44,0L192.7,198.6a4,4,0,0,0,3,1.4H208a24,24,0,0,0,24-24V64A24,24,0,0,0,208,40Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,64V176a16,16,0,0,1-16,16H155.43L128,160l-27.43,32H48a16,16,0,0,1-16-16V64A16,16,0,0,1,48,48H208A16,16,0,0,1,224,64Z" opacity="0.2"/><path d="M134.08,154.79a8,8,0,0,0-12.15,0l-48,56A8,8,0,0,0,80,224h96a8,8,0,0,0,6.07-13.21ZM97.39,208,128,172.29,158.61,208ZM232,64V176a24,24,0,0,1-24,24H192a8,8,0,0,1,0-16h16a8,8,0,0,0,8-8V64a8,8,0,0,0-8-8H48a8,8,0,0,0-8,8V176a8,8,0,0,0,8,8H64a8,8,0,0,1,0,16H48a24,24,0,0,1-24-24V64A24,24,0,0,1,48,40H208A24,24,0,0,1,232,64Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M131,157.4a4,4,0,0,0-6.07,0l-48,56a4,4,0,0,0,3,6.6h96a4,4,0,0,0,3-6.6ZM88.7,212,128,166.15,167.3,212ZM228,64V176a20,20,0,0,1-20,20H192a4,4,0,0,1,0-8h16a12,12,0,0,0,12-12V64a12,12,0,0,0-12-12H48A12,12,0,0,0,36,64V176a12,12,0,0,0,12,12H64a4,4,0,0,1,0,8H48a20,20,0,0,1-20-20V64A20,20,0,0,1,48,44H208A20,20,0,0,1,228,64Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M137.11,152.19a12,12,0,0,0-18.22,0l-48,56A12,12,0,0,0,80,228h96a12,12,0,0,0,9.11-19.81Zm-31,51.81L128,178.44,149.91,204ZM236,64V176a28,28,0,0,1-28,28,12,12,0,0,1,0-24,4,4,0,0,0,4-4V64a4,4,0,0,0-4-4H48a4,4,0,0,0-4,4V176a4,4,0,0,0,4,4,12,12,0,0,1,0,24,28,28,0,0,1-28-28V64A28,28,0,0,1,48,36H208A28,28,0,0,1,236,64Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M132.56,156.1a6,6,0,0,0-9.11,0l-48,56A6,6,0,0,0,80,222h96a6,6,0,0,0,4.56-9.9ZM93.05,210l35-40.78L163,210ZM230,64V176a22,22,0,0,1-22,22H192a6,6,0,0,1,0-12h16a10,10,0,0,0,10-10V64a10,10,0,0,0-10-10H48A10,10,0,0,0,38,64V176a10,10,0,0,0,10,10H64a6,6,0,0,1,0,12H48a22,22,0,0,1-22-22V64A22,22,0,0,1,48,42H208A22,22,0,0,1,230,64Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M134.08,154.79a8,8,0,0,0-12.15,0l-48,56A8,8,0,0,0,80,224h96a8,8,0,0,0,6.07-13.21ZM97.39,208,128,172.29,158.61,208ZM232,64V176a24,24,0,0,1-24,24H192a8,8,0,0,1,0-16h16a8,8,0,0,0,8-8V64a8,8,0,0,0-8-8H48a8,8,0,0,0-8,8V176a8,8,0,0,0,8,8H64a8,8,0,0,1,0,16H48a24,24,0,0,1-24-24V64A24,24,0,0,1,48,40H208A24,24,0,0,1,232,64Z"/> }.into_view()
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