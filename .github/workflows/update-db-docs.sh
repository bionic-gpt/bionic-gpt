sed -i '/^## Database Schema: postgres/,$d' crates/db/README.md

.github/workflows/schema-to-markdown.sh >> crates/db/README.md

