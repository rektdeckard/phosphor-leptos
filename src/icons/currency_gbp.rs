/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn CurrencyGbp(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm40,168H80a8,8,0,0,1,0-16h4a20,20,0,0,0,20-20V136H80a8,8,0,0,1,0-16h24V96a40,40,0,0,1,60-34.64,8,8,0,1,1-8,13.85A24,24,0,0,0,120,96v24h24a8,8,0,0,1,0,16H120v20a35.79,35.79,0,0,1-6.08,20H168a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M168,208H60a36,36,0,0,0,36-36V84a44,44,0,0,1,72-33.95Z" opacity="0.2"/><path d="M192,208a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16h4a28,28,0,0,0,28-28V136H56a8,8,0,0,1,0-16H88V84a52,52,0,0,1,85.08-40.12A8,8,0,1,1,162.9,56.22,36,36,0,0,0,104,84v36h32a8,8,0,0,1,0,16H104v36a43.82,43.82,0,0,1-10.08,28H184A8,8,0,0,1,192,208Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M188,208a4,4,0,0,1-4,4H56a4,4,0,0,1,0-8h4a32,32,0,0,0,32-32V132H56a4,4,0,0,1,0-8H92V84a48,48,0,0,1,78.53-37,4,4,0,1,1-5.09,6.17A40,40,0,0,0,100,84v40h36a4,4,0,0,1,0,8H100v40a40,40,0,0,1-16,32H184A4,4,0,0,1,188,208Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M196,208a12,12,0,0,1-12,12H56a12,12,0,0,1,0-24h4a24,24,0,0,0,24-24V140H56a12,12,0,0,1,0-24H84V84a56,56,0,0,1,91.63-43.21A12,12,0,0,1,160.35,59.3,31.66,31.66,0,0,0,140,52a32,32,0,0,0-32,32v32h28a12,12,0,0,1,0,24H108v32a47.74,47.74,0,0,1-6.44,24H184A12,12,0,0,1,196,208Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M190,208a6,6,0,0,1-6,6H56a6,6,0,0,1,0-12h4a30,30,0,0,0,30-30V134H56a6,6,0,0,1,0-12H90V84a50,50,0,0,1,81.81-38.58,6,6,0,1,1-7.64,9.26A38,38,0,0,0,102,84v38h34a6,6,0,0,1,0,12H102v38a41.88,41.88,0,0,1-12.63,30H184A6,6,0,0,1,190,208Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M192,208a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16h4a28,28,0,0,0,28-28V136H56a8,8,0,0,1,0-16H88V84a52,52,0,0,1,85.08-40.12A8,8,0,1,1,162.9,56.22,36,36,0,0,0,104,84v36h32a8,8,0,0,1,0,16H104v36a43.82,43.82,0,0,1-10.08,28H184A8,8,0,0,1,192,208Z"/> }.into_view()
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