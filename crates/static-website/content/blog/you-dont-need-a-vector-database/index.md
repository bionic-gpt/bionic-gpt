## The short version

[Postgres](https://www.postgresql.org/) and [pgVector](https://github.com/pgvector/pgvector) do everything a specialist Vector database can do and a whole lot more. If you choose a specialist Vector database you're going to end up needing 2 databases.

### 1. You don't need "a database for AI" you need a database

From a discussion on [HN](https://news.ycombinator.com/item?id=37420628)

> Exactly. The whole point about databases is you don't need "a database for AI" **you need a database, ideally with an extension to add additional AI functionality (i.e. postgres and pgVector)**. Trying to take a special store you invent for AI and retrofit all the desirable things you need to make it work properly in the context of a real application you're just going to end up with a mess.

> As a thought-experiment for people who don't understand why you need (for example) regular relational columns alongside vector storage, consider how you would implement [RAG](/blog/retrieval-augmented-generation/) for a set of documents where not everyone has permission to view every document. In the pgVector case it's easy - I can add one or more label columns and then when I do my search query filter to only include labels that user has permission to view. Then my vector similarity results will definitely not include anything that violates my access control. Trivial with something like pgVector - basically impossible (afaics) with special-purpose vector stores.

> Or think about ranking. Say you want to do RAG over a space where you want to prioritise the most recent results, not just pure similarity. Or prioritise on a set of other features somehow (source credibility whatever). Easy to do if you have relational columns, no bueno if you just have a vector store.

> And that's not to mention the obvious things around ACID, availability, recovery, replication, etc.

Thanks to [Sean Hunter](https://news.ycombinator.com/submitted?id=seanhunter) who articulates it so well.

### 2. Real world requirements

As an architect working on LLM applications I have these criteria for a database.

- Full SQL support

- Has good tooling around migrations (i.e. dbmate)

- Good support for running in Kubernetes or in the cloud

- Well understood by operations i.e. backups and scaling

- Supports vectors and similarity search.

- Well supported client libraries

So basically Postgres and PgVector.

### 3. It's a long way from your laptop to production

Here's an outline of the Roles involved in getting a database application into production.

* Database Administrator (DBA): Responsible for database design, performance optimization, security, backups, and overall database management.

* Database Developer: Designs and develops the database schema, writes SQL queries, and ensures efficient data retrieval and manipulation.

* DevOps Engineers: Implement database automation, continuous integration/continuous deployment (CI/CD) pipelines, and manage infrastructure.

You may have 1 person that takes on all these roles or you might be the person taking on this role. The point is you need a story around how the database will perform in production especially if you can't use a hosted solution which is often the case for corporate data.

### 4. Pick learning curves that pay off year after year

I learned Postgres 15 years ago. In that time new technologies have come along and Postgres has assimilated the useful ones into it's code base.

Reusing a technology over many projects means he can fix issues quickly as you've often seen them before.

Basically, You hit the ground running.

Here's some other tech that I predict will last you a whole career.

1. SQL
1. Docker
1. Kubernetes

### 5. How about performance?

For an LLM architecture the main bottle neck is probably going to be the LLM itself.

So performance will only be an issue for pgVector if it turns out to be a lot slower than the competition.

There's also another Postgres solution that claims to be [20x faster the pgVector](https://neon.tech/blog/pg-embedding-extension-for-vector-search) 

