/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Jeep(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M248,103.47A8.17,8.17,0,0,0,239.73,96h-9.26l-9.29-43.35A16.08,16.08,0,0,0,205.53,40H50.47A16.08,16.08,0,0,0,34.82,52.65L25.53,96H16.27A8.17,8.17,0,0,0,8,103.47,8,8,0,0,0,16,112h8v96a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V184h20a4,4,0,0,0,4-4V136.27a8.17,8.17,0,0,1,7.47-8.25,8,8,0,0,1,8.53,8v44a4,4,0,0,0,4,4h8a4,4,0,0,0,4-4V136.27a8.17,8.17,0,0,1,7.47-8.25,8,8,0,0,1,8.53,8v44a4,4,0,0,0,4,4h20v24a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V112h8A8,8,0,0,0,248,103.47ZM68,152a12,12,0,1,1,12-12A12,12,0,0,1,68,152Zm120,0a12,12,0,1,1,12-12A12,12,0,0,1,188,152Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,104H32L42.65,54.32A8,8,0,0,1,50.47,48H205.53a8,8,0,0,1,7.82,6.32Z" opacity="0.2"/><path d="M240,96h-9.53l-9.29-43.35A16.08,16.08,0,0,0,205.53,40H50.47A16.08,16.08,0,0,0,34.82,52.65L25.53,96H16a8,8,0,0,0,0,16h8v96a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V184h96v24a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V112h8a8,8,0,0,0,0-16ZM50.47,56H205.53l8.57,40H41.9ZM64,208H40V184H64Zm128,0V184h24v24Zm24-40H152V136a8,8,0,0,0-16,0v32H120V136a8,8,0,0,0-16,0v32H40V112H216ZM56,140a12,12,0,1,1,12,12A12,12,0,0,1,56,140Zm120,0a12,12,0,1,1,12,12A12,12,0,0,1,176,140Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M240,100H227.23l-10-46.51A12.07,12.07,0,0,0,205.53,44H50.47a12.07,12.07,0,0,0-11.74,9.49L28.77,100H16a4,4,0,0,0,0,8H28V208a12,12,0,0,0,12,12H64a12,12,0,0,0,12-12V180H180v28a12,12,0,0,0,12,12h24a12,12,0,0,0,12-12V108h12a4,4,0,0,0,0-8ZM46.56,55.16A4,4,0,0,1,50.47,52H205.53a4,4,0,0,1,3.91,3.16L219.05,100H37ZM68,208a4,4,0,0,1-4,4H40a4,4,0,0,1-4-4V180H68Zm148,4H192a4,4,0,0,1-4-4V180h32v28A4,4,0,0,1,216,212Zm4-40H148V136a4,4,0,0,0-8,0v36H116V136a4,4,0,0,0-8,0v36H36V108H220ZM60,140a8,8,0,1,1,8,8A8,8,0,0,1,60,140Zm120,0a8,8,0,1,1,8,8A8,8,0,0,1,180,140Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M240,92h-6.3l-8.61-40.19A20.11,20.11,0,0,0,205.53,36H50.47A20.11,20.11,0,0,0,30.91,51.81L22.3,92H16a12,12,0,0,0,0,24h4v92a20,20,0,0,0,20,20H64a20,20,0,0,0,20-20V188h88v20a20,20,0,0,0,20,20h24a20,20,0,0,0,20-20V116h4a12,12,0,0,0,0-24ZM53.7,60H202.3l6.86,32H46.84ZM60,204H44V188H60Zm136,0V188h16v16Zm16-40H180V140a12,12,0,0,0-24,0v24H140V140a12,12,0,0,0-24,0v24H100V140a12,12,0,0,0-24,0v24H44V116H212Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M240,98H228.85l-9.63-44.93A14.06,14.06,0,0,0,205.53,42H50.47A14.06,14.06,0,0,0,36.78,53.07L27.15,98H16a6,6,0,0,0,0,12H26v98a14,14,0,0,0,14,14H64a14,14,0,0,0,14-14V182H178v26a14,14,0,0,0,14,14h24a14,14,0,0,0,14-14V110h10a6,6,0,0,0,0-12ZM48.51,55.58a2,2,0,0,1,2-1.58H205.53a2,2,0,0,1,2,1.58L216.58,98H39.42ZM66,208a2,2,0,0,1-2,2H40a2,2,0,0,1-2-2V182H66Zm150,2H192a2,2,0,0,1-2-2V182h28v26A2,2,0,0,1,216,210Zm2-40H150V136a6,6,0,0,0-12,0v34H118V136a6,6,0,0,0-12,0v34H38V110H218ZM58,140a10,10,0,1,1,10,10A10,10,0,0,1,58,140Zm120,0a10,10,0,1,1,10,10A10,10,0,0,1,178,140Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M240,96h-9.53l-9.29-43.35A16.08,16.08,0,0,0,205.53,40H50.47A16.08,16.08,0,0,0,34.82,52.65L25.53,96H16a8,8,0,0,0,0,16h8v96a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V184h96v24a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V112h8a8,8,0,0,0,0-16ZM50.47,56H205.53l8.57,40H41.9ZM64,208H40V184H64Zm128,0V184h24v24Zm24-40H152V136a8,8,0,0,0-16,0v32H120V136a8,8,0,0,0-16,0v32H40V112H216ZM56,140a12,12,0,1,1,12,12A12,12,0,0,1,56,140Zm120,0a12,12,0,1,1,12,12A12,12,0,0,1,176,140Z"/> }.into_view()
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