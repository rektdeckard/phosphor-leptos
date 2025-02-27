/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn IceCream(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M208,89.37V88A80,80,0,0,0,48,88v1.37A24,24,0,0,0,56,136h3.36l61.69,108a8,8,0,0,0,13.9,0l61.69-108H200a24,24,0,0,0,8-46.63ZM128,223.88,77.79,136H97.07l40.57,71Zm18.86-33L115.5,136h19.29l21.71,38Zm18.85-33L153.21,136h25Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,112a16,16,0,0,1-16,16H56a16,16,0,0,1,0-32V88a72,72,0,0,1,144,0v8A16,16,0,0,1,216,112Z" opacity="0.2"/><path d="M208,89.37V88A80,80,0,0,0,48,88v1.37A24,24,0,0,0,56,136h3.36l61.69,108a8,8,0,0,0,13.9,0l61.69-108H200a24,24,0,0,0,8-46.63ZM128,223.88,77.79,136H97.07l40.57,71ZM134.79,136l21.71,38-9.64,16.88L115.5,136Zm30.92,21.88L153.21,136h25ZM200,120H56a8,8,0,0,1,0-16,8,8,0,0,0,8-8V88a64,64,0,0,1,128,0v8a8,8,0,0,0,8,8,8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M204,92.4V88A76,76,0,0,0,52,88v4.4A20,20,0,0,0,56,132h5.68l62.85,110a4,4,0,0,0,6.94,0l62.85-110H200a20,20,0,0,0,4-39.6ZM128,231.94,70.89,132h28.5l42.86,75ZM137.11,132l24,42-14.25,24.94L108.61,132Zm28.6,33.94L146.32,132h38.79ZM200,124H56a12,12,0,0,1,0-24,4,4,0,0,0,4-4V88a68,68,0,0,1,136,0v8a4,4,0,0,0,4,4,12,12,0,0,1,0,24Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M212,86.7a84,84,0,0,0-168,0A28,28,0,0,0,56,140h1l60.54,106a12,12,0,0,0,20.84,0L199,140h1a28,28,0,0,0,12-53.3ZM128,215.81,84.68,140h26.07L141,193Zm26.86-47L138.39,140h32.93ZM200,116H56a4,4,0,0,1,0-8A12,12,0,0,0,68,96V88a60,60,0,0,1,120,0v8a12,12,0,0,0,12,12,4,4,0,0,1,0,8Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M206,90.83V88A78,78,0,0,0,50,88v2.83A22,22,0,0,0,56,134h4.52l62.27,109a6,6,0,0,0,10.42,0l62.27-109H200a22,22,0,0,0,6-43.17ZM128,227.91,74.34,134H98.23L140,207ZM136,134l22.85,40-11.94,20.91L112.05,134Zm29.76,27.91L149.77,134h31.89ZM200,122H56a10,10,0,0,1,0-20,6,6,0,0,0,6-6V88a66,66,0,0,1,132,0v8a6,6,0,0,0,6,6,10,10,0,0,1,0,20Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,89.37V88A80,80,0,0,0,48,88v1.37A24,24,0,0,0,56,136h3.36l61.69,108a8,8,0,0,0,13.9,0l61.69-108H200a24,24,0,0,0,8-46.63ZM128,223.88,77.79,136H97.07l40.57,71ZM134.79,136l21.71,38-9.64,16.88L115.5,136Zm30.92,21.88L153.21,136h25ZM200,120H56a8,8,0,0,1,0-16,8,8,0,0,0,8-8V88a64,64,0,0,1,128,0v8a8,8,0,0,0,8,8,8,8,0,0,1,0,16Z"/> }.into_view()
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