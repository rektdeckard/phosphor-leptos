/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn WaveTriangle(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40Zm-9.85,93.12-40,48A8,8,0,0,1,160,184h-.43a8,8,0,0,1-6.23-3.55l-58-87.09L62.15,133.12a8,8,0,0,1-12.3-10.24l40-48a8,8,0,0,1,12.81.68l58.05,87.09,33.14-39.77a8,8,0,1,1,12.3,10.24Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M76,56l52,72H24Zm156,72H128l52,72Z" opacity="0.2"/><path d="M238.48,132.68l-52,72a8,8,0,0,1-13,0L76,69.66l-45.51,63a8,8,0,1,1-13-9.36l52-72a8,8,0,0,1,13,0l97.51,135,45.51-63a8,8,0,1,1,13,9.36Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M235.24,130.34l-52,72a4,4,0,0,1-6.48,0L76,62.83,27.24,130.34a4,4,0,1,1-6.48-4.68l52-72a4,4,0,0,1,6.48,0L180,193.17l48.76-67.51a4,4,0,0,1,6.48,4.68Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M241.73,135l-52,72a12,12,0,0,1-19.46,0L76,76.5,33.73,135A12,12,0,1,1,14.27,121l52-72a12,12,0,0,1,19.46,0L180,179.5,222.27,121A12,12,0,1,1,241.73,135Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M236.86,131.51l-52,72a6,6,0,0,1-9.72,0L76,66.25,28.86,131.51a6,6,0,1,1-9.72-7l52-72a6,6,0,0,1,9.72,0L180,189.75l47.14-65.26a6,6,0,0,1,9.72,7Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M238.48,132.68l-52,72a8,8,0,0,1-13,0L76,69.66l-45.51,63a8,8,0,1,1-13-9.36l52-72a8,8,0,0,1,13,0l97.51,135,45.51-63a8,8,0,1,1,13,9.36Z"/> }.into_view()
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