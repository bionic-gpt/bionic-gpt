use leptos::*;
use leptos_meta::*;

#[component]
pub fn IndexPage() -> impl IntoView {
    view! {
        <div class="navbar bg-base-100">
            <div class="flex-none">
            <button class="btn btn-square btn-ghost">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
            </button>
            </div>
            <div class="flex-1">
                <a class="btn btn-ghost text-xl">API Keys</a>
            </div>
            <div class="flex-none">
            <button class="btn btn-primary">
                New API Key
            </button>
            </div>
        </div>
        <div class="m-5 mb-0">
            <h1>"Welcome to Leptos!"</h1>
            <Counter/>
        </div>
        <Drawer/>
    }
}

#[component]
fn Drawer() -> impl IntoView {
    view! {
        <div class="drawer drawer-end">
            <input id="my-drawer-4" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content">
                <label for="my-drawer-4" class="drawer-button btn btn-primary">Open drawer</label>
            </div> 
            <div class="drawer-side">
                <label for="my-drawer-4" aria-label="close sidebar" class="drawer-overlay"></label>
                <ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
                <li><a>Sidebar Item 1</a></li>
                <li><a>Sidebar Item 2</a></li>
                </ul>
            </div>
        </div>
    }
}

#[island]
fn Counter() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        // sets the document title
        <Title text="The cllick button page"/>
        <button class="btn btn-primary" on:click=on_click>"Click Me: " {count}</button>
    }
}