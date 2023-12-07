+++
title = "Reset Database and Uninstall"
weight = 50
sort_by = "weight"
+++

If you want to reset the database back to factory defaults run the following

## CPU Database reset

If you installed our CPU based `docker-compose.yml` then simply...

```sh
docker-compose down -v
```

The `-v` removes all volumes which drops all the tables from the database.

## GPU Database reset

If you used our GPU based `docker-compose` setup then...

```sh
docker-compose -f docker-compose.yml -f docker-compose-gpu.yml down -v
```

## Removing the docker containers

For CPU

```sh
docker-compose rm -v
```

and for GPU

```sh
docker-compose -f docker-compose.yml -f docker-compose-gpu.yml rm -v
```
