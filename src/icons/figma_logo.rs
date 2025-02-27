/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn FigmaLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M184,96a40,40,0,0,0-24-72H88A40,40,0,0,0,64,96a40,40,0,0,0,1.37,65A44,44,0,1,0,136,196V160a40,40,0,1,0,48-64ZM136,40h24a24,24,0,0,1,0,48H136Zm24,112a24,24,0,1,1,24-24A24,24,0,0,1,160,152Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M192,128a32,32,0,1,1-32-32A32,32,0,0,1,192,128ZM88,96h40V32H88a32,32,0,0,0,0,64ZM56,196a36,36,0,0,0,72,0V160H92A36,36,0,0,0,56,196Z" opacity="0.2"/><path d="M184,96a40,40,0,0,0-24-72H88A40,40,0,0,0,64,96a40,40,0,0,0,1.37,65A44,44,0,1,0,136,196V160a40,40,0,1,0,48-64Zm0-32a24,24,0,0,1-24,24H136V40h24A24,24,0,0,1,184,64ZM64,64A24,24,0,0,1,88,40h32V88H88A24,24,0,0,1,64,64Zm24,88a24,24,0,0,1,0-48h32v48H88Zm32,44a28,28,0,1,1-28-28h28Zm40-44a24,24,0,1,1,24-24A24,24,0,0,1,160,152Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M176.46,96A36,36,0,0,0,160,28H88A36,36,0,0,0,71.54,96a36,36,0,0,0,1.56,64.76A40,40,0,1,0,132,196V150.59A36,36,0,1,0,176.46,96ZM188,64a28,28,0,0,1-28,28H132V36h28A28,28,0,0,1,188,64Zm-56,36h5.41a36.41,36.41,0,0,0-5.41,5.41ZM60,64A28,28,0,0,1,88,36h36V92H88A28,28,0,0,1,60,64Zm64,132a32,32,0,1,1-32-32h32Zm0-40H88a28,28,0,0,1,0-56h36Zm36,0a28,28,0,1,1,28-28A28,28,0,0,1,160,156Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M190.15,96A44,44,0,0,0,160,20H88A44,44,0,0,0,57.85,96a43.9,43.9,0,0,0,1.23,65.12A48,48,0,1,0,140,196V167.17A44,44,0,0,0,190.15,96ZM180,64a20,20,0,0,1-20,20H140V44h20A20,20,0,0,1,180,64ZM68,64A20,20,0,0,1,88,44h28V84H88A20,20,0,0,1,68,64Zm20,84a20,20,0,0,1,0-40h28v40H88Zm28,48a24,24,0,1,1-24-24h24Zm44-48a20,20,0,1,1,20-20A20,20,0,0,1,160,148Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M180.45,96A38,38,0,0,0,160,26H88A38,38,0,0,0,67.55,96,38,38,0,0,0,69,160.89,42,42,0,1,0,134,196V155.68A38,38,0,1,0,180.45,96ZM186,64a26,26,0,0,1-26,26H134V38h26A26,26,0,0,1,186,64ZM62,64A26,26,0,0,1,88,38h34V90H88A26,26,0,0,1,62,64Zm26,90a26,26,0,0,1,0-52h34v52H88Zm34,42a30,30,0,1,1-30-30h30Zm38-42a26,26,0,1,1,26-26A26,26,0,0,1,160,154Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M184,96a40,40,0,0,0-24-72H88A40,40,0,0,0,64,96a40,40,0,0,0,1.37,65A44,44,0,1,0,136,196V160a40,40,0,1,0,48-64Zm0-32a24,24,0,0,1-24,24H136V40h24A24,24,0,0,1,184,64ZM64,64A24,24,0,0,1,88,40h32V88H88A24,24,0,0,1,64,64Zm24,88a24,24,0,0,1,0-48h32v48H88Zm32,44a28,28,0,1,1-28-28h28Zm40-44a24,24,0,1,1,24-24A24,24,0,0,1,160,152Z"/> }.into_view()
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