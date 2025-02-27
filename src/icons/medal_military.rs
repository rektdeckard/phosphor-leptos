/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn MedalMilitary(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M207,32H49A17,17,0,0,0,32,49V98.21a17,17,0,0,0,10,15.47l62.6,28.45a48,48,0,1,0,46.88,0L214,113.68a17,17,0,0,0,10-15.47V49A17,17,0,0,0,207,32ZM96,48h64v72.67l-32,14.54L96,120.67Zm32,168a32,32,0,1,1,32-32A32,32,0,0,1,128,216Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M168,184a40,40,0,1,1-40-40A40,40,0,0,1,168,184ZM207,40H168v85.82l42.72-19.42A9,9,0,0,0,216,98.2V49A9,9,0,0,0,207,40ZM88,40H49a9,9,0,0,0-9,9V98.2a9,9,0,0,0,5.28,8.2L88,125.82Z" opacity="0.2"/><path d="M207,32H49A17,17,0,0,0,32,49V98.21a17,17,0,0,0,10,15.47l62.6,28.45a48,48,0,1,0,46.88,0L214,113.68a17,17,0,0,0,10-15.47V49A17,17,0,0,0,207,32ZM160,48v72.67l-32,14.54L96,120.67V48ZM48,98.21V49a1,1,0,0,1,1-1H80v65.39L48.59,99.12A1,1,0,0,1,48,98.21ZM128,216a32,32,0,1,1,32-32A32,32,0,0,1,128,216ZM208,98.21a1,1,0,0,1-.59.91L176,113.39V48h31a1,1,0,0,1,1,1Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M207,36H49A13,13,0,0,0,36,49V98.21A13,13,0,0,0,43.62,110l70.72,32.14a44,44,0,1,0,27.32,0L212.38,110A13,13,0,0,0,220,98.21V49A13,13,0,0,0,207,36Zm-43,8v79.24l-36,16.37L92,123.24V44ZM44,98.21V49a5,5,0,0,1,5-5H84v75.61L46.93,102.76A5,5,0,0,1,44,98.21ZM164,184a36,36,0,1,1-36-36A36,36,0,0,1,164,184Zm48-85.79a5,5,0,0,1-2.93,4.55L172,119.61V44h35a5,5,0,0,1,5,5Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M207,28H49A21,21,0,0,0,28,49V98.21a21,21,0,0,0,12.31,19.11l56,25.47a52,52,0,1,0,63.32,0l56-25.47A21,21,0,0,0,228,98.21V49A21,21,0,0,0,207,28ZM128,130.82l-28-12.73V52h56v66.09ZM52,52H76v55.18L52,96.27Zm76,160a28,28,0,1,1,28-28A28,28,0,0,1,128,212ZM204,96.27l-24,10.91V52h24Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M207,34H49A15,15,0,0,0,34,49V98.21a15,15,0,0,0,8.79,13.65L109.19,142a46,46,0,1,0,37.62,0l66.4-30.18A15,15,0,0,0,222,98.21V49A15,15,0,0,0,207,34ZM162,46v76l-34,15.45L94,122V46ZM46,98.21V49a3,3,0,0,1,3-3H82v70.5L47.76,100.94A3,3,0,0,1,46,98.21ZM162,184a34,34,0,1,1-34-34A34,34,0,0,1,162,184Zm48-85.79a3,3,0,0,1-1.76,2.73L174,116.5V46h33a3,3,0,0,1,3,3Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M207,32H49A17,17,0,0,0,32,49V98.21a17,17,0,0,0,10,15.47l62.6,28.45a48,48,0,1,0,46.88,0L214,113.68a17,17,0,0,0,10-15.47V49A17,17,0,0,0,207,32ZM160,48v72.67l-32,14.54L96,120.67V48ZM48,98.21V49a1,1,0,0,1,1-1H80v65.39L48.59,99.12A1,1,0,0,1,48,98.21ZM128,216a32,32,0,1,1,32-32A32,32,0,0,1,128,216ZM208,98.21a1,1,0,0,1-.59.91L176,113.39V48h31a1,1,0,0,1,1,1Z"/> }.into_view()
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