/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArrowsInLineHorizontal(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M101.66,122.34a8,8,0,0,1,0,11.32l-32,32A8,8,0,0,1,56,160V136H16a8,8,0,0,1,0-16H56V96a8,8,0,0,1,13.66-5.66ZM240,120H200V96a8,8,0,0,0-13.66-5.66l-32,32a8,8,0,0,0,0,11.32l32,32A8,8,0,0,0,200,160V136h40a8,8,0,0,0,0-16ZM128,32a8,8,0,0,0-8,8V216a8,8,0,0,0,16,0V40A8,8,0,0,0,128,32Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M64,96l32,32L64,160Zm96,32,32,32V96Z" opacity="0.2"/><path d="M136,40V216a8,8,0,0,1-16,0V40a8,8,0,0,1,16,0Zm-34.34,82.34a8,8,0,0,1,0,11.32l-32,32A8,8,0,0,1,56,160V136H16a8,8,0,0,1,0-16H56V96a8,8,0,0,1,13.66-5.66Zm-17,5.66L72,115.31v25.38ZM248,128a8,8,0,0,1-8,8H200v24a8,8,0,0,1-13.66,5.66l-32-32a8,8,0,0,1,0-11.32l32-32A8,8,0,0,1,200,96v24h40A8,8,0,0,1,248,128Zm-64-12.69L171.31,128,184,140.69Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M132,40V216a4,4,0,0,1-8,0V40a4,4,0,0,1,8,0ZM66.83,93.17a4,4,0,0,0-5.66,5.66L86.34,124H16a4,4,0,0,0,0,8H86.34L61.17,157.17a4,4,0,0,0,5.66,5.66l32-32a4,4,0,0,0,0-5.66ZM240,124H169.66l25.17-25.17a4,4,0,1,0-5.66-5.66l-32,32a4,4,0,0,0,0,5.66l32,32a4,4,0,0,0,5.66-5.66L169.66,132H240a4,4,0,0,0,0-8Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M140,40V216a12,12,0,0,1-24,0V40a12,12,0,0,1,24,0ZM64.49,87.51a12,12,0,0,0-17,17L59,116H16a12,12,0,0,0,0,24H59L47.51,151.51a12,12,0,0,0,17,17l32-32a12,12,0,0,0,0-17ZM240,116H197l11.52-11.51a12,12,0,0,0-17-17l-32,32a12,12,0,0,0,0,17l32,32a12,12,0,0,0,17-17L197,140h43a12,12,0,0,0,0-24Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M134,40V216a6,6,0,0,1-12,0V40a6,6,0,0,1,12,0ZM68.24,91.76a6,6,0,0,0-8.48,8.48L81.51,122H16a6,6,0,0,0,0,12H81.51L59.76,155.76a6,6,0,1,0,8.48,8.48l32-32a6,6,0,0,0,0-8.48ZM240,122H174.49l21.75-21.76a6,6,0,0,0-8.48-8.48l-32,32a6,6,0,0,0,0,8.48l32,32a6,6,0,0,0,8.48-8.48L174.49,134H240a6,6,0,0,0,0-12Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M136,40V216a8,8,0,0,1-16,0V40a8,8,0,0,1,16,0ZM69.66,90.34a8,8,0,0,0-11.32,11.32L76.69,120H16a8,8,0,0,0,0,16H76.69L58.34,154.34a8,8,0,0,0,11.32,11.32l32-32a8,8,0,0,0,0-11.32ZM240,120H179.31l18.35-18.34a8,8,0,0,0-11.32-11.32l-32,32a8,8,0,0,0,0,11.32l32,32a8,8,0,0,0,11.32-11.32L179.31,136H240a8,8,0,0,0,0-16Z"/> }.into_view()
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