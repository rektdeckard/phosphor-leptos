/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArrowFatRight(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M237.66,133.66l-96,96A8,8,0,0,1,128,224V184H48a16,16,0,0,1-16-16V88A16,16,0,0,1,48,72h80V32a8,8,0,0,1,13.66-5.66l96,96A8,8,0,0,1,237.66,133.66Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M136,224V176H48a8,8,0,0,1-8-8V88a8,8,0,0,1,8-8h88V32l96,96Z" opacity="0.2"/><path d="M237.66,122.34l-96-96A8,8,0,0,0,128,32V72H48A16,16,0,0,0,32,88v80a16,16,0,0,0,16,16h80v40a8,8,0,0,0,13.66,5.66l96-96A8,8,0,0,0,237.66,122.34ZM144,204.69V176a8,8,0,0,0-8-8H48V88h88a8,8,0,0,0,8-8V51.31L220.69,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M234.83,125.17l-96-96A4,4,0,0,0,132,32V76H48A12,12,0,0,0,36,88v80a12,12,0,0,0,12,12h84v44a4,4,0,0,0,2.47,3.7,4,4,0,0,0,4.36-.87l96-96A4,4,0,0,0,234.83,125.17ZM140,214.34V176a4,4,0,0,0-4-4H48a4,4,0,0,1-4-4V88a4,4,0,0,1,4-4h88a4,4,0,0,0,4-4V41.66L226.34,128Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M240.49,119.51l-96-96A12,12,0,0,0,124,32V68H48A20,20,0,0,0,28,88v80a20,20,0,0,0,20,20h76v36a12,12,0,0,0,20.49,8.49l96-96A12,12,0,0,0,240.49,119.51ZM148,195V176a12,12,0,0,0-12-12H52V92h84a12,12,0,0,0,12-12V61l67,67Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M236.24,123.76l-96-96A6,6,0,0,0,130,32V74H48A14,14,0,0,0,34,88v80a14,14,0,0,0,14,14h82v42a6,6,0,0,0,10.24,4.24l96-96A6,6,0,0,0,236.24,123.76ZM142,209.51V176a6,6,0,0,0-6-6H48a2,2,0,0,1-2-2V88a2,2,0,0,1,2-2h88a6,6,0,0,0,6-6V46.49L223.51,128Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M237.66,122.34l-96-96A8,8,0,0,0,128,32V72H48A16,16,0,0,0,32,88v80a16,16,0,0,0,16,16h80v40a8,8,0,0,0,13.66,5.66l96-96A8,8,0,0,0,237.66,122.34ZM144,204.69V176a8,8,0,0,0-8-8H48V88h88a8,8,0,0,0,8-8V51.31L220.69,128Z"/> }.into_view()
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