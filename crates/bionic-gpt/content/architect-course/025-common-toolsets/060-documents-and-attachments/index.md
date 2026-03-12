# Documents and Attachments

```js
# Search indexed documents
document_search(query: string, top_k: number): object[]

# List files attached to a thread or run
attachment_list(scope_id: string): object[]

# Read attachment content or extracted text
attachment_read(attachment_id: string): string

# Add a new attachment
attachment_add(scope_id: string, filename: string, content: string): string
```
