use leptos::{component, view, IntoView};

#[allow(unused_braces)]
#[component]
pub fn Cells(#[prop(default = 250)] cells: usize) -> impl IntoView {
    let cells = (0..cells)
        .map(|_| view! { <span></span> })
        .collect::<Vec<_>>();
    view! { {cells} }
}
