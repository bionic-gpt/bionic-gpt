use leptos::*;
use leptos_meta::*;

#[island]
pub fn Counter() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        // sets the document title
        <Title text="The cllick3 button page"/>
        <button class="btn btn-primary" on:click=on_click>"Click Me Yo: " {count}</button>
    }
}
