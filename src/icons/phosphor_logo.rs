/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn PhosphorLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M144,24H64a8,8,0,0,0-8,8V160a80.09,80.09,0,0,0,80,80,8,8,0,0,0,8-8V168a72,72,0,0,0,0-144ZM128,223.5A64.14,64.14,0,0,1,72.51,168H128Zm0-94L77.68,40H128ZM144,152V40a56,56,0,0,1,0,112Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,96a64,64,0,0,1-64,64h-8V32h8A64,64,0,0,1,208,96ZM64,160h72L64,32Z" opacity="0.2"/><path d="M144,24H64a8,8,0,0,0-8,8V160a80.09,80.09,0,0,0,80,80,8,8,0,0,0,8-8V168a72,72,0,0,0,0-144ZM72,62.54,122.32,152H72Zm56,161A64.14,64.14,0,0,1,72.51,168H128Zm0-94L77.68,40H128ZM144,152V40a56,56,0,0,1,0,112Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M144,28H64a4,4,0,0,0-4,4V160a76.08,76.08,0,0,0,76,76,4,4,0,0,0,4-4V164h4a68,68,0,0,0,0-136ZM68,47.27,129.16,156H68Zm64,97.46L70.84,36H132ZM68.13,164H132v63.88A68.1,68.1,0,0,1,68.13,164ZM144,156h-4V36h4a60,60,0,0,1,0,120Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M220,96a76.08,76.08,0,0,0-76-76H64A12,12,0,0,0,52,32V160a84.09,84.09,0,0,0,84,84,12,12,0,0,0,12-12V171.89A76.09,76.09,0,0,0,220,96ZM76,77.81,115.48,148H76Zm48,36.38L84.52,44H124ZM77.22,172H124v46.79A60.18,60.18,0,0,1,77.22,172ZM148,147.83V44.17a52,52,0,0,1,0,103.66Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M144,26H64a6,6,0,0,0-6,6V160a78.09,78.09,0,0,0,78,78,6,6,0,0,0,6-6V166h2a70,70,0,0,0,0-140ZM70,54.91,125.74,154H70Zm60,82.19L74.26,38H130ZM70.28,166H130v59.73A66.1,66.1,0,0,1,70.28,166ZM144,154h-2V38h2a58,58,0,0,1,0,116Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M144,24H64a8,8,0,0,0-8,8V160a80.09,80.09,0,0,0,80,80,8,8,0,0,0,8-8V168a72,72,0,0,0,0-144ZM72,62.54,122.32,152H72Zm56,66.92L77.68,40H128ZM72.51,168H128v55.5A64.14,64.14,0,0,1,72.51,168ZM144,152V40a56,56,0,0,1,0,112Z"/> }.into_view()
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