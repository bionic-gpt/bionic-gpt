# Rules and Guidelines

This is a [Rust on Nails](https://rust-on-nails.com/) project using Rust to build a full stack web application.
All of the pages are generated server side with a little bit of Typescript on the front end to add interactivity when needed.

## Tech Stack

- Axum              # Handles all the applications routes and actions (https://github.com/tokio-rs/axum)
- Cornucopia        # Generates rust functions from `.sql` files. (https://cornucopia-rs.netlify.app/)
- Dioxus rsx! macro # Used to create UI components and pages on the server side. (https://dioxuslabs.com/)
- Daisy UI          # Tailwind components (https://daisyui.com/)
- daisy_rsx         # A rust crate that implements the Daisy UI components in rsx!
- DbMate            # Database Migrations (https://github.com/amacneil/dbmate)
- Postgres          # Database
- Earthly           # Build system for production. (https://earthly.dev/)

## Folder: db

- All of the `dbmate` migrations are stored in the `migrations` folder.
- To create a new migration run `dbmate new migration-name` where migration name somehow represents the work you are doing.
- All of the `.sql` files are in a folder called `queries`.
- The `sql` files are named after the main tables use. i.e. `users.sql` for the `users` table.
- All the database CRUD operation are in these files.
- When you update the file a code generator runs and creates rust code from the sql. (cornucopia).
- We export these functions ans structs in `crates/db/lib.rs`

### Cornucopia SQL Guidelines

- **Struct Definitions**: Add `--: StructName()` before queries to define return types
- **Query Naming**: Use `--! query_name : StructName` to name queries and specify return types
- **Parameters**: Cornucopia auto-detects parameters from `:param_name` syntax - don't declare them manually
- **Intervals**: Use `(:days || ' days')::INTERVAL` for dynamic intervals, not `INTERVAL ':days days'`
- **Optional Fields**: Add `field_name?` in struct definitions for nullable columns

## Folder: static-website

The marketing pages for the application. They are generated by a rust program which uses RSX! and markdown to generate HTML pages.

- `src/blog_summary.rs` takes the markdown files in `content/blog` to create a blog based on the `src/layouts/blog.rs` layout.
- `src/docs_summary.rs` takes the markdown files in `content/docs` to create the documentation based on the `src/layouts/docs.rs` layout.
- `src/pages_summary.rs` takes the markdown files in `content/pages` to create the documentation based on the `src/layouts/pages.rs` layout.

## Folder: web-assets

- Any images that are needed by the application are stored in a sub folder called images
- Also the tailwind config is stored here.
- The user will run `just tailwind` this will watch the tailwind `input.css` and src files for any changes. 
- When changes occur the resulting `tailwind.css` is stored in a `dist` folder.
- There is a `build.rs` it uses a crate called `cache-busters` that sees the images and css files. 
- It takes the hash of the files and crates a struct that gives us the ability to access the images by name in a typesafe way.
- For example the `tailwind.css` will be exported as `web_assets::files::tailwind_css` in the app and we reference it by calling `web_assets::files::tailwind.name`.

## Folder: web-pages

- Every route in the application has a corresponding page. 
- For example if we have a route `/api_keys` then we will have an `api_keys.rs`.
- This will have a function that `index` that takes the parameteres needed to populate the page.
- The `index` function uses rsx! to generate the page using.
- If the page gets large we can break it up into components.
- If the file gets large we may break the file into a module.
- So for example `api_keys.rs` would have a corresponding folder called `api_keys` with components inside it.

## Folder: web-server

- Every route in the application has a corresponding rust file.
- For example if we have a route `/api_keys` then we will have an `api_keys.rs`.
- Every route has a function called `loader`.
- The loader fn calls the database to get the data and passes to to the corresponding page component.
- The populated page is then returned as a Html<String>.
- Any action for the route i.e. create, delete, update are also in this file. 
- They call the corresponing database functions before redirecting the browser.

## Setting up for Development

Bionic runs in a `devcontainer` and uses [k3d](https://k3d.io/stable/) to run supporting backend services i.e. Postgres.

1. Run `just dev-init` to setup `k3d`
1. Run `just dev-setup` to run the kubernetes operator that install Bionic into the locally running `k3d`.
1. If you get a Permission denied (os error 13) run `sudo chmod -R 777 target`
1. If you get a *service unavailable* error wait a bit longer for *k3d* to start.
1. Use `k9s` to check the status of the services.
1. When all the services are loaded you can check by running `db` you should now have access to the database.
1. `dbmate up` to create the database tables
1. Run `wp` to run and watch the Typescript pipeline.
1. Run `wt` to run and watch the Tailwind pipeline.
1. Run `wa` to build and watch the Bionic server.
1. You can now access the front end on `http://localhost:7703`.

## Running the integration tests

Selenium is configured as part of our `devcontainer` and is accessible via `http://localhost:7706/`

1. Run `just integration-testing` and watch the tests run via the VNC connection in `http://localhost:7706/`