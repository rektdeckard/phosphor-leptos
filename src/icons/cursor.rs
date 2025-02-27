/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Cursor(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M219.31,192a16,16,0,0,1,0,22.63l-4.68,4.68a16,16,0,0,1-22.63,0l-55.25-55.24-21.88,50.34A15.84,15.84,0,0,1,100.26,224l-.78,0a15.82,15.82,0,0,1-14.41-11L32.8,52.92A15.95,15.95,0,0,1,52.92,32.8L213,85.07a16,16,0,0,1,1.41,29.8l-50.34,21.88Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M213.66,201,201,213.66a8,8,0,0,1-11.31,0L140,164a8,8,0,0,0-13,2.46l-19.46,44.77a8,8,0,0,1-14.85-.71L40.41,50.44a8,8,0,0,1,10-10L210.51,92.68a8,8,0,0,1,.71,14.85L166.45,127A8,8,0,0,0,164,140l49.67,49.67A8,8,0,0,1,213.66,201Z" opacity="0.2"/><path d="M169.64,134.33l44.77-19.46A16,16,0,0,0,213,85.07L52.92,32.8A16,16,0,0,0,32.8,52.92L85.07,213a15.83,15.83,0,0,0,14.41,11l.78,0a15.83,15.83,0,0,0,14.61-9.59h0l19.46-44.77L184,219.31a16,16,0,0,0,22.63,0l12.68-12.68a16,16,0,0,0,0-22.63Zm-69.48,73.76.06-.05Zm95.15-.09-49.67-49.67a16,16,0,0,0-26,4.94l-19.42,44.65L48,48l159.87,52.21-44.64,19.41a16,16,0,0,0-4.94,26L208,195.31Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M166.81,137.16a4,4,0,0,1,1.24-6.5l44.76-19.46a12,12,0,0,0-1.05-22.33L51.67,36.6A12,12,0,0,0,36.6,51.67L88.87,211.76A11.86,11.86,0,0,0,99.67,220h.58a11.86,11.86,0,0,0,11-7.19l19.46-44.76a3.92,3.92,0,0,1,2.92-2.34,4,4,0,0,1,3.58,1.1l49.67,49.68a12,12,0,0,0,17,0l12.69-12.69a12,12,0,0,0,0-17Zm44,61-12.69,12.69a4,4,0,0,1-5.66,0l-49.67-49.67a12,12,0,0,0-8.48-3.52,12.21,12.21,0,0,0-2.24.21,12,12,0,0,0-8.77,7l-19.46,44.76a4,4,0,0,1-7.39-.35L44.2,49.19a4,4,0,0,1,5-5L209.27,96.47a4,4,0,0,1,.35,7.39l-44.76,19.46a12,12,0,0,0-3.7,19.49l49.67,49.67A4,4,0,0,1,210.83,198.14Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M180.85,131.88l34.62-13.13.53-.21a20,20,0,0,0-1.76-37.27L54.16,29A20,20,0,0,0,29,54.16L81.27,214.24a19.81,19.81,0,0,0,18,13.74l1,0a19.81,19.81,0,0,0,18.27-12l.21-.53,13.13-34.62,41.29,41.29a20,20,0,0,0,28.29,0l20.68-20.68a20,20,0,0,0,0-28.29Zm6.46,70.46L144.47,159.5a20,20,0,0,0-14.13-5.86,20.5,20.5,0,0,0-3.74.35A20,20,0,0,0,112,165.67c-.08.17-.15.35-.22.53l-11.25,29.67L54.28,54.28l141.59,46.24L166.2,111.77l-.53.22a20,20,0,0,0-6.17,32.48l42.84,42.84Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M168.23,135.74a2,2,0,0,1,.62-3.25L213.61,113A14,14,0,0,0,212.38,87L52.29,34.7A13.95,13.95,0,0,0,34.7,52.29L87,212.38a13.82,13.82,0,0,0,12.6,9.6c.23,0,.46,0,.69,0A13.84,13.84,0,0,0,113,213.61h0l19.46-44.76a2,2,0,0,1,3.25-.62l49.67,49.67a14,14,0,0,0,19.8,0l12.69-12.69a14,14,0,0,0,0-19.8Zm41.18,61-12.68,12.68a2,2,0,0,1-2.83,0l-49.67-49.67a14,14,0,0,0-22.74,4.32L102,208.82a2,2,0,0,1-3.65-.17L46.11,48.57a1.87,1.87,0,0,1,.47-2A1.92,1.92,0,0,1,47.93,46a2.22,2.22,0,0,1,.64.1L208.65,98.38a2,2,0,0,1,.17,3.65l-44.76,19.46a14,14,0,0,0-4.32,22.74l49.67,49.67A2,2,0,0,1,209.41,196.73Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M169.64,134.33l44.77-19.46A16,16,0,0,0,213,85.07L52.92,32.8A15.95,15.95,0,0,0,32.8,52.92L85.07,213a15.82,15.82,0,0,0,14.41,11l.78,0a15.84,15.84,0,0,0,14.61-9.59h0l19.46-44.77L184,219.31a16,16,0,0,0,22.63,0l12.68-12.68a16,16,0,0,0,0-22.63Zm-69.48,73.76.06-.05Zm95.15-.09-49.67-49.67a16,16,0,0,0-26,4.93l-19.41,44.66L48,48l159.87,52.2-44.65,19.42a16,16,0,0,0-4.93,26L208,195.31Z"/> }.into_view()
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