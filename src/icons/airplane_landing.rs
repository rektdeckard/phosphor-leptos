/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn AirplaneLanding(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M248,216a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16H240A8,8,0,0,1,248,216Zm-24-24a8,8,0,0,0,8-8V148.32a40.13,40.13,0,0,0-29.28-38.54l-60.84-17-22.5-53.63a8,8,0,0,0-4.85-4.5l-5.47-1.82A16,16,0,0,0,88,48V77.39L58.13,68.88,47.52,39.51a8,8,0,0,0-5-4.87l-5.47-1.82A16,16,0,0,0,16,48v55.72a40.12,40.12,0,0,0,29.21,38.52L221.84,191.7A8,8,0,0,0,224,192Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,148.32V184L47.37,134.54A32,32,0,0,1,24,103.73V48a8,8,0,0,1,10.53-7.59L40,42.24,52,75.46,96,88V48a8,8,0,0,1,10.53-7.59L112,42.24l24,57.2,64.56,18A32,32,0,0,1,224,148.32Z" opacity="0.2"/><path d="M248,216a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16H240A8,8,0,0,1,248,216Zm-26.16-24.3L45.21,142.24A40.12,40.12,0,0,1,16,103.72V48A16,16,0,0,1,37.06,32.82l5.47,1.82a8,8,0,0,1,5,4.87L58.13,68.88,88,77.39V48a16,16,0,0,1,21.06-15.18l5.47,1.82a8,8,0,0,1,4.85,4.5l22.5,53.63,60.84,17A40.13,40.13,0,0,1,232,148.32V184a8,8,0,0,1-10.16,7.7ZM216,148.32a24.09,24.09,0,0,0-17.58-23.13l-64.57-18a8,8,0,0,1-5.23-4.61L106,48.67,104,48V88a8,8,0,0,1-10.19,7.7l-44-12.54a8,8,0,0,1-5.33-5L33.79,48.59,32,48v55.72a24.09,24.09,0,0,0,17.53,23.12L216,173.45Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M244,216a4,4,0,0,1-4,4H96a4,4,0,0,1,0-8H240A4,4,0,0,1,244,216Zm-21.08-28.15L46.29,138.4A36.12,36.12,0,0,1,20,103.73V48A12,12,0,0,1,35.79,36.63l5.48,1.82a4,4,0,0,1,2.49,2.44L55.07,72.18,92,82.71V48a12,12,0,0,1,15.79-11.38l5.48,1.82a4,4,0,0,1,2.42,2.25l23.25,55.42,62.7,17.52A36.1,36.1,0,0,1,228,148.33V184a4,4,0,0,1-5.08,3.85ZM220,148.33a28.07,28.07,0,0,0-20.51-27l-64.57-18a4,4,0,0,1-2.61-2.31L109,45.47l-3.75-1.25A4,4,0,0,0,100,48V88a4,4,0,0,1-5.1,3.85l-44-12.54a4,4,0,0,1-2.66-2.49L36.9,45.43l-3.64-1.21a3.95,3.95,0,0,0-3.6.55A4,4,0,0,0,28,48v55.72a28.1,28.1,0,0,0,20.45,27l171.55,48Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M252,216a12,12,0,0,1-12,12H96a12,12,0,0,1,0-24H240A12,12,0,0,1,252,216Zm-31.24-24.45L44.14,142.09A44.13,44.13,0,0,1,12,99.72V48A20,20,0,0,1,38.32,29l5.48,1.83a12,12,0,0,1,7.49,7.3L61.2,65.59,84,72.09V48a20,20,0,0,1,26.32-19l5.48,1.83a12,12,0,0,1,7.27,6.74l21.75,51.85,59,16.49A44.12,44.12,0,0,1,236,148.32V180a12,12,0,0,1-15.24,11.55ZM212,148.32a20.05,20.05,0,0,0-14.65-19.27L132.77,111a12,12,0,0,1-7.84-6.91L108,63.71V88A12,12,0,0,1,92.71,99.53L48.71,87a12,12,0,0,1-8-7.46L36,66.48V99.72A20.07,20.07,0,0,0,50.61,119L212,164.18Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M246,216a6,6,0,0,1-6,6H96a6,6,0,0,1,0-12H240A6,6,0,0,1,246,216Zm-23.62-26.22L45.75,140.32A38.14,38.14,0,0,1,18,103.72V48A14,14,0,0,1,36.43,34.71l5.47,1.83a6,6,0,0,1,3.74,3.65l11,30.33L90,80V48a14,14,0,0,1,18.43-13.29l5.47,1.83a6,6,0,0,1,3.63,3.37l22.88,54.53,61.77,17.27A38.09,38.09,0,0,1,230,148.32V184a6,6,0,0,1-7.62,5.78ZM218,148.32a26.07,26.07,0,0,0-19-25l-64.58-18a6,6,0,0,1-3.91-3.46l-23-54.7-2.89-1A2,2,0,0,0,102,48V88a6,6,0,0,1-7.64,5.77l-44-12.54a6,6,0,0,1-4-3.73L35.34,47l-2.71-.9A1.91,1.91,0,0,0,32,46a2,2,0,0,0-1.16.38A2,2,0,0,0,30,48v55.72a26.09,26.09,0,0,0,19,25l169,47.33Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M248,216a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16H240A8,8,0,0,1,248,216Zm-26.16-24.3L45.21,142.24A40.12,40.12,0,0,1,16,103.72V48A16,16,0,0,1,37.06,32.82l5.47,1.82a8,8,0,0,1,5,4.87L58.13,68.88,88,77.39V48a16,16,0,0,1,21.06-15.18l5.47,1.82a8,8,0,0,1,4.85,4.5l22.5,53.63,60.84,17A40.13,40.13,0,0,1,232,148.32V184a8,8,0,0,1-10.16,7.7ZM216,148.32a24.09,24.09,0,0,0-17.58-23.13l-64.57-18a8,8,0,0,1-5.23-4.61L106,48.67,104,48V88a8,8,0,0,1-10.19,7.7l-44-12.54a8,8,0,0,1-5.33-5L33.79,48.59,32,48v55.72a24.09,24.09,0,0,0,17.53,23.12L216,173.45Z"/> }.into_view()
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