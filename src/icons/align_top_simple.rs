/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn AlignTopSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M208,32a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,32ZM160,56H96A16,16,0,0,0,80,72V224a16,16,0,0,0,16,16h64a16,16,0,0,0,16-16V72A16,16,0,0,0,160,56Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M168,72V224a8,8,0,0,1-8,8H96a8,8,0,0,1-8-8V72a8,8,0,0,1,8-8h64A8,8,0,0,1,168,72Z" opacity="0.2"/><path d="M208,32a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,32ZM176,72V224a16,16,0,0,1-16,16H96a16,16,0,0,1-16-16V72A16,16,0,0,1,96,56h64A16,16,0,0,1,176,72Zm-16,0H96V224h64Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M204,32a4,4,0,0,1-4,4H56a4,4,0,0,1,0-8H200A4,4,0,0,1,204,32ZM172,72V224a12,12,0,0,1-12,12H96a12,12,0,0,1-12-12V72A12,12,0,0,1,96,60h64A12,12,0,0,1,172,72Zm-8,0a4,4,0,0,0-4-4H96a4,4,0,0,0-4,4V224a4,4,0,0,0,4,4h64a4,4,0,0,0,4-4Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M212,32a12,12,0,0,1-12,12H56a12,12,0,0,1,0-24H200A12,12,0,0,1,212,32ZM180,80V224a20,20,0,0,1-20,20H96a20,20,0,0,1-20-20V80A20,20,0,0,1,96,60h64A20,20,0,0,1,180,80Zm-24,4H100V220h56Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M206,32a6,6,0,0,1-6,6H56a6,6,0,0,1,0-12H200A6,6,0,0,1,206,32ZM174,72V224a14,14,0,0,1-14,14H96a14,14,0,0,1-14-14V72A14,14,0,0,1,96,58h64A14,14,0,0,1,174,72Zm-12,0a2,2,0,0,0-2-2H96a2,2,0,0,0-2,2V224a2,2,0,0,0,2,2h64a2,2,0,0,0,2-2Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,32a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,32ZM176,72V224a16,16,0,0,1-16,16H96a16,16,0,0,1-16-16V72A16,16,0,0,1,96,56h64A16,16,0,0,1,176,72Zm-16,0H96V224h64Z"/> }.into_view()
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