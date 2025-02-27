/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn CurrencyBtc(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M168,152a24,24,0,0,1-24,24H104V128h40A24,24,0,0,1,168,152Zm64-24A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-48,24a40,40,0,0,0-17.63-33.15A32,32,0,0,0,152,65V56a8,8,0,0,0-16,0v8H120V56a8,8,0,0,0-16,0v8H88a8,8,0,0,0,0,16v96a8,8,0,0,0,0,16h16v8a8,8,0,0,0,16,0v-8h16v8a8,8,0,0,0,16,0v-8.81A40.05,40.05,0,0,0,184,152ZM160,96a16,16,0,0,0-16-16H104v32h40A16,16,0,0,0,160,96Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M192,160a40,40,0,0,1-40,40H80V48h60a36,36,0,0,1,0,72h12A40,40,0,0,1,192,160Z" opacity="0.2"/><path d="M170.48,115.7A44,44,0,0,0,144,40.19V24a8,8,0,0,0-16,0V40H112V24a8,8,0,0,0-16,0V40H64a8,8,0,0,0,0,16h8V192H64a8,8,0,0,0,0,16H96v16a8,8,0,0,0,16,0V208h16v16a8,8,0,0,0,16,0V208h8a48,48,0,0,0,18.48-92.3ZM88,56h52a28,28,0,0,1,0,56H88Zm64,136H88V128h64a32,32,0,0,1,0,64Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M162.27,117.21A40,40,0,0,0,140,44V24a4,4,0,0,0-8,0V44H108V24a4,4,0,0,0-8,0V44H64a4,4,0,0,0,0,8H76V196H64a4,4,0,0,0,0,8h36v20a4,4,0,0,0,8,0V204h24v20a4,4,0,0,0,8,0V204h12a44,44,0,0,0,10.27-86.79ZM84,52h56a32,32,0,0,1,0,64H84Zm68,144H84V124h68a36,36,0,0,1,0,72Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M177.08,114.46A48,48,0,0,0,152,37.52V24a12,12,0,0,0-24,0V36H112V24a12,12,0,0,0-24,0V36H64a12,12,0,0,0,0,24h4V188H64a12,12,0,0,0,0,24H88v12a12,12,0,0,0,24,0V212h16v12a12,12,0,0,0,24,0V212a52,52,0,0,0,25.08-97.54ZM164,84a24,24,0,0,1-24,24H92V60h48A24,24,0,0,1,164,84ZM152,188H92V132h60a28,28,0,0,1,0,56Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M166.69,116.41A42,42,0,0,0,142,42.05V24a6,6,0,0,0-12,0V42H110V24a6,6,0,0,0-12,0V42H64a6,6,0,0,0,0,12H74V194H64a6,6,0,0,0,0,12H98v18a6,6,0,0,0,12,0V206h20v18a6,6,0,0,0,12,0V206h10a46,46,0,0,0,14.69-89.59ZM170,84a30,30,0,0,1-30,30H86V54h54A30,30,0,0,1,170,84ZM152,194H86V126h66a34,34,0,0,1,0,68Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M170.48,115.7A44,44,0,0,0,144,40.19V24a8,8,0,0,0-16,0V40H112V24a8,8,0,0,0-16,0V40H64a8,8,0,0,0,0,16h8V192H64a8,8,0,0,0,0,16H96v16a8,8,0,0,0,16,0V208h16v16a8,8,0,0,0,16,0V208h8a48,48,0,0,0,18.48-92.3ZM168,84a28,28,0,0,1-28,28H88V56h52A28,28,0,0,1,168,84ZM152,192H88V128h64a32,32,0,0,1,0,64Z"/> }.into_view()
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