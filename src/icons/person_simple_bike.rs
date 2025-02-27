/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn PersonSimpleBike(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M136,52a28,28,0,1,1,28,28A28,28,0,0,1,136,52ZM240,176a40,40,0,1,1-40-40A40,40,0,0,1,240,176Zm-16,0a24,24,0,1,0-24,24A24,24,0,0,0,224,176Zm-24-64a8,8,0,0,0-8-8H155.31L125.66,74.34a8,8,0,0,0-11.32,0l-32,32a8,8,0,0,0,0,11.32L120,155.31V200a8,8,0,0,0,16,0V152a8,8,0,0,0-2.34-5.66L99.31,112,120,91.31l26.34,26.35A8,8,0,0,0,152,120h40A8,8,0,0,0,200,112ZM96,176a40,40,0,1,1-40-40A40,40,0,0,1,96,176Zm-16,0a24,24,0,1,0-24,24A24,24,0,0,0,80,176Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M232,176a32,32,0,1,1-32-32A32,32,0,0,1,232,176ZM56,144a32,32,0,1,0,32,32A32,32,0,0,0,56,144Z" opacity="0.2"/><path d="M164,80a28,28,0,1,0-28-28A28,28,0,0,0,164,80Zm0-40a12,12,0,1,1-12,12A12,12,0,0,1,164,40Zm36,96a40,40,0,1,0,40,40A40,40,0,0,0,200,136Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,200,200ZM56,136a40,40,0,1,0,40,40A40,40,0,0,0,56,136Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,56,200Zm136-80H152a8,8,0,0,1-5.66-2.34L120,91.31,99.31,112l34.35,34.34A8,8,0,0,1,136,152v48a8,8,0,0,1-16,0V155.31L82.34,117.66a8,8,0,0,1,0-11.32l32-32a8,8,0,0,1,11.32,0L155.31,104H192a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M164,76a24,24,0,1,0-24-24A24,24,0,0,0,164,76Zm0-40a16,16,0,1,1-16,16A16,16,0,0,1,164,36Zm36,104a36,36,0,1,0,36,36A36,36,0,0,0,200,140Zm0,64a28,28,0,1,1,28-28A28,28,0,0,1,200,204ZM56,140a36,36,0,1,0,36,36A36,36,0,0,0,56,140Zm0,64a28,28,0,1,1,28-28A28,28,0,0,1,56,204Zm136-88H152a4,4,0,0,1-2.83-1.17L120,85.66,93.66,112l37.17,37.17A4,4,0,0,1,132,152v48a4,4,0,0,1-8,0V153.66L85.17,114.83a4,4,0,0,1,0-5.66l32-32a4,4,0,0,1,5.66,0L153.66,108H192a4,4,0,0,1,0,8Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M168,84a32,32,0,1,0-32-32A32,32,0,0,0,168,84Zm0-40a8,8,0,1,1-8,8A8,8,0,0,1,168,44Zm34,92a42,42,0,1,0,42,42A42,42,0,0,0,202,136Zm0,60a18,18,0,1,1,18-18A18,18,0,0,1,202,196ZM54,136a42,42,0,1,0,42,42A42,42,0,0,0,54,136Zm0,60a18,18,0,1,1,18-18A18,18,0,0,1,54,196Zm134-68H152a12,12,0,0,1-8.49-3.51L120,101l-15,15,31.52,31.51A12,12,0,0,1,140,156v48a12,12,0,0,1-24,0V161L79.51,124.49a12,12,0,0,1,0-17l32-32a12,12,0,0,1,17,0L157,104h31a12,12,0,0,1,0,24Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M164,78a26,26,0,1,0-26-26A26,26,0,0,0,164,78Zm0-40a14,14,0,1,1-14,14A14,14,0,0,1,164,38Zm36,100a38,38,0,1,0,38,38A38,38,0,0,0,200,138Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,200,202ZM56,138a38,38,0,1,0,38,38A38,38,0,0,0,56,138Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,56,202Zm136-84H152a6,6,0,0,1-4.24-1.76L120,88.49,96.49,112l35.75,35.76A6,6,0,0,1,134,152v48a6,6,0,0,1-12,0V154.49L83.76,116.24a6,6,0,0,1,0-8.48l32-32a6,6,0,0,1,8.48,0L154.49,106H192a6,6,0,0,1,0,12Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M164,80a28,28,0,1,0-28-28A28,28,0,0,0,164,80Zm0-40a12,12,0,1,1-12,12A12,12,0,0,1,164,40Zm36,96a40,40,0,1,0,40,40A40,40,0,0,0,200,136Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,200,200ZM56,136a40,40,0,1,0,40,40A40,40,0,0,0,56,136Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,56,200Zm136-80H152a8,8,0,0,1-5.66-2.34L120,91.31,99.31,112l34.35,34.34A8,8,0,0,1,136,152v48a8,8,0,0,1-16,0V155.31L82.34,117.66a8,8,0,0,1,0-11.32l32-32a8,8,0,0,1,11.32,0L155.31,104H192a8,8,0,0,1,0,16Z"/> }.into_view()
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