# web-pages crate

This crate contains all server side UI built with Dioxus.  Each route has its own
folder and the main page of the route is implemented in `page.rs`.

```
crates/web-pages
├── api_keys/
│   ├── page.rs       # `/api_keys` page
│   ├── ...           # components
│   └── mod.rs        # `pub mod page;` and component exports
├── integrations/
│   ├── page.rs
│   └── ...
├── components/
│   ├── confirm_modal.rs
│   └── logout_form.rs
└── routes.rs         # typed path definitions
```

`routes.rs` defines typed paths which are used in `web-server/handlers`.  Each
handler module under `crates/web-server/handlers` loads data and calls the
corresponding `page` function from these folders to render HTML.

