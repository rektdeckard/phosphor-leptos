/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn CurrencyJpy(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm62,53.27L145.63,128H160a8,8,0,0,1,0,16H136v16h24a8,8,0,0,1,0,16H136v16a8,8,0,0,1-16,0V176H96a8,8,0,0,1,0-16h24V144H96a8,8,0,0,1,0-16h14.37L66,77.27A8,8,0,0,1,78,66.73l50,57.12,50-57.12a8,8,0,1,1,12,10.54Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M200,48l-72,88L56,48Z" opacity="0.2"/><path d="M206.19,53.07,144.88,128H176a8,8,0,0,1,0,16H136v16h40a8,8,0,0,1,0,16H136v40a8,8,0,0,1-16,0V176H80a8,8,0,0,1,0-16h40V144H80a8,8,0,0,1,0-16h31.12L49.81,53.07A8,8,0,0,1,62.19,42.93L128,123.37l65.81-80.44a8,8,0,1,1,12.38,10.14Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M203.1,50.53,136.44,132H176a4,4,0,0,1,0,8H132v24h44a4,4,0,0,1,0,8H132v44a4,4,0,0,1-8,0V172H80a4,4,0,0,1,0-8h44V140H80a4,4,0,0,1,0-8h39.56L52.9,50.53a4,4,0,0,1,6.2-5.06L128,129.68l68.9-84.21a4,4,0,1,1,6.2,5.06Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M209.29,55.6l-56,68.4H176a12,12,0,0,1,0,24H140v16h36a12,12,0,0,1,0,24H140v28a12,12,0,0,1-24,0V188H80a12,12,0,0,1,0-24h36V148H80a12,12,0,0,1,0-24h22.68l-56-68.4A12,12,0,1,1,65.29,40.4L128,117.05,190.71,40.4a12,12,0,1,1,18.58,15.2Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M204.64,51.8l-64,78.2H176a6,6,0,0,1,0,12H134v20h42a6,6,0,0,1,0,12H134v42a6,6,0,0,1-12,0V174H80a6,6,0,0,1,0-12h42V142H80a6,6,0,0,1,0-12h35.34l-64-78.2a6,6,0,1,1,9.28-7.6L128,126.53,195.36,44.2a6,6,0,0,1,9.28,7.6Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M206.19,53.07,144.88,128H176a8,8,0,0,1,0,16H136v16h40a8,8,0,0,1,0,16H136v40a8,8,0,0,1-16,0V176H80a8,8,0,0,1,0-16h40V144H80a8,8,0,0,1,0-16h31.12L49.81,53.07A8,8,0,0,1,62.19,42.93L128,123.37l65.81-80.44a8,8,0,1,1,12.38,10.14Z"/> }.into_view()
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