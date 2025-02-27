/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn NoteBlank(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H156.69A15.86,15.86,0,0,0,168,219.31L219.31,168A15.86,15.86,0,0,0,224,156.69V48A16,16,0,0,0,208,32ZM160,204.69V160h44.69Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,160l-56,56V160Z" opacity="0.2"/><path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H156.69A15.86,15.86,0,0,0,168,219.31L219.31,168A15.86,15.86,0,0,0,224,156.69V48A16,16,0,0,0,208,32ZM48,48H208V152H160a8,8,0,0,0-8,8v48H48ZM196.69,168,168,196.69V168Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M208,36H48A12,12,0,0,0,36,48V208a12,12,0,0,0,12,12H156.69a11.93,11.93,0,0,0,8.48-3.51l51.32-51.32a11.93,11.93,0,0,0,3.51-8.48V48A12,12,0,0,0,208,36ZM44,208V48a4,4,0,0,1,4-4H208a4,4,0,0,1,4,4V156H160a4,4,0,0,0-4,4v52H48A4,4,0,0,1,44,208Zm120-1.66V164h42.35Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M208,28H48A20,20,0,0,0,28,48V208a20,20,0,0,0,20,20H156.69a19.86,19.86,0,0,0,14.14-5.86l51.31-51.31A19.86,19.86,0,0,0,228,156.69V48A20,20,0,0,0,208,28ZM52,52H204v92H156a12,12,0,0,0-12,12v48H52ZM168,191V168h23Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M208,34H48A14,14,0,0,0,34,48V208a14,14,0,0,0,14,14H156.69a13.94,13.94,0,0,0,9.9-4.1l51.31-51.31a13.94,13.94,0,0,0,4.1-9.9V48A14,14,0,0,0,208,34ZM46,208V48a2,2,0,0,1,2-2H208a2,2,0,0,1,2,2V154H160a6,6,0,0,0-6,6v50H48A2,2,0,0,1,46,208Zm120-6.49V166h35.52Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H156.69A15.86,15.86,0,0,0,168,219.31L219.31,168A15.86,15.86,0,0,0,224,156.69V48A16,16,0,0,0,208,32ZM48,48H208V152H160a8,8,0,0,0-8,8v48H48ZM196.69,168,168,196.69V168Z"/> }.into_view()
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