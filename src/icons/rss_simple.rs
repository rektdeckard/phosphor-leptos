/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn RssSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM76,192a12,12,0,1,1,12-12A12,12,0,0,1,76,192Zm60,0a8,8,0,0,1-8-8,56.06,56.06,0,0,0-56-56,8,8,0,0,1,0-16,72.08,72.08,0,0,1,72,72A8,8,0,0,1,136,192Zm48,0a8,8,0,0,1-8-8A104.11,104.11,0,0,0,72,80a8,8,0,0,1,0-16A120.13,120.13,0,0,1,192,184,8,8,0,0,1,184,192Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,200H56V48A152,152,0,0,1,208,200Z" opacity="0.2"/><path d="M216,200a8,8,0,0,1-16,0c0-79.4-64.6-144-144-144a8,8,0,0,1,0-16C144.22,40,216,111.78,216,200ZM56,112a8,8,0,0,0,0,16,72.08,72.08,0,0,1,72,72,8,8,0,0,0,16,0A88.1,88.1,0,0,0,56,112Zm4,72a12,12,0,1,0,12,12A12,12,0,0,0,60,184Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M212,200a4,4,0,0,1-8,0c0-81.61-66.39-148-148-148a4,4,0,0,1,0-8C142,44,212,114,212,200ZM56,116a4,4,0,0,0,0,8,76.08,76.08,0,0,1,76,76,4,4,0,0,0,8,0A84.09,84.09,0,0,0,56,116Zm4,72a8,8,0,1,0,8,8A8,8,0,0,0,60,188Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M220,200a12,12,0,0,1-24,0c0-77.2-62.8-140-140-140a12,12,0,0,1,0-24C146.43,36,220,109.57,220,200ZM56,108a12,12,0,0,0,0,24,68.07,68.07,0,0,1,68,68,12,12,0,0,0,24,0A92.1,92.1,0,0,0,56,108Zm4,72a16,16,0,1,0,16,16A16,16,0,0,0,60,180Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M214,200a6,6,0,0,1-12,0c0-80.5-65.5-146-146-146a6,6,0,0,1,0-12C143.12,42,214,112.88,214,200ZM56,114a6,6,0,0,0,0,12,74.09,74.09,0,0,1,74,74,6,6,0,0,0,12,0A86.1,86.1,0,0,0,56,114Zm4,72a10,10,0,1,0,10,10A10,10,0,0,0,60,186Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M216,200a8,8,0,0,1-16,0c0-79.4-64.6-144-144-144a8,8,0,0,1,0-16C144.22,40,216,111.78,216,200ZM56,112a8,8,0,0,0,0,16,72.08,72.08,0,0,1,72,72,8,8,0,0,0,16,0A88.1,88.1,0,0,0,56,112Zm4,72a12,12,0,1,0,12,12A12,12,0,0,0,60,184Z"/> }.into_view()
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