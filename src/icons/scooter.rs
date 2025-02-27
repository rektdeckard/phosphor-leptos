/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Scooter(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M244,172a32,32,0,1,1-49.38-26.85l-7.35-22-45,57.8A8,8,0,0,1,136,184H73.66a32,32,0,1,1,2.08-16h56.35l49.1-63.13L162.23,48H136a8,8,0,0,1,0-16h32a8,8,0,0,1,7.59,5.47L209.8,140.08c.72-.05,1.46-.08,2.2-.08A32,32,0,0,1,244,172Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M72,172a28,28,0,1,1-28-28A28,28,0,0,1,72,172Zm140-28a28,28,0,1,0,28,28A28,28,0,0,0,212,144Z" opacity="0.2"/><path d="M212,136c-1.18,0-2.35.06-3.51.17l-10.75-32.25v0L175.59,37.47A8,8,0,0,0,168,32H136a8,8,0,0,0,0,16h26.23l19,56.87L132.09,168H79.77a36,36,0,1,0-1.83,16H136a8,8,0,0,0,6.31-3.09l45-57.8,6,18.13A36,36,0,1,0,212,136ZM44,192a20,20,0,1,1,20-20A20,20,0,0,1,44,192Zm168,0a20,20,0,1,1,20-20A20,20,0,0,1,212,192Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M212,140a31.29,31.29,0,0,0-6.24.62l-11.82-35.46h0L171.79,38.74A4,4,0,0,0,168,36H136a4,4,0,0,0,0,8h29.12l20.54,61.63L134,172H76a32,32,0,1,0-1,8h61a4,4,0,0,0,3.16-1.54l49.54-63.7,9.47,28.39A32,32,0,1,0,212,140ZM44,196a24,24,0,1,1,24-24A24,24,0,0,1,44,196Zm168,0a24,24,0,1,1,24-24A24,24,0,0,1,212,196Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M212,132l-.68,0L197.94,91.89v0L179.38,36.21A12,12,0,0,0,168,28H136a12,12,0,0,0,0,24h23.35l13.77,41.3-55,70.7H83.2a40,40,0,1,0-2.55,24H124a12,12,0,0,0,9.47-4.63l48.77-62.7,6.32,19A40,40,0,1,0,212,132ZM44,188a16,16,0,1,1,16-16A16,16,0,0,1,44,188Zm168,0a16,16,0,1,1,16-16A16,16,0,0,1,212,188Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M212,138a34.32,34.32,0,0,0-4.89.36l-11.27-33.82v0L173.69,38.1A6,6,0,0,0,168,34H136a6,6,0,0,0,0,12h27.68l19.75,59.25L133.07,170H77.94a34,34,0,1,0-1.44,12H136a6,6,0,0,0,4.74-2.32L188,118.93l7.74,23.23A34,34,0,1,0,212,138ZM44,194a22,22,0,1,1,22-22A22,22,0,0,1,44,194Zm168,0a22,22,0,1,1,22-22A22,22,0,0,1,212,194Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M212,136c-1.18,0-2.35.06-3.51.17l-10.75-32.25v0L175.59,37.47A8,8,0,0,0,168,32H136a8,8,0,0,0,0,16h26.23l19,56.87L132.09,168H79.77a36,36,0,1,0-1.83,16H136a8,8,0,0,0,6.31-3.09l45-57.8,6,18.13A36,36,0,1,0,212,136ZM44,192a20,20,0,1,1,20-20A20,20,0,0,1,44,192Zm168,0a20,20,0,1,1,20-20A20,20,0,0,1,212,192Z"/> }.into_view()
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