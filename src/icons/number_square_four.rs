/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn NumberSquareFour(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM160,152h-8v24a8,8,0,0,1-16,0V152H96a8,8,0,0,1-7.59-10.53l24-72a8,8,0,0,1,15.18,5.06L107.1,136H136V112a8,8,0,0,1,16,0v24h8a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,48V208a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V48a8,8,0,0,1,8-8H208A8,8,0,0,1,216,48Z" opacity="0.2"/><path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32Zm0,176H48V48H208V208Zm-40-64a8,8,0,0,1-8,8h-8v24a8,8,0,0,1-16,0V152H96a8,8,0,0,1-7.59-10.53l24-72a8,8,0,0,1,15.18,5.06L107.1,136H136V112a8,8,0,0,1,16,0v24h8A8,8,0,0,1,168,144Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M208,36H48A12,12,0,0,0,36,48V208a12,12,0,0,0,12,12H208a12,12,0,0,0,12-12V48A12,12,0,0,0,208,36Zm4,172a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V48a4,4,0,0,1,4-4H208a4,4,0,0,1,4,4Zm-48-64a4,4,0,0,1-4,4H148v28a4,4,0,0,1-8,0V148H96a4,4,0,0,1-3.79-5.27l24-72a4,4,0,0,1,7.58,2.54L101.55,140H140V112a4,4,0,0,1,8,0v28h12A4,4,0,0,1,164,144Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M208,28H48A20,20,0,0,0,28,48V208a20,20,0,0,0,20,20H208a20,20,0,0,0,20-20V48A20,20,0,0,0,208,28Zm-4,176H52V52H204ZM86.2,150.93A12,12,0,0,1,84.68,140l24-68a12,12,0,1,1,22.64,8L113,132h19V116a12,12,0,0,1,24,0v16h4a12,12,0,0,1,0,24h-4v20a12,12,0,0,1-24,0V156H96A12,12,0,0,1,86.2,150.93Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M208,34H48A14,14,0,0,0,34,48V208a14,14,0,0,0,14,14H208a14,14,0,0,0,14-14V48A14,14,0,0,0,208,34Zm2,174a2,2,0,0,1-2,2H48a2,2,0,0,1-2-2V48a2,2,0,0,1,2-2H208a2,2,0,0,1,2,2Zm-44-64a6,6,0,0,1-6,6H150v26a6,6,0,0,1-12,0V150H96a6,6,0,0,1-5.69-7.9l24-72a6,6,0,1,1,11.38,3.8L104.32,138H138V112a6,6,0,0,1,12,0v26h10A6,6,0,0,1,166,144Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32Zm0,176H48V48H208V208Zm-40-64a8,8,0,0,1-8,8h-8v24a8,8,0,0,1-16,0V152H96a8,8,0,0,1-7.59-10.53l24-72a8,8,0,0,1,15.18,5.06L107.1,136H136V112a8,8,0,0,1,16,0v24h8A8,8,0,0,1,168,144Z"/> }.into_view()
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