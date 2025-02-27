/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn FolderNotchMinus(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M216,72H130.66L102.92,51.2A16,16,0,0,0,93.34,48H40A16,16,0,0,0,24,64V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V88A16,16,0,0,0,216,72ZM40,96V64H93.34l21.33,16L93.33,96Zm112,64H104a8,8,0,0,1,0-16h48a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M128,80,98.13,102.4a8,8,0,0,1-4.8,1.6H32V64a8,8,0,0,1,8-8H93.33a8,8,0,0,1,4.8,1.6Z" opacity="0.2"/><path d="M216,72H130.67L102.93,51.2a16.12,16.12,0,0,0-9.6-3.2H40A16,16,0,0,0,24,64V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V88A16,16,0,0,0,216,72ZM40,64H93.33l21.34,16L93.33,96H40ZM216,200H40V112H93.33a16.12,16.12,0,0,0,9.6-3.2L130.67,88H216Zm-64-56a8,8,0,0,1,0,16H104a8,8,0,0,1,0-16Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M216,76H129.33l-28.8-21.6a12.05,12.05,0,0,0-7.2-2.4H40A12,12,0,0,0,28,64V200a12,12,0,0,0,12,12H216a12,12,0,0,0,12-12V88A12,12,0,0,0,216,76ZM36,64a4,4,0,0,1,4-4H93.33a4,4,0,0,1,2.4.8L121.33,80,95.73,99.2a4,4,0,0,1-2.4.8H36ZM220,200a4,4,0,0,1-4,4H40a4,4,0,0,1-4-4V108H93.33a12.05,12.05,0,0,0,7.2-2.4L129.33,84H216a4,4,0,0,1,4,4Zm-64-48a4,4,0,0,1-4,4H104a4,4,0,0,1,0-8h48A4,4,0,0,1,156,152Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M216,68H132L105.33,48a20.12,20.12,0,0,0-12-4H40A20,20,0,0,0,20,64V200a20,20,0,0,0,20,20H216a20,20,0,0,0,20-20V88A20,20,0,0,0,216,68ZM44,68H92l16,12L92,92H44ZM212,196H44V116H93.33a20.12,20.12,0,0,0,12-4L132,92h80Zm-60-56a12,12,0,0,1,0,24H104a12,12,0,0,1,0-24Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M216,74H130L101.73,52.8a14,14,0,0,0-8.4-2.8H40A14,14,0,0,0,26,64V200a14,14,0,0,0,14,14H216a14,14,0,0,0,14-14V88A14,14,0,0,0,216,74ZM38,64a2,2,0,0,1,2-2H93.33a2,2,0,0,1,1.2.4L118,80,94.53,97.6a2,2,0,0,1-1.2.4H38ZM218,200a2,2,0,0,1-2,2H40a2,2,0,0,1-2-2V110H93.33a14,14,0,0,0,8.4-2.8L130,86h86a2,2,0,0,1,2,2Zm-60-48a6,6,0,0,1-6,6H104a6,6,0,0,1,0-12h48A6,6,0,0,1,158,152Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M216,72H130.67L102.93,51.2a16.12,16.12,0,0,0-9.6-3.2H40A16,16,0,0,0,24,64V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V88A16,16,0,0,0,216,72ZM40,64H93.33l21.34,16L93.33,96H40ZM216,200H40V112H93.33a16.12,16.12,0,0,0,9.6-3.2L130.67,88H216ZM104,144h48a8,8,0,0,1,0,16H104a8,8,0,0,1,0-16Z"/> }.into_view()
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