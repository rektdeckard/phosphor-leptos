/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Notepad(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M200,32H184V24a8,8,0,0,0-16,0v8H136V24a8,8,0,0,0-16,0v8H88V24a8,8,0,0,0-16,0v8H56A16,16,0,0,0,40,48V200a32,32,0,0,0,32,32H184a32,32,0,0,0,32-32V48A16,16,0,0,0,200,32ZM160,168H96a8,8,0,0,1,0-16h64a8,8,0,0,1,0,16Zm0-32H96a8,8,0,0,1,0-16h64a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,48V200a24,24,0,0,1-24,24H72a24,24,0,0,1-24-24V48a8,8,0,0,1,8-8H200A8,8,0,0,1,208,48Z" opacity="0.2"/><path d="M168,128a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h64A8,8,0,0,1,168,128Zm-8,24H96a8,8,0,0,0,0,16h64a8,8,0,0,0,0-16ZM216,48V200a32,32,0,0,1-32,32H72a32,32,0,0,1-32-32V48A16,16,0,0,1,56,32H72V24a8,8,0,0,1,16,0v8h32V24a8,8,0,0,1,16,0v8h32V24a8,8,0,0,1,16,0v8h16A16,16,0,0,1,216,48Zm-16,0H184v8a8,8,0,0,1-16,0V48H136v8a8,8,0,0,1-16,0V48H88v8a8,8,0,0,1-16,0V48H56V200a16,16,0,0,0,16,16H184a16,16,0,0,0,16-16Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M164,128a4,4,0,0,1-4,4H96a4,4,0,0,1,0-8h64A4,4,0,0,1,164,128Zm-4,28H96a4,4,0,0,0,0,8h64a4,4,0,0,0,0-8ZM212,48V200a28,28,0,0,1-28,28H72a28,28,0,0,1-28-28V48A12,12,0,0,1,56,36H76V24a4,4,0,0,1,8,0V36h40V24a4,4,0,0,1,8,0V36h40V24a4,4,0,0,1,8,0V36h20A12,12,0,0,1,212,48Zm-8,0a4,4,0,0,0-4-4H180V56a4,4,0,0,1-8,0V44H132V56a4,4,0,0,1-8,0V44H84V56a4,4,0,0,1-8,0V44H56a4,4,0,0,0-4,4V200a20,20,0,0,0,20,20H184a20,20,0,0,0,20-20Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M172,124a12,12,0,0,1-12,12H96a12,12,0,0,1,0-24h64A12,12,0,0,1,172,124Zm-12,28H96a12,12,0,0,0,0,24h64a12,12,0,0,0,0-24ZM220,48V200a36,36,0,0,1-36,36H72a36,36,0,0,1-36-36V48A20,20,0,0,1,56,28H72V24a12,12,0,0,1,24,0v4h20V24a12,12,0,0,1,24,0v4h20V24a12,12,0,0,1,24,0v4h16A20,20,0,0,1,220,48Zm-24,4H184v4a12,12,0,0,1-24,0V52H140v4a12,12,0,0,1-24,0V52H96v4a12,12,0,0,1-24,0V52H60V200a12,12,0,0,0,12,12H184a12,12,0,0,0,12-12Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M166,128a6,6,0,0,1-6,6H96a6,6,0,0,1,0-12h64A6,6,0,0,1,166,128Zm-6,26H96a6,6,0,0,0,0,12h64a6,6,0,0,0,0-12ZM214,48V200a30,30,0,0,1-30,30H72a30,30,0,0,1-30-30V48A14,14,0,0,1,56,34H74V24a6,6,0,0,1,12,0V34h36V24a6,6,0,0,1,12,0V34h36V24a6,6,0,0,1,12,0V34h18A14,14,0,0,1,214,48Zm-12,0a2,2,0,0,0-2-2H182V56a6,6,0,0,1-12,0V46H134V56a6,6,0,0,1-12,0V46H86V56a6,6,0,0,1-12,0V46H56a2,2,0,0,0-2,2V200a18,18,0,0,0,18,18H184a18,18,0,0,0,18-18Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M168,128a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h64A8,8,0,0,1,168,128Zm-8,24H96a8,8,0,0,0,0,16h64a8,8,0,0,0,0-16ZM216,48V200a32,32,0,0,1-32,32H72a32,32,0,0,1-32-32V48A16,16,0,0,1,56,32H72V24a8,8,0,0,1,16,0v8h32V24a8,8,0,0,1,16,0v8h32V24a8,8,0,0,1,16,0v8h16A16,16,0,0,1,216,48Zm-16,0H184v8a8,8,0,0,1-16,0V48H136v8a8,8,0,0,1-16,0V48H88v8a8,8,0,0,1-16,0V48H56V200a16,16,0,0,0,16,16H184a16,16,0,0,0,16-16Z"/> }.into_view()
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