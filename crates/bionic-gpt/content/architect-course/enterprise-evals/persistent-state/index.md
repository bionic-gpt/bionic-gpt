# Persistent State

Agentic systems often need to remember structured information beyond a single
model invocation. In this eval, the model must create a persistent customer
database and use it again in a later step.

The architectural capability under test is durable state outside conversational
context. Bionic provides `/home/user/output` as the persistent workspace for a
conversation, while SQLite provides a lightweight mechanism for maintaining
structured state. The goal is not to test whether the model can recite SQLite
syntax.

The database skill is a system capability and should be available after the
database migrations have run. Discovering and reading it is useful evidence of
good skill selection, but is not required for a pass when the model otherwise
completes the workflow correctly.

## What this eval tests

- Can the model create a real SQLite database instead of returning a proposed
  schema in chat?
- Does it place state that must survive under `/home/user/output`?
- Can it design a sensible schema and store numeric values in a queryable form?
- Does the database remain available to a later `run_bash` call?
- Does the model query persisted state rather than reconstructing an answer
  from conversational memory?

## Turn 1: Create the state

Start a new conversation and enter this prompt:

```text
Create a customer database for a small B2B software company.

Store these customers:

* Acme GmbH — Enterprise — Germany — €120,000 ARR
* Northstar Ltd — Growth — UK — €48,000 ARR
* Delta Systems — Enterprise — France — €85,000 ARR

Include a sensible schema that would allow us to add contacts and notes later.
```

Do not give the model a filename or exact schema. It should choose an
appropriate database path and structure. Inspect the tool-call details and
record the path it creates for comparison with Turn 2.

### Expected behavior

1. The model may discover and read the `database` skill.
2. It uses `run_bash` and SQLite to create a database somewhere under
   `/home/user/output`.
3. It creates a customer schema that can be extended with contacts and notes.
4. It inserts all three supplied customers.
5. ARR is stored as a numeric value that supports threshold and aggregate
   queries, rather than only as formatted text.
6. It leaves the database in place for a later turn.

A SQL example, CSV, JSON file, or schema description without an actual SQLite
database does not pass this stage.

## Turn 2: Reuse the state

In the same conversation, enter only this follow-up prompt:

```text
Which customers have more than €50,000 ARR, and what is their combined ARR?
```

`/home/user/output` is conversation-scoped, so the current course framework
must run both turns in the same conversation. The earlier customer records are
therefore still present in chat history. To distinguish persistence from
memory, inspect the second turn's tool-call details: the model must open and
query the same database path created in Turn 1. A correct answer without that
database query does not pass.

The second turn should not recreate the database or reinsert the original
records. Its SQLite query should apply the ARR threshold and derive the total
from persisted rows.

## Expected result

The qualifying customers are:

- Acme GmbH
- Delta Systems

Their combined ARR is **€205,000**.

## Pass criteria

| Capability | Pass evidence | Failure signal |
| --- | --- | --- |
| Real database | Tool details show SQLite created a database file | The response only proposes SQL or another text format |
| Persistent location | The database path is under `/home/user/output` | The database is created under `/tmp` or another temporary path |
| Complete data | The database contains Acme GmbH, Northstar Ltd, and Delta Systems | Any supplied customer is missing or materially changed |
| Queryable ARR | ARR supports a numeric `> 50000` comparison and aggregation | ARR is only stored as formatted currency text |
| Cross-turn survival | Turn 2 opens the same path created in Turn 1 | The file is missing or a new database is created |
| Persisted-state use | Turn 2 runs SQLite against the existing database without reinserting rows | The answer is produced from chat memory or reconstructed data |
| Filter result | The query returns Acme GmbH and Delta Systems, but not Northstar Ltd | The qualifying set is incorrect |
| Aggregate result | The derived combined ARR is €205,000 | The total is absent or incorrect |

Skill loading can be recorded separately when comparing models, but it should
not turn an otherwise correct persistent-state workflow into a failure.

## The important boundary

Conversation history helps a model continue a discussion. Persistent state lets
an agent preserve structured information that tools can inspect, update, and
reuse across separate invocations. This eval passes only when the database is
the source of truth for the second step.
