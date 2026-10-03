## TLDR Just use bashkit. Need more convincing than that, read on.

## What problem are we trying to solve

## Start With a Task

Consider a simple enterprise agent evaluation:

> Analyze orders.csv. Identify the five customers whose spending has increased the most over the last six months compared with the previous six months.
> Create an Excel workbook containing the analysis, the underlying monthly totals, and a chart showing the trend for each customer.

## Lets imagine we only have 2 tools

```json
[
  {
    "name": "run_bash",
    "description": "Run a Bash command.",
    "parameters": {
      "command": {
        "type": "string"
      }
    }
  },
  {
    "name": "run_python",
    "description": "Run Python code.",
    "parameters": {
      "code": {
        "type": "string"
      }
    }
  }
]
```

- code mode
- tool discovery

## We Need a Filesystem

Once capabilities are accessed programmatically, a filesystem becomes
extremely useful.

It gives the model somewhere to discover capabilities, inspect data, keep
intermediate results and produce artifacts.

For example:

```sh
/
├── skills/
│   ├── salesforce/
│   │   ├── SKILL.md
│   │   └── operations/
│   │       ├── query.md
│   │       ├── create-lead.md
│   │       └── update-contact.md
│   └── gmail/
│       └── ...
│
├── memory/
│   └── ...
│
└── workspace/
    ├── uploads/
    └── generated/
```

The filesystem now becomes part of the agent interface.

## System Prompt

## What Our Eval Might Look Like

Our original task can now be executed through a very small model-facing
interface.

The model might:

```sh
$ cat /skills/gmail/SKILL.md
$ [search Gmail for Jordan Lee]
$ cat /skills/salesforce/SKILL.md
$ [query Salesforce for Jordan Lee]
$ [update the phone number]
```

## Option 1: A Compute Sandbox

Unfortunaltey this is the most omplicated way to solve 

You can simplify this as a Dockerfile

- developer drift
- need to manage loading and updating files.
- start stop etc
- Kubernetes

## Option 2: Bashkit

- much simpler
- very fast
- can support a virtual file system
- multi threaded no processes
- Operational simplicity

## Skills - Steer as well as map the capbility surface

We're already starting to steer models by skill category

https://github.com/Mercor-Intelligence/archipelago

| Skill | Example capabilities |
|---|---|
| **Spreadsheets** | Read/write XLSX, formulas, formatting, charts, tables |
| **Documents** | Read/create/edit DOCX, formatting, templates |
| **PDFs** | Extract content, create PDFs, merge/split, forms |
| **Presentations** | Read/create/edit slides, layouts, charts |
| **Web** | Search, retrieve pages, extract information |
| **Images** | Inspect, resize, convert, extract information |

Then 1 skill per api.

we already know a bunch of use cases for the prosos thing.

## Capability trade off

- Sandboxes could allow pip install etc, but this is still an open question
- pandas etc add capbilities
- Bench marks show the way

## Conclusion

Bashkit is a very simple way to handle potentially milions of use cases in the neterpirsie, combined with some skills, api discovery I think it will cover so much.

You might cover more use cases with a sandbox, but now you have to worry more about operations and security.