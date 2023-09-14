+++
title = "Database Backups and Development"
description = "Database Backups"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 140
sort_by = "weight"


[extra]
lead = 'Ideally we want to take regular backups of our database and also make those backups off site. We can achieve this by using a backup service. This is how we set it up.'
toc = true
top = false
section = "rust-llm"
+++

We can use a tool called [replibyte](https://www.replibyte.com/) to fix two problems in one go.

1. We need to be able to backup our database. `replibyte` can backup data from Postgres to S3 storage where S3 is a storage protocol supported by lots of providers.
2. We sometimes want to use production data in our development environments. `replibyte` allows us to do this and gives us tools to manage the size of the data as well as obscure fields.

## Creating a read only Database User

Ideally we want our backups to run with minimum database privileges. You could add the following to your database migrations.

```
CREATE ROLE readonly LOGIN ENCRYPTED PASSWORD '****************';
```

```
GRANT SELECT ON ALL TABLES IN SCHEMA public TO readonly;
GRANT SELECT ON ALL SEQUENCES IN SCHEMA public TO readonly;
```

## Configuration

We can create a generic configuration file in `crates/db/replibyte.yaml` like so. This will reference ENV vars only. 

```yaml
# This confuration file is for https://www.replibyte.com/ a tool for to tool to 
# seed your databases with your production data while keeping sensitive data safe.
#
# optional - encrypt data on datastore
encryption_key: $PRIVATE_ENC_KEY 
source:
  # A connection to your prod DB. Make it readonly.
  connection_uri: $SOURCE_DATABASE_URL
datastore:
  # It says AWS but you can use any S3 compatible service by setting the endpoint
  aws:
    bucket: $S3_BUCKET
    region: $S3_REGION
    credentials:
      access_key_id: $ACCESS_KEY_ID
      secret_access_key: $SECRET_ACCESS_KEY
    endpoint:
      custom: 'https://your-s3-compatible-endpoint'
# If you want to populate a dev database 
destination:
  connection_uri: $DEST_DATABASE_URL
```

Now create your S3 bucket and your cloud provider.

## Add Secrets to Cloak

To generate an encryption key you can use

```sh
$ openssl rand -hex 32
1c684969dc5ae500e320e579f4082da106e31edea190ca4f876de31cd5f6b1b8
```

Add your secrets to your secrets manager. Here we're using [Cloak](https://cloak.software).

![Adding secrets to cloak](../cloak-screenshot.png)

## Test Your Connection

If you're using `cloak`.

```sh
cloak replibyte -c crates/db/backup-conf.yaml source schema 
```

And you should see your database schema.