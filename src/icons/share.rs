/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Share(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M229.66,109.66l-48,48A8,8,0,0,1,168,152V112h-3a88,88,0,0,0-85.23,66,8,8,0,0,1-15.5-4A103.94,103.94,0,0,1,165,96h3V56a8,8,0,0,1,13.66-5.66l48,48A8,8,0,0,1,229.66,109.66ZM192,208H40V88a8,8,0,0,0-16,0V208a16,16,0,0,0,16,16H192a8,8,0,0,0,0-16Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,104l-48,48V56Z" opacity="0.2"/><path d="M229.66,98.34l-48-48A8,8,0,0,0,168,56V96h-3A103.94,103.94,0,0,0,64.25,174a8,8,0,0,0,15.5,4A88,88,0,0,1,165,112h3v40a8,8,0,0,0,13.66,5.66l48-48A8,8,0,0,0,229.66,98.34ZM184,132.69V75.31L212.69,104ZM200,216a8,8,0,0,1-8,8H40a16,16,0,0,1-16-16V88a8,8,0,0,1,16,0V208H192A8,8,0,0,1,200,216Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M226.83,106.83l-48,48a4,4,0,0,1-5.66-5.66L214.34,108H165a92,92,0,0,0-89.11,69A4,4,0,0,1,72,180a3.87,3.87,0,0,1-1-.13A4,4,0,0,1,68.13,175,99.93,99.93,0,0,1,165,100h49.36L173.17,58.83a4,4,0,0,1,5.66-5.66l48,48A4,4,0,0,1,226.83,106.83ZM192,212H40a4,4,0,0,1-4-4V88a4,4,0,0,0-8,0V208a12,12,0,0,0,12,12H192a4,4,0,0,0,0-8Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M232.49,112.49l-48,48a12,12,0,0,1-17-17L195,116H165a84,84,0,0,0-81.36,63,12,12,0,1,1-23.24-6A107.94,107.94,0,0,1,165,92H195L167.51,64.48a12,12,0,0,1,17-17l48,48A12,12,0,0,1,232.49,112.49ZM192,204H44V88a12,12,0,0,0-24,0V208a20,20,0,0,0,20,20H192a12,12,0,0,0,0-24Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M228.24,108.24l-48,48a6,6,0,0,1-8.48-8.48L209.51,110H165a89.94,89.94,0,0,0-87.17,67.5,6,6,0,0,1-11.62-3A101.94,101.94,0,0,1,165,98h44.53L171.76,60.24a6,6,0,0,1,8.48-8.48l48,48A6,6,0,0,1,228.24,108.24ZM192,210H40a2,2,0,0,1-2-2V88a6,6,0,0,0-12,0V208a14,14,0,0,0,14,14H192a6,6,0,0,0,0-12Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M229.66,109.66l-48,48a8,8,0,0,1-11.32-11.32L204.69,112H165a88,88,0,0,0-85.23,66,8,8,0,0,1-15.5-4A103.94,103.94,0,0,1,165,96h39.71L170.34,61.66a8,8,0,0,1,11.32-11.32l48,48A8,8,0,0,1,229.66,109.66ZM192,208H40V88a8,8,0,0,0-16,0V208a16,16,0,0,0,16,16H192a8,8,0,0,0,0-16Z"/> }.into_view()
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