/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Signpost(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M246,117.35l-36,40A8,8,0,0,1,204,160H136v64a8,8,0,0,1-16,0V160H40a16,16,0,0,1-16-16V80A16,16,0,0,1,40,64h80V32a8,8,0,0,1,16,0V64h68A8,8,0,0,1,210,66.65l36,40A8,8,0,0,1,246,117.35Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M240,112l-36,40H40a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H204Z" opacity="0.2"/><path d="M246,106.65l-36-40A8,8,0,0,0,204,64H136V32a8,8,0,0,0-16,0V64H40A16,16,0,0,0,24,80v64a16,16,0,0,0,16,16h80v64a8,8,0,0,0,16,0V160h68a8,8,0,0,0,5.95-2.65l36-40A8,8,0,0,0,246,106.65ZM200.44,144H40V80H200.44l28.8,32Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M243,109.32l-36-40A4,4,0,0,0,204,68H132V32a4,4,0,0,0-8,0V68H40A12,12,0,0,0,28,80v64a12,12,0,0,0,12,12h84v68a4,4,0,0,0,8,0V156h72a4,4,0,0,0,3-1.32l36-40A4,4,0,0,0,243,109.32ZM202.22,148H40a4,4,0,0,1-4-4V80a4,4,0,0,1,4-4H202.22l32.4,36Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M248.92,104l-36-40A12,12,0,0,0,204,60H140V32a12,12,0,0,0-24,0V60H40A20,20,0,0,0,20,80v64a20,20,0,0,0,20,20h76v60a12,12,0,0,0,24,0V164h64a12,12,0,0,0,8.92-4l36-40A12,12,0,0,0,248.92,104Zm-50.26,36H44V84H198.66l25.2,28Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M244.46,108l-36-40A6,6,0,0,0,204,66H134V32a6,6,0,0,0-12,0V66H40A14,14,0,0,0,26,80v64a14,14,0,0,0,14,14h82v66a6,6,0,0,0,12,0V158h70a6,6,0,0,0,4.46-2l36-40A6,6,0,0,0,244.46,108Zm-43.13,38H40a2,2,0,0,1-2-2V80a2,2,0,0,1,2-2H201.33l30.6,34Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M246,106.65l-36-40A8,8,0,0,0,204,64H136V32a8,8,0,0,0-16,0V64H40A16,16,0,0,0,24,80v64a16,16,0,0,0,16,16h80v64a8,8,0,0,0,16,0V160h68a8,8,0,0,0,5.95-2.65l36-40A8,8,0,0,0,246,106.65ZM200.44,144H40V80H200.44l28.8,32Z"/> }.into_view()
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