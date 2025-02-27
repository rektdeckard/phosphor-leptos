/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn CarSimple(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M240,112H229.2L201.42,49.5A16,16,0,0,0,186.8,40H69.2a16,16,0,0,0-14.62,9.5L26.8,112H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V192h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V128h8a8,8,0,0,0,0-16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,120H32L61.89,52.75A8,8,0,0,1,69.2,48H186.8a8,8,0,0,1,7.31,4.75Z" opacity="0.2"/><path d="M240,112H229.2L201.42,49.5A16,16,0,0,0,186.8,40H69.2a16,16,0,0,0-14.62,9.5L26.8,112H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V192h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V128h8a8,8,0,0,0,0-16ZM69.2,56H186.8l24.89,56H44.31ZM216,208H192V184a8,8,0,0,0-8-8H72a8,8,0,0,0-8,8v24H40V128H216Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M240,116H226.6L197.77,51.13a12,12,0,0,0-11-7.13H69.2a12,12,0,0,0-11,7.13L29.4,116H16a4,4,0,0,0,0,8H28v84a12,12,0,0,0,12,12H64a12,12,0,0,0,12-12V188H180v20a12,12,0,0,0,12,12h24a12,12,0,0,0,12-12V124h12a4,4,0,0,0,0-8ZM65.54,54.38A4,4,0,0,1,69.2,52H186.8a4,4,0,0,1,3.66,2.38L217.84,116H38.16ZM220,208a4,4,0,0,1-4,4H192a4,4,0,0,1-4-4V184a4,4,0,0,0-4-4H72a4,4,0,0,0-4,4v24a4,4,0,0,1-4,4H40a4,4,0,0,1-4-4V124H220Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M240,108h-8.2L205.08,47.88A20,20,0,0,0,186.8,36H69.2A20,20,0,0,0,50.92,47.88L24.2,108H16a12,12,0,0,0,0,24h4v76a20,20,0,0,0,20,20H64a20,20,0,0,0,20-20V196h88v12a20,20,0,0,0,20,20h24a20,20,0,0,0,20-20V132h4a12,12,0,0,0,0-24ZM71.8,60H184.2l21.33,48H50.47ZM212,204H196V184a12,12,0,0,0-12-12H72a12,12,0,0,0-12,12v20H44V132H212Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M240,114H227.9L199.59,50.31A14,14,0,0,0,186.8,42H69.2a14,14,0,0,0-12.79,8.31L28.1,114H16a6,6,0,0,0,0,12H26v82a14,14,0,0,0,14,14H64a14,14,0,0,0,14-14V190H178v18a14,14,0,0,0,14,14h24a14,14,0,0,0,14-14V126h10a6,6,0,0,0,0-12ZM67.37,55.19A2,2,0,0,1,69.2,54H186.8a2,2,0,0,1,1.83,1.19L214.77,114H41.23ZM218,208a2,2,0,0,1-2,2H192a2,2,0,0,1-2-2V184a6,6,0,0,0-6-6H72a6,6,0,0,0-6,6v24a2,2,0,0,1-2,2H40a2,2,0,0,1-2-2V126H218Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M240,112H229.2L201.42,49.5A16,16,0,0,0,186.8,40H69.2a16,16,0,0,0-14.62,9.5L26.8,112H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V192h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V128h8a8,8,0,0,0,0-16ZM69.2,56H186.8l24.89,56H44.31ZM216,208H192V184a8,8,0,0,0-8-8H72a8,8,0,0,0-8,8v24H40V128H216Z"/> }.into_view()
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