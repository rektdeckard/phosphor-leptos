/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn BriefcaseMetal(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M28,112H228a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H28a4,4,0,0,1-4-4V116A4,4,0,0,1,28,112Zm-4,88a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V180a4,4,0,0,0-4-4H28a4,4,0,0,0-4,4ZM232,72V92a4,4,0,0,1-4,4H28a4,4,0,0,1-4-4V72A16,16,0,0,1,40,56H80V48a24,24,0,0,1,24-24h48a24,24,0,0,1,24,24v8h40A16,16,0,0,1,232,72ZM160,48a8,8,0,0,0-8-8H104a8,8,0,0,0-8,8v8h64Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,72v32H32V72a8,8,0,0,1,8-8H216A8,8,0,0,1,224,72ZM32,200a8,8,0,0,0,8,8H216a8,8,0,0,0,8-8V168H32Z" opacity="0.2"/><path d="M216,56H176V48a24,24,0,0,0-24-24H104A24,24,0,0,0,80,48v8H40A16,16,0,0,0,24,72V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V72A16,16,0,0,0,216,56ZM40,112H216v48H40ZM96,48a8,8,0,0,1,8-8h48a8,8,0,0,1,8,8v8H96ZM216,72V96H40V72Zm0,128H40V176H216v24Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M216,60H172V48a20,20,0,0,0-20-20H104A20,20,0,0,0,84,48V60H40A12,12,0,0,0,28,72V200a12,12,0,0,0,12,12H216a12,12,0,0,0,12-12V72A12,12,0,0,0,216,60ZM36,108H220v56H36ZM92,48a12,12,0,0,1,12-12h48a12,12,0,0,1,12,12V60H92ZM40,68H216a4,4,0,0,1,4,4v28H36V72A4,4,0,0,1,40,68ZM216,204H40a4,4,0,0,1-4-4V172H220v28A4,4,0,0,1,216,204Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M216,52H180V40a28,28,0,0,0-28-28H104A28,28,0,0,0,76,40V52H40A20,20,0,0,0,20,72V200a20,20,0,0,0,20,20H216a20,20,0,0,0,20-20V72A20,20,0,0,0,216,52ZM44,116H212v40H44Zm56-76a4,4,0,0,1,4-4h48a4,4,0,0,1,4,4V52H100ZM212,76V92H44V76ZM44,196V180H212v16Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M216,58H174V48a22,22,0,0,0-22-22H104A22,22,0,0,0,82,48V58H40A14,14,0,0,0,26,72V200a14,14,0,0,0,14,14H216a14,14,0,0,0,14-14V72A14,14,0,0,0,216,58ZM38,110H218v52H38ZM94,48a10,10,0,0,1,10-10h48a10,10,0,0,1,10,10V58H94ZM40,70H216a2,2,0,0,1,2,2V98H38V72A2,2,0,0,1,40,70ZM216,202H40a2,2,0,0,1-2-2V174H218v26A2,2,0,0,1,216,202Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M216,56H176V48a24,24,0,0,0-24-24H104A24,24,0,0,0,80,48v8H40A16,16,0,0,0,24,72V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V72A16,16,0,0,0,216,56ZM40,112H216v48H40ZM96,48a8,8,0,0,1,8-8h48a8,8,0,0,1,8,8v8H96ZM216,72V96H40V72Zm0,128H40V176H216v24Z"/> }.into_view()
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