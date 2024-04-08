use leptos::*;
use axum::extract::Extension;
use db::{queries, Pool};
use leptos_axum::extract;

#[component]
pub fn IndexPage() -> impl IntoView {

    let keys = async {
        let pool: Extension<Pool> = extract().await.unwrap();
        let mut client = pool.get().await.unwrap();
        let transaction = client.transaction().await.unwrap();

        let api_keys = queries::api_keys::api_keys()
            .bind(&transaction, &1)
            .all()
            .await.unwrap();
        api_keys
    };

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
            <div class="card border">
                <div class="card-body">
                    <h2 class="card-title">Card title!</h2>
                    <div class="overflow-x-auto">
                        <table class="table">
                            <thead>
                            <tr>
                                <th></th>
                                <th>Name</th>
                                <th>Job</th>
                                <th>Favorite Color</th>
                            </tr>
                            </thead>

                            <TableBody/>
                            
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TableBody() -> impl IntoView {

    view! {
        <tbody>
        <tr>
            <th>1</th>
            <td>Cy Ganderton</td>
            <td>Quality Control Specialist</td>
            <td>Blue</td>
        </tr>
        <tr>
            <th>2</th>
            <td>Hart Hagerty</td>
            <td>Desktop Support Technician</td>
            <td>Purple</td>
        </tr>
        <tr>
            <th>3</th>
            <td>Brice Swyre</td>
            <td>Tax Accountant</td>
            <td>Red</td>
        </tr>
        </tbody>
    }
}