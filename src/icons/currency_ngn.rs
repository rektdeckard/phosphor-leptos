/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn CurrencyNgn(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M88,93.63,110.61,120H88Zm80,68.74V136H145.39ZM232,128A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-32,0a8,8,0,0,0-8-8h-8V72a8,8,0,0,0-16,0v48H131.68L86.07,66.79A8,8,0,0,0,72,72v48H64a8,8,0,0,0,0,16h8v48a8,8,0,0,0,16,0V136h36.32l45.61,53.21A8,8,0,0,0,176,192a8,8,0,0,0,8-8V136h8A8,8,0,0,0,200,128Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M192,112v98l-51.51-66H64V46l51.51,66Z" opacity="0.2"/><path d="M216,136H200V120h16a8,8,0,0,0,0-16H200V46a8,8,0,0,0-16,0v58H119.42L70.31,41.08A8,8,0,0,0,56,46v58H40a8,8,0,0,0,0,16H56v16H40a8,8,0,0,0,0,16H56v58a8,8,0,0,0,16,0V152h64.58l49.11,62.92A8,8,0,0,0,192,218a7.8,7.8,0,0,0,2.6-.44A8,8,0,0,0,200,210V152h16a8,8,0,0,0,0-16Zm-32-16v16H144.39L131.9,120ZM72,69.25,99.12,104H72ZM72,136V120h39.61l12.49,16Zm112,50.75L156.88,152H184Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M216,140H196V116h20a4,4,0,0,0,0-8H196V46a4,4,0,0,0-8,0v62H117.46L67.15,43.54A4,4,0,0,0,60,46v62H40a4,4,0,0,0,0,8H60v24H40a4,4,0,0,0,0,8H60v62a4,4,0,0,0,8,0V148h70.54l50.31,64.46A4,4,0,0,0,192,214a3.9,3.9,0,0,0,1.3-.22A4,4,0,0,0,196,210V148h20a4,4,0,0,0,0-8Zm-28-24v24H142.44l-18.73-24ZM68,57.63,107.32,108H68ZM68,140V116h45.56l18.73,24Zm120,58.37L148.68,148H188Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M216,116H204V46a12,12,0,0,0-24,0v70H133.86L73.46,38.62A12,12,0,0,0,52,46v70H40a12,12,0,0,0,0,24H52v70a12,12,0,0,0,24,0V140h46.14l60.4,77.38A12,12,0,0,0,204,210V140h12a12,12,0,0,0,0-24ZM76,116V80.88L103.41,116Zm104,59.12L152.59,140H180Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M216,138H198V118h18a6,6,0,0,0,0-12H198V46a6,6,0,0,0-12,0v60H118.44L68.73,42.31A6,6,0,0,0,58,46v60H40a6,6,0,0,0,0,12H58v20H40a6,6,0,0,0,0,12H58v60a6,6,0,0,0,12,0V150h67.56l49.71,63.69A6,6,0,0,0,198,210V150h18a6,6,0,0,0,0-12Zm-30-20v20H143.42l-15.61-20ZM70,63.44,103.22,106H70ZM70,138V118h42.58l15.61,20Zm116,54.56L152.78,150H186Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M216,136H200V120h16a8,8,0,0,0,0-16H200V46a8,8,0,0,0-16,0v58H119.42L70.31,41.08A8,8,0,0,0,56,46v58H40a8,8,0,0,0,0,16H56v16H40a8,8,0,0,0,0,16H56v58a8,8,0,0,0,16,0V152h64.58l49.11,62.92A8,8,0,0,0,192,218a7.8,7.8,0,0,0,2.6-.44A8,8,0,0,0,200,210V152h16a8,8,0,0,0,0-16Zm-32-16v16H144.39L131.9,120ZM72,69.25,99.12,104H72ZM72,136V120h39.61l12.49,16Zm112,50.75L156.88,152H184Z"/> }.into_view()
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