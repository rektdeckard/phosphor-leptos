/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn FadersHorizontal(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M32,80a8,8,0,0,1,8-8H72a8,8,0,0,1,0,16H40A8,8,0,0,1,32,80Zm184,88H192V152a8,8,0,0,0-8-8H168a8,8,0,0,0-8,8v48a8,8,0,0,0,8,8h16a8,8,0,0,0,8-8V184h24a8,8,0,0,0,0-16Zm-80,0H40a8,8,0,0,0,0,16h96a8,8,0,0,0,0-16Zm-32-56h16a8,8,0,0,0,8-8V88h88a8,8,0,0,0,0-16H128V56a8,8,0,0,0-8-8H104a8,8,0,0,0-8,8v48A8,8,0,0,0,104,112Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,80v96H40V80Z" opacity="0.2"/><path d="M32,80a8,8,0,0,1,8-8H72a8,8,0,0,1,0,16H40A8,8,0,0,1,32,80Zm184,88H176V152a8,8,0,0,0-16,0v48a8,8,0,0,0,16,0V184h40a8,8,0,0,0,0-16Zm-80,0H40a8,8,0,0,0,0,16h96a8,8,0,0,0,0-16Zm-32-56a8,8,0,0,0,8-8V88H216a8,8,0,0,0,0-16H112V56a8,8,0,0,0-16,0v48A8,8,0,0,0,104,112Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M36,80a4,4,0,0,1,4-4H72a4,4,0,0,1,0,8H40A4,4,0,0,1,36,80Zm180,92H172V152a4,4,0,0,0-8,0v48a4,4,0,0,0,8,0V180h44a4,4,0,0,0,0-8Zm-80,0H40a4,4,0,0,0,0,8h96a4,4,0,0,0,0-8Zm-32-64a4,4,0,0,0,4-4V84H216a4,4,0,0,0,0-8H108V56a4,4,0,0,0-8,0v48A4,4,0,0,0,104,108Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M28,80A12,12,0,0,1,40,68H68a12,12,0,0,1,0,24H40A12,12,0,0,1,28,80Zm188,84H184V152a12,12,0,0,0-24,0v48a12,12,0,0,0,24,0V188h32a12,12,0,0,0,0-24Zm-84,0H40a12,12,0,0,0,0,24h92a12,12,0,0,0,0-24Zm-24-48a12,12,0,0,0,12-12V92h96a12,12,0,0,0,0-24H120V56a12,12,0,0,0-24,0v48A12,12,0,0,0,108,116Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M34,80a6,6,0,0,1,6-6H72a6,6,0,0,1,0,12H40A6,6,0,0,1,34,80Zm182,90H174V152a6,6,0,0,0-12,0v48a6,6,0,0,0,12,0V182h42a6,6,0,0,0,0-12Zm-80,0H40a6,6,0,0,0,0,12h96a6,6,0,0,0,0-12Zm-32-60a6,6,0,0,0,6-6V86H216a6,6,0,0,0,0-12H110V56a6,6,0,0,0-12,0v48A6,6,0,0,0,104,110Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M32,80a8,8,0,0,1,8-8H72a8,8,0,0,1,0,16H40A8,8,0,0,1,32,80Zm184,88H176V152a8,8,0,0,0-16,0v48a8,8,0,0,0,16,0V184h40a8,8,0,0,0,0-16Zm-80,0H40a8,8,0,0,0,0,16h96a8,8,0,0,0,0-16Zm-32-56a8,8,0,0,0,8-8V88H216a8,8,0,0,0,0-16H112V56a8,8,0,0,0-16,0v48A8,8,0,0,0,104,112Z"/> }.into_view()
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