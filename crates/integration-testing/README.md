## Expose Selenium Ports

Setup the DATABASE_URL

```sh
dburl
```

Expose the ports in k9s

## Run selenium

From the host

```sh
docker run -p 4444:4444 \
    -p 7900:7900 \
    --shm-size="2g" \
    -v /home/ianpurton/Documents/bionic-gpt/crates/integration-testing/files:/workspace \
    selenium/standalone-chrome:latest
```

Make sure the Postgres port is open