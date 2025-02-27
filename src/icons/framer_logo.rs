/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn FramerLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M200,104H149l56.27,50A8,8,0,0,1,200,168H136v64a8,8,0,0,1-13.66,5.66l-72-72A8,8,0,0,1,48,160V96a8,8,0,0,1,8-8h51L50.69,38A8,8,0,0,1,56,24H200a8,8,0,0,1,8,8V96A8,8,0,0,1,200,104Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M200,96H128L56,32H200ZM56,160l72,72V160h72L128,96H56Z" opacity="0.2"/><path d="M208,96V32a8,8,0,0,0-8-8H56a8,8,0,0,0-5.31,14L107,88H56a8,8,0,0,0-8,8v64a8,8,0,0,0,2.34,5.66l72,72A8,8,0,0,0,136,232V168h64a8,8,0,0,0,5.31-14L149,104h51A8,8,0,0,0,208,96Zm-29,56H128a8,8,0,0,0-8,8v52.69l-56-56V104h61Zm13-64H131L77,40H192Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M204,96V32a4,4,0,0,0-4-4H56a4,4,0,0,0-2.66,7l64.14,57H56a4,4,0,0,0-4,4v64a4,4,0,0,0,1.17,2.83l72,72A4,4,0,0,0,132,232V164h68a4,4,0,0,0,2.66-7l-64.14-57H200A4,4,0,0,0,204,96Zm-14.52,60H128a4,4,0,0,0-4,4v62.34l-64-64V100h66.48ZM196,92H129.52l-63-56H196Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M212,96V32a12,12,0,0,0-12-12H56a12,12,0,0,0-8,21L96.44,84H56A12,12,0,0,0,44,96v64a12,12,0,0,0,3.52,8.49l72,72A12,12,0,0,0,140,232V172h60a12,12,0,0,0,8-21l-48.41-43H200A12,12,0,0,0,212,96Zm-43.56,52H128a12,12,0,0,0-12,12v43L68,155V108h55.44ZM188,84H132.56l-45-40H188Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M206,96V32a6,6,0,0,0-6-6H56a6,6,0,0,0-4,10.48L112.22,90H56a6,6,0,0,0-6,6v64a6,6,0,0,0,1.76,4.24l72,72A6,6,0,0,0,134,232V166h66a6,6,0,0,0,4-10.48L143.78,102H200A6,6,0,0,0,206,96Zm-21.78,58H128a6,6,0,0,0-6,6v57.51l-60-60V102h63.72ZM194,90H130.28L71.78,38H194Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,96V32a8,8,0,0,0-8-8H56a8,8,0,0,0-5.31,14L107,88H56a8,8,0,0,0-8,8v64a8,8,0,0,0,2.34,5.66l72,72A8,8,0,0,0,136,232V168h64a8,8,0,0,0,5.31-14L149,104h51A8,8,0,0,0,208,96Zm-29,56H128a8,8,0,0,0-8,8v52.69l-56-56V104h61Zm13-64H131L77,40H192Z"/> }.into_view()
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