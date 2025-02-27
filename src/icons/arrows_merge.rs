/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn ArrowsMerge(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M192,40v64a8,8,0,0,1-2.34,5.66L136,163.31V192h24a8,8,0,0,1,5.66,13.66l-32,32a8,8,0,0,1-11.32,0l-32-32A8,8,0,0,1,96,192h24V163.31L66.34,109.66A8,8,0,0,1,64,104V40a8,8,0,0,1,16,0v60.69l48,48,48-48V40a8,8,0,0,1,16,0Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M160,200l-32,32L96,200Z" opacity="0.2"/><path d="M184,32a8,8,0,0,0-8,8v60.69l-48,48-48-48V40a8,8,0,0,0-16,0v64a8,8,0,0,0,2.34,5.66L120,163.31V192H96a8,8,0,0,0-5.66,13.66l32,32a8,8,0,0,0,11.32,0l32-32A8,8,0,0,0,160,192H136V163.31l53.66-53.65A8,8,0,0,0,192,104V40A8,8,0,0,0,184,32ZM128,220.69,115.31,208h25.38Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M188,40v64a4,4,0,0,1-1.17,2.83L132,161.66v60.68l25.17-25.17a4,4,0,0,1,5.66,5.66l-32,32a4,4,0,0,1-5.66,0l-32-32a4,4,0,0,1,5.66-5.66L124,222.34V161.66L69.17,106.83A4,4,0,0,1,68,104V40a4,4,0,0,1,8,0v62.34l52,52,52-52V40a4,4,0,0,1,8,0Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M196,40v64a12,12,0,0,1-3.51,8.49L140,165V203l11.51-11.52a12,12,0,0,1,17,17l-32,32a12,12,0,0,1-17,0l-32-32a12,12,0,0,1,17-17L116,203V165L63.51,112.49A12,12,0,0,1,60,104V40a12,12,0,0,1,24,0V99l44,44,44-44V40a12,12,0,0,1,24,0Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M190,40v64a6,6,0,0,1-1.76,4.24L134,162.49v55l21.76-21.75a6,6,0,0,1,8.48,8.48l-32,32a6,6,0,0,1-8.48,0l-32-32a6,6,0,0,1,8.48-8.48L122,217.51v-55L67.76,108.24A6,6,0,0,1,66,104V40a6,6,0,0,1,12,0v61.51l50,50,50-50V40a6,6,0,0,1,12,0Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M192,40v64a8,8,0,0,1-2.34,5.66L136,163.31v49.38l18.34-18.35a8,8,0,0,1,11.32,11.32l-32,32a8,8,0,0,1-11.32,0l-32-32a8,8,0,0,1,11.32-11.32L120,212.69V163.31L66.34,109.66A8,8,0,0,1,64,104V40a8,8,0,0,1,16,0v60.69l48,48,48-48V40a8,8,0,0,1,16,0Z"/> }.into_view()
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