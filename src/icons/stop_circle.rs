/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn StopCircle(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm32,128a8,8,0,0,1-8,8H104a8,8,0,0,1-8-8V104a8,8,0,0,1,8-8h48a8,8,0,0,1,8,8Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M128,32a96,96,0,1,0,96,96A96,96,0,0,0,128,32Zm24,120H104V104h48Z" opacity="0.2"/><path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216ZM152,96H104a8,8,0,0,0-8,8v48a8,8,0,0,0,8,8h48a8,8,0,0,0,8-8V104A8,8,0,0,0,152,96Zm-8,48H112V112h32Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,192a92,92,0,1,1,92-92A92.1,92.1,0,0,1,128,220Zm24-120H104a4,4,0,0,0-4,4v48a4,4,0,0,0,4,4h48a4,4,0,0,0,4-4V104A4,4,0,0,0,152,100Zm-4,48H108V108h40Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,192a84,84,0,1,1,84-84A84.09,84.09,0,0,1,128,212Zm32-104v40a12,12,0,0,1-12,12H108a12,12,0,0,1-12-12V108a12,12,0,0,1,12-12h40A12,12,0,0,1,160,108Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218ZM152,98H104a6,6,0,0,0-6,6v48a6,6,0,0,0,6,6h48a6,6,0,0,0,6-6V104A6,6,0,0,0,152,98Zm-6,48H110V110h36Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216ZM152,96H104a8,8,0,0,0-8,8v48a8,8,0,0,0,8,8h48a8,8,0,0,0,8-8V104A8,8,0,0,0,152,96Zm-8,48H112V112h32Z"/> }.into_view()
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