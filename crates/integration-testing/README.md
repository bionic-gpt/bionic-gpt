## Integration Testing in the Devcontainer

From within the dev container

Run the application and the rag engine

```
cargo run --bin rag-engine
```

```
export WEB_DRIVER_URL=http://selenium:4444
export APPLICATION_URL=http://envoy:7700
export MAILHOG_URL=http://smtp:8025
export ENABLE_BARRICADE=barricade
cargo test
```

## Accessing Selenium

Point VNC at `http://localhost:7705`