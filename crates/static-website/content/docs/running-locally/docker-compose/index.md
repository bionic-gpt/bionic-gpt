# Running Locally

We have a very lightweight version of Bionic for running locally for for limited Proofs of concept. If you require features such as user management, document pipelines etc from the enterprise version then install the enterprise version instead.

## Prerequisites

The easiest way to get running with BionicGPT is with our `docker-compose.yml` file. You'll need [Docker](https://docs.docker.com/engine/install/) installed on your machine.

### OSX and Linux

```sh
curl -O https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/321fb19cc6819a9e294eaae254d0689a13f9bdd9/infra-as-code/docker-compose.yml
```

### Windows

```sh
Invoke-WebRequest -Uri https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/321fb19cc6819a9e294eaae254d0689a13f9bdd9/infra-as-code/docker-compose.yml -OutFile docker-compose.yml
```

### And run

```sh
docker-compose up
```

You can then access the front end from `http://localhost:3000`.

## Screenshot

![Alt text](/landing-page/bionic-console.png "Start Screen")