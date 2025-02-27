/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn HouseLine(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M240,208H224V115.55a16,16,0,0,0-5.17-11.78l-80-75.48a1.14,1.14,0,0,1-.11-.11,16,16,0,0,0-21.53,0l-.11.11L37.17,103.77A16,16,0,0,0,32,115.55V208H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16Zm-88,0H104V160a8,8,0,0,1,8-8h32a8,8,0,0,1,8,8Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,115.54V216H152V160a8,8,0,0,0-8-8H112a8,8,0,0,0-8,8v56H40V115.54a8,8,0,0,1,2.62-5.92l80-75.54a8,8,0,0,1,10.77,0l80,75.54A8,8,0,0,1,216,115.54Z" opacity="0.2"/><path d="M240,208H224V115.55a16,16,0,0,0-5.17-11.78l-80-75.48a1.14,1.14,0,0,1-.11-.11,16,16,0,0,0-21.53,0l-.11.11L37.17,103.77A16,16,0,0,0,32,115.55V208H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM48,115.55l.11-.1L128,40l79.9,75.43.11.1V208H160V160a16,16,0,0,0-16-16H112a16,16,0,0,0-16,16v48H48ZM144,208H112V160h32Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M240,212H220V115.54a12,12,0,0,0-3.87-8.82L136.07,31.13a12,12,0,0,0-16.2.05L39.93,106.67A12,12,0,0,0,36,115.54V212H16a4,4,0,0,0,0,8H240a4,4,0,0,0,0-8ZM44,115.54a4.09,4.09,0,0,1,1.36-3L125.3,37.05a4,4,0,0,1,5.33,0l80.06,75.58a4,4,0,0,1,1.31,3V212H156V160a12,12,0,0,0-12-12H112a12,12,0,0,0-12,12v52H44ZM148,212H108V160a4,4,0,0,1,4-4h32a4,4,0,0,1,4,4Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M240,204H228V115.55a20.07,20.07,0,0,0-6.44-14.7L141.61,25.38l-.16-.15a19.93,19.93,0,0,0-26.91,0l-.17.15L34.44,100.85A20.07,20.07,0,0,0,28,115.55V204H16a12,12,0,0,0,0,24H240a12,12,0,0,0,0-24ZM52,117.28l76-71.75,76,71.75V204H164V160a20,20,0,0,0-20-20H112a20,20,0,0,0-20,20v44H52ZM140,204H116V164h24Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M240,210H222V115.55a14.06,14.06,0,0,0-4.53-10.32l-80-75.49-.09-.08a13.94,13.94,0,0,0-18.83,0l-.09.08-80,75.5A14,14,0,0,0,34,115.55V210H16a6,6,0,0,0,0,12H240a6,6,0,0,0,0-12ZM46,115.55a2,2,0,0,1,.65-1.48l.09-.08,79.94-75.48a2,2,0,0,1,2.63,0L209.26,114l.08.08a2,2,0,0,1,.66,1.48V210H158V160a14,14,0,0,0-14-14H112a14,14,0,0,0-14,14v50H46ZM146,210H110V160a2,2,0,0,1,2-2h32a2,2,0,0,1,2,2Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M240,208H224V115.55a16,16,0,0,0-5.17-11.78l-80-75.48a1.14,1.14,0,0,1-.11-.11,16,16,0,0,0-21.53,0l-.11.11L37.17,103.77A16,16,0,0,0,32,115.55V208H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM48,115.55l.11-.1L128,40l79.9,75.43.11.1V208H160V160a16,16,0,0,0-16-16H112a16,16,0,0,0-16,16v48H48ZM144,208H112V160h32Z"/> }.into_view()
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