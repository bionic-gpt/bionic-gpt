## Testing RAG

Have K3s running locally so we can expose the chunking engine.

## Make sure Ollama is set up

```sh
ollama pull nomic-embed-text
ollama pull granite4:3b
```

## Test Embeddings

```sh
curl -X POST http://localhost:11434/v1/embeddings \
  -H "Content-Type: application/json" \
  -d '{
    "model": "nomic-embed-text",
    "input": "The quick brown fox jumps over the lazy dog."
  }'
```

## Remove the Existing RAG Engine

```sh
sed '/^    rag-engine:/,$d' infra-as-code/stack.yaml > /tmp/stack.yaml
stack deploy --manifest /tmp/stack.yaml --profile dev
```

```
kubectl delete deploy rag-engine -n bionic-gpt
kubectl delete svc rag-engine -n bionic-gpt
```

## Open a Port to the Doc Engine

```sh
just expose-doc-engine
```

## Run the Job

```sh
KREUZBERG_API_ENDPOINT=http://localhost:8000 cargo run --bin rag-engine
```