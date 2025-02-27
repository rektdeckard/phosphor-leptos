/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Rug(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M200,16a8,8,0,0,0-8,8V40H160V24a8,8,0,0,0-16,0V40H112V24a8,8,0,0,0-16,0V40H64V24a8,8,0,0,0-16,0V232a8,8,0,0,0,16,0V216H96v16a8,8,0,0,0,16,0V216h32v16a8,8,0,0,0,16,0V216h32v16a8,8,0,0,0,16,0V24A8,8,0,0,0,200,16ZM64,56H192V200H64Zm57.14,116.12-24-40a8,8,0,0,1,0-8.24l24-40a8,8,0,0,1,13.72,0l24,40a8,8,0,0,1,0,8.24l-24,40a8,8,0,0,1-13.72,0Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M56,48V208H200V48Zm72,120-24-40,24-40,24,40Z" opacity="0.2"/><path d="M200,16a8,8,0,0,0-8,8V40H160V24a8,8,0,0,0-16,0V40H112V24a8,8,0,0,0-16,0V40H64V24a8,8,0,0,0-16,0V232a8,8,0,0,0,16,0V216H96v16a8,8,0,0,0,16,0V216h32v16a8,8,0,0,0,16,0V216h32v16a8,8,0,0,0,16,0V24A8,8,0,0,0,200,16Zm-8,184H64V56H192Zm-70.86-27.88a8,8,0,0,0,13.72,0l24-40a8,8,0,0,0,0-8.24l-24-40a8,8,0,0,0-13.72,0l-24,40a8,8,0,0,0,0,8.24ZM128,103.55,142.67,128,128,152.45,113.33,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M200,20a4,4,0,0,0-4,4V44H156V24a4,4,0,0,0-8,0V44H108V24a4,4,0,0,0-8,0V44H60V24a4,4,0,0,0-8,0V232a4,4,0,0,0,8,0V212h40v20a4,4,0,0,0,8,0V212h40v20a4,4,0,0,0,8,0V212h40v20a4,4,0,0,0,8,0V24A4,4,0,0,0,200,20ZM60,52H196V204H60Zm68,120a4,4,0,0,0,3.43-1.94l24-40a4,4,0,0,0,0-4.12l-24-40a4,4,0,0,0-6.86,0l-24,40a4,4,0,0,0,0,4.12l24,40A4,4,0,0,0,128,172Zm0-76.23L147.33,128,128,160.23,108.67,128Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M200,12a12,12,0,0,0-12,12V36H164V24a12,12,0,0,0-24,0V36H116V24a12,12,0,0,0-24,0V36H68V24a12,12,0,0,0-24,0V232a12,12,0,0,0,24,0V220H92v12a12,12,0,0,0,24,0V220h24v12a12,12,0,0,0,24,0V220h24v12a12,12,0,0,0,24,0V24A12,12,0,0,0,200,12ZM68,60H188V196H68Zm60,120a12,12,0,0,0,10.29-5.83l24-40a12,12,0,0,0,0-12.34l-24-40a12,12,0,0,0-20.58,0l-24,40a12,12,0,0,0,0,12.34l24,40A12,12,0,0,0,128,180Zm0-68.68L138,128l-10,16.68L118,128Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M200,18a6,6,0,0,0-6,6V42H158V24a6,6,0,0,0-12,0V42H110V24a6,6,0,0,0-12,0V42H62V24a6,6,0,0,0-12,0V232a6,6,0,0,0,12,0V214H98v18a6,6,0,0,0,12,0V214h36v18a6,6,0,0,0,12,0V214h36v18a6,6,0,0,0,12,0V24A6,6,0,0,0,200,18ZM62,54H194V202H62Zm66,120a6,6,0,0,0,5.14-2.91l24-40a6,6,0,0,0,0-6.18l-24-40a6,6,0,0,0-10.28,0l-24,40a6,6,0,0,0,0,6.18l24,40A6,6,0,0,0,128,174Zm0-74.34L145,128l-17,28.34L111,128Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M200,16a8,8,0,0,0-8,8V40H160V24a8,8,0,0,0-16,0V40H112V24a8,8,0,0,0-16,0V40H64V24a8,8,0,0,0-16,0V232a8,8,0,0,0,16,0V216H96v16a8,8,0,0,0,16,0V216h32v16a8,8,0,0,0,16,0V216h32v16a8,8,0,0,0,16,0V24A8,8,0,0,0,200,16ZM64,56H192V200H64Zm64,120a8,8,0,0,0,6.86-3.88l24-40a8,8,0,0,0,0-8.24l-24-40a8,8,0,0,0-13.72,0l-24,40a8,8,0,0,0,0,8.24l24,40A8,8,0,0,0,128,176Zm0-72.45L142.67,128,128,152.45,113.33,128Z"/> }.into_view()
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