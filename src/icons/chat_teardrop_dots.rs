/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ChatTeardropDots(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M132,24A100.11,100.11,0,0,0,32,124v84.33A15.69,15.69,0,0,0,47.67,224H132a100,100,0,0,0,0-200ZM88,140a12,12,0,1,1,12-12A12,12,0,0,1,88,140Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,132,140Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,176,140Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,124h0a92,92,0,0,1-92,92H47.67A7.66,7.66,0,0,1,40,208.33V124a92,92,0,0,1,92-92h0A92,92,0,0,1,224,124Z" opacity="0.2"/><path d="M132,24A100.11,100.11,0,0,0,32,124v84.33A15.69,15.69,0,0,0,47.67,224H132a100,100,0,0,0,0-200Zm0,184H48V124a84,84,0,1,1,84,84Zm12-80a12,12,0,1,1-12-12A12,12,0,0,1,144,128Zm-44,0a12,12,0,1,1-12-12A12,12,0,0,1,100,128Zm88,0a12,12,0,1,1-12-12A12,12,0,0,1,188,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M132,28a96.11,96.11,0,0,0-96,96v84.33A11.68,11.68,0,0,0,47.67,220H132a96,96,0,0,0,0-192Zm0,184H47.67A3.67,3.67,0,0,1,44,208.33V124a88,88,0,1,1,88,88Zm8-84a8,8,0,1,1-8-8A8,8,0,0,1,140,128Zm-44,0a8,8,0,1,1-8-8A8,8,0,0,1,96,128Zm88,0a8,8,0,1,1-8-8A8,8,0,0,1,184,128Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M132,20A104.11,104.11,0,0,0,28,124v84.33A19.69,19.69,0,0,0,47.67,228H132a104,104,0,0,0,0-208Zm0,184H52V124a80,80,0,1,1,80,80Zm-8-76a16,16,0,1,1-16-16A16,16,0,0,1,124,128Zm48,0a16,16,0,1,1-16-16A16,16,0,0,1,172,128Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M132,26a98.11,98.11,0,0,0-98,98v84.33A13.68,13.68,0,0,0,47.67,222H132a98,98,0,0,0,0-196Zm0,184H47.67A1.67,1.67,0,0,1,46,208.33V124a86,86,0,1,1,86,86Zm10-82a10,10,0,1,1-10-10A10,10,0,0,1,142,128Zm-44,0a10,10,0,1,1-10-10A10,10,0,0,1,98,128Zm88,0a10,10,0,1,1-10-10A10,10,0,0,1,186,128Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M132,24A100.11,100.11,0,0,0,32,124v84.33A15.69,15.69,0,0,0,47.67,224H132a100,100,0,0,0,0-200Zm0,184H48V124a84,84,0,1,1,84,84Zm12-80a12,12,0,1,1-12-12A12,12,0,0,1,144,128Zm-44,0a12,12,0,1,1-12-12A12,12,0,0,1,100,128Zm88,0a12,12,0,1,1-12-12A12,12,0,0,1,188,128Z"/> }.into_view()
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