/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn GooglePlayLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M223.82,114.18,56,18.16a16,16,0,0,0-16.12,0A15.68,15.68,0,0,0,32,31.87V224.13a15.68,15.68,0,0,0,7.92,13.67,16,16,0,0,0,16.12,0l167.78-96a15.76,15.76,0,0,0,0-27.64ZM144,139.31l18.92,18.92-88.5,50.66ZM74.4,47.1l88.53,50.67L144,116.69ZM177.31,150l-22-22,22-22,38.43,22Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M144,128,42.32,230A7.7,7.7,0,0,1,40,224.45V31.55A7.7,7.7,0,0,1,42.32,26Z" opacity="0.2"/><path d="M223.82,114.18,56,18.16a16,16,0,0,0-16.12,0A15.68,15.68,0,0,0,32,31.87V224.13a15.68,15.68,0,0,0,7.92,13.67,16,16,0,0,0,16.12,0l167.78-96a15.76,15.76,0,0,0,0-27.64ZM48,212.67V43.33L132.69,128Zm96-73.36,18.92,18.92-88.5,50.66ZM74.4,47.1l88.53,50.67L144,116.69ZM177.31,150l-22-22,22-22,38.43,22Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M221.89,117.69,54.05,21.62a12,12,0,0,0-12.13,0A11.69,11.69,0,0,0,36,31.87V224.13a11.69,11.69,0,0,0,5.92,10.21,12,12,0,0,0,12.13,0l167.77-96a11.76,11.76,0,0,0,.07-20.66Zm-52.44-20.8L144,122.34,50.4,28.75ZM44,222.33V33.67L138.34,128Zm6.4,4.92L144,133.66l25.45,25.45Zm167.51-95.88L176.65,155l-27-27,27-27L218,124.66a3.77,3.77,0,0,1-.07,6.71Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M225.79,110.7,58,14.65a20.24,20.24,0,0,0-20.12.06A19.62,19.62,0,0,0,28,31.84V224.16a19.62,19.62,0,0,0,9.91,17.13,20.22,20.22,0,0,0,20.12.06l167.76-96a19.76,19.76,0,0,0,0-34.6ZM52,203V53l75,75ZM144,145l12.4,12.4-58,33.2ZM98.41,65.43l58,33.2L144,111ZM178,145l-17-17,17-17,29.72,17Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M222.84,115.93,55,19.89a14,14,0,0,0-14.12,0A13.68,13.68,0,0,0,34,31.87V224.13a13.68,13.68,0,0,0,6.92,11.94,14,14,0,0,0,14.12,0l167.8-96a13.75,13.75,0,0,0,0-24.14ZM46,217.5V38.5L135.51,128Zm98-81,22.19,22.19L62.4,218.07ZM62.4,37.93l103.79,59.4L144,119.52ZM217,129.58l-.1.06L177,152.49,152.49,128,177,103.51l39.94,22.85.1.06a1.76,1.76,0,0,1,0,3.16Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M223.82,114.19,56,18.16a16,16,0,0,0-16.12,0A15.68,15.68,0,0,0,32,31.87V224.13a15.68,15.68,0,0,0,7.92,13.67,16,16,0,0,0,16.12,0l167.78-96a15.75,15.75,0,0,0,0-27.62ZM48,212.67V43.33L132.69,128Zm96-73.36,18.92,18.92-88.5,50.66ZM74.4,47.1l88.53,50.67L144,116.69ZM177.31,150l-22-22,22-22,38.43,22Z"/> }.into_view()
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