/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ChatCenteredDots(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M216,40H40A16,16,0,0,0,24,56V184a16,16,0,0,0,16,16l59.5.06,14.78,24.17a16,16,0,0,0,27.41.06L156.53,200H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM84,132a12,12,0,1,1,12-12A12,12,0,0,1,84,132Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,128,132Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,172,132Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,56V184a8,8,0,0,1-8,8H156.53a8,8,0,0,0-6.86,3.88l-14.81,24.24a8,8,0,0,1-13.72,0l-14.81-24.24A8,8,0,0,0,99.47,192H40a8,8,0,0,1-8-8V56a8,8,0,0,1,8-8H216A8,8,0,0,1,224,56Z" opacity="0.2"/><path d="M116,120a12,12,0,1,1,12,12A12,12,0,0,1,116,120Zm56,12a12,12,0,1,0-12-12A12,12,0,0,0,172,132Zm60-76V184a16,16,0,0,1-16,16H156.53l-14.84,24.29a16,16,0,0,1-27.41-.06L99.47,200H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56Zm-16,0H40V184H99.47a16.08,16.08,0,0,1,13.7,7.73L128,216l14.82-24.32A16.07,16.07,0,0,1,156.53,184H216ZM84,132a12,12,0,1,0-12-12A12,12,0,0,0,84,132Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M216,44H40A12,12,0,0,0,28,56V184a12,12,0,0,0,12,12H99.47a4,4,0,0,1,3.43,1.94l0,0,14.79,24.2a12,12,0,0,0,20.56,0l14.83-24.26a4,4,0,0,1,3.43-1.94H216a12,12,0,0,0,12-12V56A12,12,0,0,0,216,44Zm4,140a4,4,0,0,1-4,4H156.53a12,12,0,0,0-10.27,5.8l-14.83,24.26a4,4,0,0,1-6.88,0l-14.8-24.22A12,12,0,0,0,99.47,188H40a4,4,0,0,1-4-4V56a4,4,0,0,1,4-4H216a4,4,0,0,1,4,4Zm-84-64a8,8,0,1,1-8-8A8,8,0,0,1,136,120Zm-44,0a8,8,0,1,1-8-8A8,8,0,0,1,92,120Zm88,0a8,8,0,1,1-8-8A8,8,0,0,1,180,120Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M216,36H40A20,20,0,0,0,20,56V184a20,20,0,0,0,20,20H97.23l13.62,22.29a20,20,0,0,0,34.25.08L158.77,204H216a20,20,0,0,0,20-20V56A20,20,0,0,0,216,36Zm-4,144H156.53a20,20,0,0,0-17.1,9.63L128,208.33l-11.41-18.67A20.1,20.1,0,0,0,99.47,180H44V60H212ZM88,120a16,16,0,1,1,16,16A16,16,0,0,1,88,120Zm48,0a16,16,0,1,1,16,16A16,16,0,0,1,136,120Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M216,42H40A14,14,0,0,0,26,56V184a14,14,0,0,0,14,14H99.47a2,2,0,0,1,1.71,1l0,0L116,223.2a14,14,0,0,0,24,.05L154.82,199a2,2,0,0,1,1.71-1H216a14,14,0,0,0,14-14V56A14,14,0,0,0,216,42Zm2,142a2,2,0,0,1-2,2H156.53a14,14,0,0,0-12,6.75L129.72,217a2,2,0,0,1-3.46,0l-14.8-24.22a14.09,14.09,0,0,0-12-6.77H40a2,2,0,0,1-2-2V56a2,2,0,0,1,2-2H216a2,2,0,0,1,2,2Zm-80-64a10,10,0,1,1-10-10A10,10,0,0,1,138,120Zm-44,0a10,10,0,1,1-10-10A10,10,0,0,1,94,120Zm88,0a10,10,0,1,1-10-10A10,10,0,0,1,182,120Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M116,120a12,12,0,1,1,12,12A12,12,0,0,1,116,120Zm56,12a12,12,0,1,0-12-12A12,12,0,0,0,172,132Zm60-76V184a16,16,0,0,1-16,16H156.53l-14.84,24.29a16,16,0,0,1-27.41-.06L99.47,200H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56Zm-16,0H40V184H99.47a16.08,16.08,0,0,1,13.7,7.73L128,216l14.82-24.32A16.07,16.07,0,0,1,156.53,184H216ZM84,132a12,12,0,1,0-12-12A12,12,0,0,0,84,132Z"/> }.into_view()
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