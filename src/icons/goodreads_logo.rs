/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn GoodreadsLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M160,104v8a32,32,0,0,1-64,0v-8a32,32,0,0,1,64,0Zm72,24A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128ZM176,64a8,8,0,0,0-16,0v4.26A48,48,0,0,0,80,104v8a48,48,0,0,0,80,35.74V160a32,32,0,0,1-57.6,19.2,8,8,0,1,0-12.8,9.6A48,48,0,0,0,176,160Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M184,88v24a56,56,0,0,1-112,0V88a56,56,0,0,1,112,0Z" opacity="0.2"/><path d="M184,24a8,8,0,0,0-8,8V45.74A64,64,0,0,0,64,88v24a64,64,0,0,0,112,42.26V168a48.05,48.05,0,0,1-48,48c-16.45,0-32.72-8.08-41.44-20.58a8,8,0,1,0-13.12,9.16C85.06,221.24,106.48,232,128,232a64.07,64.07,0,0,0,64-64V32A8,8,0,0,0,184,24ZM128,160a48.05,48.05,0,0,1-48-48V88a48,48,0,0,1,96,0v24A48.05,48.05,0,0,1,128,160Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M184,28a4,4,0,0,0-4,4V58.13A60,60,0,0,0,68,88v24a60,60,0,0,0,112,29.87V168a52.06,52.06,0,0,1-52,52c-17.72,0-35.28-8.75-44.72-22.29a4,4,0,0,0-6.56,4.58C87.61,217.91,107.74,228,128,228a60.07,60.07,0,0,0,60-60V32A4,4,0,0,0,184,28ZM128,164a52.06,52.06,0,0,1-52-52V88a52,52,0,0,1,104,0v24A52.06,52.06,0,0,1,128,164Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M184,20a12,12,0,0,0-12,12v4.22A67.94,67.94,0,0,0,60,88v24a67.94,67.94,0,0,0,112,51.78V168a44.05,44.05,0,0,1-44,44c-15,0-30.29-7.58-38.16-18.87a12,12,0,0,0-19.68,13.74C82.5,224.56,105.21,236,128,236a68.07,68.07,0,0,0,68-68V32A12,12,0,0,0,184,20ZM128,156a44.05,44.05,0,0,1-44-44V88a44,44,0,0,1,88,0v24A44.05,44.05,0,0,1,128,156Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M184,26a6,6,0,0,0-6,6V51.4A62,62,0,0,0,66,88v24a62,62,0,0,0,112,36.6V168a50.06,50.06,0,0,1-50,50c-17.09,0-34-8.41-43.08-21.43a6,6,0,1,0-9.84,6.86C86.34,219.57,107.11,230,128,230a62.07,62.07,0,0,0,62-62V32A6,6,0,0,0,184,26ZM128,162a50.06,50.06,0,0,1-50-50V88a50,50,0,0,1,100,0v24A50.06,50.06,0,0,1,128,162Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M184,24a8,8,0,0,0-8,8V45.74A64,64,0,0,0,64,88v24a64,64,0,0,0,112,42.26V168a48.05,48.05,0,0,1-48,48c-16.45,0-32.72-8.08-41.44-20.58a8,8,0,1,0-13.12,9.16C85.06,221.24,106.48,232,128,232a64.07,64.07,0,0,0,64-64V32A8,8,0,0,0,184,24ZM128,160a48.05,48.05,0,0,1-48-48V88a48,48,0,0,1,96,0v24A48.05,48.05,0,0,1,128,160Z"/> }.into_view()
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