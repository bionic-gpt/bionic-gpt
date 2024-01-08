+++
title = "Adding a Jupyter Notebook"
weight = 70
sort_by = "weight"
+++

We can extend our `docker-compose` to include a [Jupyter Notebook](https://jupyter.org/).

Store the following in `docker-compose-jupyter.yml`

```yml
services:

  jupyter:
    image: jupyter/minimal-notebook
    ports:
      - 8888:8888
```

Then run

```sh
docker-compose -f docker-compose.yml -f docker-compose-jupyter.yml up
```

## Getting the Login URL

In the logs you should see something like the below

```
http://127.0.0.1:8888/lab?token=b2eb9f7b5e0fafaef985b734e6fc4ad8f0e58c529c15f73e
```

This is the authentication token. You'll need to use that URL to Login.

## Accessing the various APIs

Example accessing the embeddings API.  open up a notebook terminal and run

```
curl embeddings-api:80/embed     -X POST     -d '{"inputs":"What is Deep Learning?"}'     -H 'Content-Type: application/json'
```


![Alt text](../jupyter-notebook.png "Jupyter Notebook")


## Accessing the Database ##

![Alt text](../jupyter-database.png "Connect to Database")


## Accessing Embeddings From Python ##

![Alt text](../jupyter-embedding.png "Embedding Calls")


## Embedding Based Query on Database ##

![Alt text](../jupyter-embedding-query.png "Database Embedding Calls")