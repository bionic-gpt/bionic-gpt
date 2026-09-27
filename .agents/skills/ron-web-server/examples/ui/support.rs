use dioxus::prelude::*;

// Minimal page models: no database or authentication crate dependency.
#[derive(Clone, PartialEq)]
pub struct ItemView {
    pub id: i64,
    pub name: String,
    pub can_edit: bool,
}
#[derive(Clone, Default, PartialEq)]
pub struct ItemForm {
    pub id: Option<i64>,
    pub name: String,
    pub error: Option<String>,
}
pub mod routes {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;
    #[derive(TypedPath, Deserialize)]
    #[typed_path("/teams/{team_id}/items")]
    pub struct Index {
        pub team_id: i64,
    }
    #[derive(TypedPath, Deserialize)]
    #[typed_path("/teams/{team_id}/items/new")]
    pub struct New {
        pub team_id: i64,
    }
    #[derive(TypedPath, Deserialize)]
    #[typed_path("/teams/{team_id}/items/create")]
    pub struct Create {
        pub team_id: i64,
    }
    #[derive(TypedPath, Deserialize)]
    #[typed_path("/teams/{team_id}/items/{id}/edit")]
    pub struct Edit {
        pub team_id: i64,
        pub id: i64,
    }
    #[derive(TypedPath, Deserialize)]
    #[typed_path("/teams/{team_id}/items/{id}/update")]
    pub struct Update {
        pub team_id: i64,
        pub id: i64,
    }
    #[derive(TypedPath, Deserialize)]
    #[typed_path("/teams/{team_id}/items/{id}/delete")]
    pub struct Delete {
        pub team_id: i64,
        pub id: i64,
    }
}

pub fn render(element: Element) -> String {
    format!(
        "<!DOCTYPE html><html lang=\"en\">{}</html>",
        dioxus_ssr::render_element(element)
    )
}

#[component]
pub fn Layout(title: String, team_id: i64, children: Element) -> Element {
    rsx! {
            head {
                meta { charset: "utf-8" }
                meta { name: "viewport", content: "width=device-width, initial-scale=1" }
                title { "{title}" }
                // Build/serve this stylesheet in the destination's asset pipeline.
                link { rel: "stylesheet", href: "/assets/app.css" }
                script { src: "/assets/dialogs.js", defer: true }
            }
            body { class: "min-h-screen bg-base-200 text-base-content",
                header { class: "navbar bg-base-100 px-4",
                    nav { "aria-label": "Main navigation",
                        a { class: "btn btn-ghost", href: routes::Index { team_id }.to_string(), "Items" }
                    }
                }
                main { class: "max-w-3xl mx-auto p-4",
                    nav { class: "breadcrumbs text-sm", "aria-label": "Breadcrumb",
                        ul {
                            li { a { href: routes::Index { team_id }.to_string(), "Items" } }
                            li { span { "aria-current": "page", "{title}" } }
                        }
                    }
                    h1 { class: "text-2xl font-bold mb-4", "{title}" }
                    {children}
                }
            }
    }
}

#[component]
pub fn Introduction(is_empty: bool) -> Element {
    rsx! {
        p { class: "mb-4",
            if is_empty { "No items yet. Add your first item." }
            else { "View and manage your team's items." }
        }
    }
}

#[component]
pub fn ItemCard(team_id: i64, item: ItemView) -> Element {
    rsx! {
        article { class: "card bg-base-100 shadow-sm mb-3",
            div { class: "card-body",
                h2 { class: "card-title", "{item.name}" }
                if item.can_edit {
                    a { class: "btn btn-sm", href: routes::Edit { team_id, id: item.id }.to_string(), "Edit" }
                }
            }
        }
    }
}

#[component]
pub fn ItemFields(team_id: i64, form: ItemForm) -> Element {
    let action = match form.id {
        Some(id) => routes::Update { team_id, id }.to_string(),
        None => routes::Create { team_id }.to_string(),
    };
    rsx! {
        form { method: "post", action, class: "flex flex-col gap-4",
            if let Some(error) = &form.error {
                div { class: "alert alert-error", role: "alert", "{error}" }
            }
            label { r#for: "item-name", "Name" }
            input { id: "item-name", class: "input input-bordered w-full", name: "name",
                value: form.name, required: true, maxlength: 100 }
            div { class: "flex gap-2",
                button { class: "btn btn-primary", r#type: "submit", "Save" }
                a { class: "btn", href: routes::Index { team_id }.to_string(), "Cancel" }
            }
        }
    }
}

#[component]
pub fn DeleteConfirmation(team_id: i64, id: i64) -> Element {
    let dialog_id = format!("delete-item-{id}");
    let heading_id = format!("delete-heading-{id}");
    rsx! {
        button { r#type: "button", class: "btn btn-error mt-4", "data-open-dialog": dialog_id.clone(), "Delete" }
        dialog { id: dialog_id, class: "modal", "aria-labelledby": heading_id.clone(),
            div { class: "modal-box",
                h2 { id: heading_id, class: "font-bold text-lg", "Delete item?" }
                p { "This action cannot be undone." }
                form { method: "post", action: routes::Delete { team_id, id }.to_string(), class: "modal-action",
                    button { r#type: "button", class: "btn", "data-close-dialog": "", "Cancel" }
                    button { r#type: "submit", class: "btn btn-error", "Delete item" }
                }
            }
        }
    }
}
