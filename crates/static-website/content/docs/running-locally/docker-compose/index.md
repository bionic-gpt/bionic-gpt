# Try Bionic with Docker Compose

This version of Bionic has all the functionality of Bionic but is not recommended for production use cases. Only this this method of install for Proofs of Concept.

## Prerequisites

The easiest way to get running with Bionic is with our `docker-compose.yml` file. You'll need [Docker](https://docs.docker.com/engine/install/) installed on your machine.

### OSX and Linux

```sh
curl -O https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/bb67c2d9bf5ca8d54049b6ee910e7a381185aa9f/infra-as-code/docker-compose.yml
```

### Windows

```sh
Invoke-WebRequest -Uri https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/bb67c2d9bf5ca8d54049b6ee910e7a381185aa9f/infra-as-code/docker-compose.yml -OutFile docker-compose.yml
```

### And run

```sh
docker-compose up
```

You can then access the front end from `http://localhost:3000`.

## Screenshot

![Alt text](/landing-page/bionic-console.png "Start Screen")