/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn PaintRoller(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M248,104v50a16.07,16.07,0,0,1-11.6,15.38L136,198v34a8,8,0,0,1-16,0V198a16.07,16.07,0,0,1,11.6-15.38L232,154V104H216v24a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V104H16a8,8,0,0,1,0-16H32V64A16,16,0,0,1,48,48H200a16,16,0,0,1,16,16V88h16A16,16,0,0,1,248,104Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,64v64a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H200A8,8,0,0,1,208,64Z" opacity="0.2"/><path d="M232,88H216V64a16,16,0,0,0-16-16H48A16,16,0,0,0,32,64V88H16a8,8,0,0,0,0,16H32v24a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V104h16v50L131.6,182.65A16.07,16.07,0,0,0,120,198v34a8,8,0,0,0,16,0V198l100.4-28.68A16.07,16.07,0,0,0,248,154V104A16,16,0,0,0,232,88Zm-32,40H48V64H200v64Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M232,92H212V64a12,12,0,0,0-12-12H48A12,12,0,0,0,36,64V92H16a4,4,0,0,0,0,8H36v28a12,12,0,0,0,12,12H200a12,12,0,0,0,12-12V100h20a4,4,0,0,1,4,4v50a4,4,0,0,1-2.9,3.84L132.7,186.5A12,12,0,0,0,124,198v34a4,4,0,0,0,8,0V198a4,4,0,0,1,2.9-3.84L235.3,165.5A12,12,0,0,0,244,154V104A12,12,0,0,0,232,92Zm-28,36a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4H200a4,4,0,0,1,4,4Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M232,84H216V64a20,20,0,0,0-20-20H52A20,20,0,0,0,32,64V84H16a12,12,0,0,0,0,24H32v20a20,20,0,0,0,20,20H196a20,20,0,0,0,20-20V108h12V151L130.5,178.8A20.09,20.09,0,0,0,116,198v34a12,12,0,0,0,24,0V201.05l97.5-27.85A20.09,20.09,0,0,0,252,154V104A20,20,0,0,0,232,84Zm-40,40H56V68H192Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M232,90H214V64a14,14,0,0,0-14-14H48A14,14,0,0,0,34,64V90H16a6,6,0,0,0,0,12H34v26a14,14,0,0,0,14,14H200a14,14,0,0,0,14-14V102h18a2,2,0,0,1,2,2v50a2,2,0,0,1-1.45,1.92l-100.4,28.68A14.06,14.06,0,0,0,122,198v34a6,6,0,0,0,12,0V198a2,2,0,0,1,1.45-1.92l100.4-28.68A14.06,14.06,0,0,0,246,154V104A14,14,0,0,0,232,90Zm-30,38a2,2,0,0,1-2,2H48a2,2,0,0,1-2-2V64a2,2,0,0,1,2-2H200a2,2,0,0,1,2,2Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M232,88H216V64a16,16,0,0,0-16-16H48A16,16,0,0,0,32,64V88H16a8,8,0,0,0,0,16H32v24a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V104h16v50L131.6,182.65A16.07,16.07,0,0,0,120,198v34a8,8,0,0,0,16,0V198l100.4-28.68A16.07,16.07,0,0,0,248,154V104A16,16,0,0,0,232,88Zm-32,40H48V64H200v64Z"/> }.into_view()
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