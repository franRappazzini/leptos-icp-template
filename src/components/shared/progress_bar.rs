use leptos::{component, view, IntoView, ReadSignal};

#[component]
pub fn ProgressBar(
    progress: ReadSignal<i32>,
    #[prop(optional, default = 50)] max: i32,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // now this works
            value=progress
        />
    }
}
