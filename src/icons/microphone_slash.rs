/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn MicrophoneSlash(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M213.38,221.92a8,8,0,0,1-11.3-.54l-26.46-29.1A79.74,79.74,0,0,1,136,207.59V232a8,8,0,0,1-16,0V207.6A80.11,80.11,0,0,1,48,128a8,8,0,0,1,16,0,64,64,0,0,0,100.79,52.36l-10.88-12A48,48,0,0,1,80,128V87.09L42.08,45.38A8,8,0,1,1,53.92,34.62l160,176A8,8,0,0,1,213.38,221.92Zm-51.3-92.11A8,8,0,0,0,176,124.43V64A48,48,0,0,0,87.16,38.78,8,8,0,0,0,88,48.37Zm30.1,31.83a8,8,0,0,0,10.36-4.55A79.62,79.62,0,0,0,208,128a8,8,0,0,0-16,0,63.71,63.71,0,0,1-4.36,23.27A8,8,0,0,0,192.18,161.64Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M168,64v64a40,40,0,0,1-40,40h0a40,40,0,0,1-40-40V64a40,40,0,0,1,40-40h0A40,40,0,0,1,168,64Z" opacity="0.2"/><path d="M213.92,210.62l-160-176A8,8,0,1,0,42.08,45.38L80,87.09V128a48,48,0,0,0,73.91,40.4l10.88,12A64,64,0,0,1,64,128a8,8,0,0,0-16,0,80.11,80.11,0,0,0,72,79.6V232a8,8,0,0,0,16,0V207.59a79.74,79.74,0,0,0,39.62-15.31l26.46,29.1a8,8,0,1,0,11.84-10.76ZM128,160a32,32,0,0,1-32-32V104.69l46.92,51.62A32,32,0,0,1,128,160ZM87.16,38.78A48,48,0,0,1,176,64v60.43a8,8,0,0,1-16,0V64a32,32,0,0,0-59.24-16.81,8,8,0,1,1-13.6-8.41ZM187.64,151.27A63.71,63.71,0,0,0,192,128a8,8,0,0,1,16,0,79.62,79.62,0,0,1-5.46,29.09,8,8,0,1,1-14.9-5.82Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M211,213.31,51,37.31A4,4,0,0,0,45,42.69L84,85.55V128a44,44,0,0,0,70.52,35.11l16.17,17.8A68,68,0,0,1,60,128a4,4,0,0,0-8,0,76.09,76.09,0,0,0,72,75.89V232a4,4,0,0,0,8,0V203.89a75.76,75.76,0,0,0,44.09-17L205,218.69a4,4,0,1,0,5.92-5.38ZM128,164a36,36,0,0,1-36-36V94.35l57.1,62.81A35.9,35.9,0,0,1,128,164ZM90.56,40.88A44,44,0,0,1,172,64v60.43a4,4,0,0,1-8,0V64A36,36,0,0,0,97.36,45.09a4,4,0,1,1-6.8-4.21ZM191.37,152.73A67.81,67.81,0,0,0,196,128a4,4,0,0,1,8,0,75.64,75.64,0,0,1-5.18,27.64,4,4,0,0,1-3.73,2.54,3.88,3.88,0,0,1-1.45-.27A4,4,0,0,1,191.37,152.73Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M56.88,31.93A12,12,0,1,0,39.12,48.07L76,88.64V120a52,52,0,0,0,72.11,48l11.26,12.39A67.34,67.34,0,0,1,128,188a68.07,68.07,0,0,1-68-68,12,12,0,0,0-24,0,92.14,92.14,0,0,0,80,91.22V232a12,12,0,0,0,24,0V211.23a90.39,90.39,0,0,0,35.92-12.68l23.2,25.52a12,12,0,0,0,17.76-16.14ZM128,148a28,28,0,0,1-28-28v-5l29.9,32.89C129.27,148,128.64,148,128,148ZM90.67,45.27a12,12,0,0,1-.48-17A52,52,0,0,1,180,64v48.54a12,12,0,1,1-24,0V64a28,28,0,0,0-48.36-19.22A12,12,0,0,1,90.67,45.27ZM191,145.58A67.63,67.63,0,0,0,196,120a12,12,0,0,1,24,0,91.48,91.48,0,0,1-6.74,34.61,12,12,0,0,1-22.23-9Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M212.44,212,52.44,36A6,6,0,0,0,43.56,44L82,86.32V128a46,46,0,0,0,72.24,37.78l13.53,14.89A66,66,0,0,1,62,128a6,6,0,0,0-12,0,78.09,78.09,0,0,0,72,77.75V232a6,6,0,0,0,12,0V205.75a77.68,77.68,0,0,0,41.86-16.18L203.56,220a6,6,0,0,0,8.88-8.08ZM128,162a34,34,0,0,1-34-34V99.52l52.07,57.28A33.92,33.92,0,0,1,128,162ZM88.86,39.83A46,46,0,0,1,174,64v60.43a6,6,0,0,1-12,0V64A34,34,0,0,0,99.06,46.14a6,6,0,1,1-10.2-6.31ZM189.5,152a65.55,65.55,0,0,0,4.5-24,6,6,0,0,1,12,0,77.65,77.65,0,0,1-5.32,28.37,6,6,0,0,1-5.59,3.82A6,6,0,0,1,189.5,152Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M213.92,210.62l-160-176A8,8,0,1,0,42.08,45.38L80,87.09V128a48,48,0,0,0,73.91,40.4l10.88,12A64,64,0,0,1,64,128a8,8,0,0,0-16,0,80.11,80.11,0,0,0,72,79.6V232a8,8,0,0,0,16,0V207.59a79.74,79.74,0,0,0,39.62-15.31l26.46,29.1a8,8,0,1,0,11.84-10.76ZM128,160a32,32,0,0,1-32-32V104.69l46.92,51.62A32,32,0,0,1,128,160ZM87.16,38.78A48,48,0,0,1,176,64v60.43a8,8,0,0,1-16,0V64a32,32,0,0,0-59.24-16.81,8,8,0,1,1-13.6-8.41ZM187.64,151.27A63.71,63.71,0,0,0,192,128a8,8,0,0,1,16,0,79.62,79.62,0,0,1-5.46,29.09,8,8,0,1,1-14.9-5.82Z"/> }.into_view()
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