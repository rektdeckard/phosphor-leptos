/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn FileJpg(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M48,120H208a8,8,0,0,0,8-8V88a8,8,0,0,0-2.34-5.66l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v72A8,8,0,0,0,48,120ZM152,44l44,44H152ZM120,144H104a8,8,0,0,0-8,8v56a8,8,0,0,0,16,0v-8h8a28,28,0,0,0,0-56Zm0,40h-8V160h8a12,12,0,0,1,0,24Zm96,16.87a8,8,0,0,1-2.22,5.53A30.06,30.06,0,0,1,192,216c-17.64,0-32-16.15-32-36s14.36-36,32-36a29.38,29.38,0,0,1,16.48,5.12,8,8,0,0,1-9,13.26A13.21,13.21,0,0,0,192,160c-8.82,0-16,9-16,20s7.18,20,16,20a13.63,13.63,0,0,0,8-2.71V192a8,8,0,0,1,0-16h8a8,8,0,0,1,8,8ZM80,152v38a26,26,0,0,1-52,0,8,8,0,0,1,16,0,10,10,0,0,0,20,0V152a8,8,0,0,1,16,0Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,88H152V32Z" opacity="0.2"/><path d="M120,144H104a8,8,0,0,0-8,8v56a8,8,0,0,0,16,0v-8h8a28,28,0,0,0,0-56Zm0,40h-8V160h8a12,12,0,0,1,0,24Zm96,0v16.87a8,8,0,0,1-2.22,5.53A30.06,30.06,0,0,1,192,216c-17.64,0-32-16.15-32-36s14.36-36,32-36a29.38,29.38,0,0,1,16.48,5.12,8,8,0,0,1-9,13.26A13.21,13.21,0,0,0,192,160c-8.82,0-16,9-16,20s7.18,20,16,20a13.63,13.63,0,0,0,8-2.71V192a8,8,0,0,1,0-16h8A8,8,0,0,1,216,184ZM80,152v38a26,26,0,0,1-52,0,8,8,0,0,1,16,0,10,10,0,0,0,20,0V152a8,8,0,0,1,16,0ZM213.66,82.34l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v72a8,8,0,0,0,16,0V40h88V88a8,8,0,0,0,8,8h48v16a8,8,0,0,0,16,0V88A8,8,0,0,0,213.66,82.34ZM160,80V51.31L188.69,80Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M120,148H104a4,4,0,0,0-4,4v56a4,4,0,0,0,8,0V196h12a24,24,0,0,0,0-48Zm0,40H108V156h12a16,16,0,0,1,0,32Zm92-4v16.87a4,4,0,0,1-1.11,2.77A26.11,26.11,0,0,1,192,212c-15.44,0-28-14.36-28-32s12.56-32,28-32a25.41,25.41,0,0,1,14.24,4.43,4,4,0,1,1-4.48,6.63A17.45,17.45,0,0,0,192,156c-11,0-20,10.77-20,24s9,24,20,24a17.87,17.87,0,0,0,12-4.82V188h-4a4,4,0,0,1,0-8h8A4,4,0,0,1,212,184ZM76,152v38a22,22,0,0,1-44,0,4,4,0,0,1,8,0,14,14,0,0,0,28,0V152a4,4,0,0,1,8,0ZM210.83,85.17l-56-56A4,4,0,0,0,152,28H56A12,12,0,0,0,44,40v72a4,4,0,0,0,8,0V40a4,4,0,0,1,4-4h92V88a4,4,0,0,0,4,4h52v20a4,4,0,0,0,8,0V88A4,4,0,0,0,210.83,85.17ZM156,84V41.65L198.34,84Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M120,140H104a12,12,0,0,0-12,12v56a12,12,0,0,0,24,0v-4h4a32,32,0,0,0,0-64Zm0,40h-4V164h4a8,8,0,0,1,0,16Zm104,20.87a12,12,0,0,1-3.33,8.3A34,34,0,0,1,196,220c-19.85,0-36-17.94-36-40s16.15-40,36-40a33.39,33.39,0,0,1,18.71,5.8,12,12,0,1,1-13.42,19.89A9.39,9.39,0,0,0,196,164c-6.5,0-12,7.33-12,16s5.5,16,12,16a9,9,0,0,0,2.7-.42A12,12,0,0,1,208,176h4a12,12,0,0,1,12,12ZM80,152v38a30,30,0,0,1-60,0,12,12,0,0,1,24,0,6,6,0,0,0,12,0V152a12,12,0,0,1,24,0ZM48,120a12,12,0,0,0,12-12V44h76V92a12,12,0,0,0,12,12h48v4a12,12,0,0,0,24,0V88a12,12,0,0,0-3.51-8.48l-56-56A12,12,0,0,0,152,20H56A20,20,0,0,0,36,40v68A12,12,0,0,0,48,120ZM160,57l23,23H160Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M120,146H104a6,6,0,0,0-6,6v56a6,6,0,0,0,12,0V198h10a26,26,0,0,0,0-52Zm0,40H110V158h10a14,14,0,0,1,0,28Zm94-2v16.87a6,6,0,0,1-1.67,4.15A28.06,28.06,0,0,1,192,214c-16.54,0-30-15.25-30-34s13.46-34,30-34a27.43,27.43,0,0,1,15.36,4.77,6,6,0,0,1-6.72,9.95A15.25,15.25,0,0,0,192,158c-9.93,0-18,9.87-18,22s8.07,22,18,22a15.75,15.75,0,0,0,10-3.73V190h-2a6,6,0,0,1,0-12h8A6,6,0,0,1,214,184ZM78,152v38a24,24,0,0,1-48,0,6,6,0,0,1,12,0,12,12,0,0,0,24,0V152a6,6,0,0,1,12,0ZM212.24,83.76l-56-56A6,6,0,0,0,152,26H56A14,14,0,0,0,42,40v72a6,6,0,0,0,12,0V40a2,2,0,0,1,2-2h90V88a6,6,0,0,0,6,6h50v18a6,6,0,0,0,12,0V88A6,6,0,0,0,212.24,83.76ZM158,82V46.48L193.52,82Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M120,144H104a8,8,0,0,0-8,8v56a8,8,0,0,0,16,0v-8h8a28,28,0,0,0,0-56Zm0,40h-8V160h8a12,12,0,0,1,0,24Zm96,0v16.87a8,8,0,0,1-2.22,5.53A30.06,30.06,0,0,1,192,216c-17.65,0-32-16.15-32-36s14.35-36,32-36a29.38,29.38,0,0,1,16.48,5.12,8,8,0,0,1-9,13.26A13.21,13.21,0,0,0,192,160c-8.82,0-16,9-16,20s7.18,20,16,20a13.63,13.63,0,0,0,8-2.71V192a8,8,0,0,1,0-16h8A8,8,0,0,1,216,184ZM80,152v38a26,26,0,0,1-52,0,8,8,0,0,1,16,0,10,10,0,0,0,20,0V152a8,8,0,0,1,16,0ZM213.66,82.34l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v72a8,8,0,0,0,16,0V40h88V88a8,8,0,0,0,8,8h48v16a8,8,0,0,0,16,0V88A8,8,0,0,0,213.66,82.34ZM160,80V51.31L188.69,80Z"/> }.into_view()
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