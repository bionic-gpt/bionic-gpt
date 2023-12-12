+++
title = "Connecting to the Database"
weight = 60
sort_by = "weight"
+++

We can extend our `docker-compose` to include [pgAdmin](https://www.pgadmin.org/).

Store the following in `docker-compose-pgadmin.yml`

```yml
services:

  pgadmin:
    image: dpage/pgadmin4
    ports:
      - "8888:80"
    environment:
      PGADMIN_DEFAULT_EMAIL: admin@admin.com
      PGADMIN_DEFAULT_PASSWORD: password
```

Then run

```sh
docker-compose -f docker-compose.yml -f docker-compose-pgadmin.yml up
```

Login to pgAdmin then

![Alt text](../pgadmin.webp "PG Admin")