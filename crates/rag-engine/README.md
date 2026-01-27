## Testing RAG

Have K3s running locally so we can expose the chunking engine.

## Remove the Existing RAG Engine


```sh
sed '/^    rag-engine:/,$d' infra-as-code/stack.yaml > /tmp/stack.yaml
stack deploy --manifest /tmp/stack.yaml --profile dev
```

```
kubectl delete deploy rag-engine -n bionic-gpt
kubectl delete svc rag-engine -n bionic-gpt
```


## Run the Job

```sh
cargo run --bin rag-engine
```