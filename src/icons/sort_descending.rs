/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn SortDescending(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M40,128a8,8,0,0,1,8-8h72a8,8,0,0,1,0,16H48A8,8,0,0,1,40,128Zm8-56h56a8,8,0,0,0,0-16H48a8,8,0,0,0,0,16ZM184,184H48a8,8,0,0,0,0,16H184a8,8,0,0,0,0-16ZM229.66,82.34l-40-40a8,8,0,0,0-11.32,0l-40,40A8,8,0,0,0,144,96h32v48a8,8,0,0,0,16,0V96h32a8,8,0,0,0,5.66-13.66Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,88H144l40-40Z" opacity="0.2"/><path d="M40,128a8,8,0,0,1,8-8h72a8,8,0,0,1,0,16H48A8,8,0,0,1,40,128Zm8-56h56a8,8,0,0,0,0-16H48a8,8,0,0,0,0,16ZM184,184H48a8,8,0,0,0,0,16H184a8,8,0,0,0,0-16Zm47.39-92.94A8,8,0,0,1,224,96H192v48a8,8,0,0,1-16,0V96H144a8,8,0,0,1-5.66-13.66l40-40a8,8,0,0,1,11.32,0l40,40A8,8,0,0,1,231.39,91.06ZM204.69,80,184,59.31,163.31,80Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M44,128a4,4,0,0,1,4-4h72a4,4,0,0,1,0,8H48A4,4,0,0,1,44,128Zm4-60h56a4,4,0,0,0,0-8H48a4,4,0,0,0,0,8ZM184,188H48a4,4,0,0,0,0,8H184a4,4,0,0,0,0-8ZM226.83,85.17l-40-40a4,4,0,0,0-5.66,0l-40,40a4,4,0,0,0,5.66,5.66L180,57.66V144a4,4,0,0,0,8,0V57.66l33.17,33.17a4,4,0,1,0,5.66-5.66Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M36,128a12,12,0,0,1,12-12h68a12,12,0,0,1,0,24H48A12,12,0,0,1,36,128ZM48,76h52a12,12,0,0,0,0-24H48a12,12,0,0,0,0,24ZM180,180H48a12,12,0,0,0,0,24H180a12,12,0,0,0,0-24ZM232.49,79.51l-40-40a12,12,0,0,0-17,0l-40,40a12,12,0,0,0,17,17L172,77v67a12,12,0,0,0,24,0V77l19.51,19.52a12,12,0,0,0,17-17Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M42,128a6,6,0,0,1,6-6h72a6,6,0,0,1,0,12H48A6,6,0,0,1,42,128Zm6-58h56a6,6,0,0,0,0-12H48a6,6,0,0,0,0,12ZM184,186H48a6,6,0,0,0,0,12H184a6,6,0,0,0,0-12ZM228.24,83.76l-40-40a6,6,0,0,0-8.48,0l-40,40a6,6,0,0,0,8.48,8.48L178,62.49V144a6,6,0,0,0,12,0V62.49l29.76,29.75a6,6,0,0,0,8.48-8.48Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M40,128a8,8,0,0,1,8-8h72a8,8,0,0,1,0,16H48A8,8,0,0,1,40,128Zm8-56h56a8,8,0,0,0,0-16H48a8,8,0,0,0,0,16ZM184,184H48a8,8,0,0,0,0,16H184a8,8,0,0,0,0-16ZM229.66,82.34l-40-40a8,8,0,0,0-11.32,0l-40,40a8,8,0,0,0,11.32,11.32L176,67.31V144a8,8,0,0,0,16,0V67.31l26.34,26.35a8,8,0,0,0,11.32-11.32Z"/> }.into_view()
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