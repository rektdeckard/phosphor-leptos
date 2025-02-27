/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn TagChevron(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M237.3,136.88l-42.66,64A16,16,0,0,1,181.33,208H24a8,8,0,0,1-6.58-12.55L64,128,17.42,60.55A8,8,0,0,1,24,48H181.33a16,16,0,0,1,13.31,7.12l42.66,64A16,16,0,0,1,237.3,136.88Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M230.66,132.44l-42.67,64a8,8,0,0,1-6.66,3.56H24l46.65-67.56a8,8,0,0,0,0-8.88L24,56H181.33A8,8,0,0,1,188,59.56l42.67,64A8,8,0,0,1,230.66,132.44Z" opacity="0.2"/><path d="M237.3,119.12l-42.66-64A16,16,0,0,0,181.33,48H24a8,8,0,0,0-6.58,12.55L64,128,17.42,195.45A8,8,0,0,0,24,208H181.33a16,16,0,0,0,13.31-7.12l42.66-64A16,16,0,0,0,237.3,119.12Zm-56,72.88H39.25l38-55,.08-.1a16,16,0,0,0,0-17.76l-.08-.1-38-55H181.33L224,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M234,121.34l-42.67-64a12,12,0,0,0-10-5.34H24a4,4,0,0,0-3.29,6.27l46.61,67.51a4,4,0,0,1,0,4.39L20.71,197.73A4,4,0,0,0,24,204H181.33a12,12,0,0,0,10-5.34l42.67-64A12,12,0,0,0,234,121.34Zm-6.66,8.88-42.66,64a4,4,0,0,1-3.33,1.78H31.62L74,134.66a12,12,0,0,0,0-13.37L31.62,60H181.33a4,4,0,0,1,3.33,1.78l42.66,64A4,4,0,0,1,227.32,130.22Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M240.63,116.91,198,52.91A19.93,19.93,0,0,0,181.32,44H24a12,12,0,0,0-9.88,18.82l45,65.18-45,65.18A12,12,0,0,0,24,212H181.32A19.93,19.93,0,0,0,198,203.09l42.67-64A19.94,19.94,0,0,0,240.63,116.91ZM179.18,188H46.87l33.65-48.74a1.63,1.63,0,0,0,.11-.17,19.91,19.91,0,0,0,0-22.18,1.63,1.63,0,0,0-.11-.17L46.87,68H179.18l40,60Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M235.64,120.23,193,56.23A14,14,0,0,0,181.33,50H24a6,6,0,0,0-4.94,9.41l46.6,67.48a2,2,0,0,1,0,2.14L19.06,196.59A6,6,0,0,0,24,206H181.33A14,14,0,0,0,193,199.77l42.67-64A14,14,0,0,0,235.64,120.23Zm-10,8.88-42.67,64a2,2,0,0,1-1.66.89H35.43l40.21-58.23a14,14,0,0,0,0-15.62L35.43,62h145.9a2,2,0,0,1,1.66.89l42.67,64A2,2,0,0,1,225.66,129.11Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M237.3,119.12l-42.66-64A16,16,0,0,0,181.33,48H24a8,8,0,0,0-6.58,12.55L64,128,17.42,195.45A8,8,0,0,0,24,208H181.33a16,16,0,0,0,13.31-7.12l42.66-64A16,16,0,0,0,237.3,119.12Zm-56,72.88H39.25l38-55,.08-.1a16,16,0,0,0,0-17.76l-.08-.1-38-55H181.33L224,128Z"/> }.into_view()
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