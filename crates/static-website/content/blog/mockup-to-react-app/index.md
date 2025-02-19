## Generating Mockups from a Prompt

### The Prompt

```md
# Create a clickable web application 

Create an application using HTML, JavaScript and CSS that is a clickable prototype.

## Files

- The current version is `v1`
- The current model is `qwen`
- `index-{version}-{model}.html` for the HTML
- `js-{version}-{model}.js` for the JavaScript
- `css-{version}-{model}.css` for any CSS that is needed

## Entities

The prototype will be a *CRUD* application for the following entities

- Aircraft(name, id, airline)
- Airlines(name, id)
- Users(name, id, email)

Make sure to add demo data for each entity.

## Features

- Create a side menu item for each entity.
- Click on an entity to see a table of that entity.
- give the user the ability to add, delete and edit entities.
- store all the data in memory.
- When the user submits a form take them back to the table view.
- Use tailwind css from a CDN for styling.
```

### The Results

I actually tried a variety of models with varying results.

- [Gemini](./gemini/index-v1-gemini.html)
- [o3 mini](./o3-mini/index-v1-03-mini.html)
- [sonnet](./sonnet/index-v1-sonnet.html)
- [qwen](./qwen/index-v1-qwen.html)
- [GPT 4o](./gpt-4o/index-v1-gpt4o.html)

### The Issues

- Different models very different results 
- API issues
- Whole day couldn't get things working

## Extending the Mockup

```md
# Extend The Clickable Prototype

## Existing Files

- The current version is `v1`
- The current model is `qwen`
- `index-{version}-{model}.html` for the HTML
- `js-{version}-{model}.js` for the JavaScript
- `css-{version}-{model}.css` for any CSS that is needed

## Task 1

Use the command line to make a copy of the `v1` files and make a `v2`

## Task 2

The files are a HTML,CSS and Javascript mockup of a CRUD application. Using our `v2` files add the following functionality.

- Add another entity called Pilots
- Put a fake company logo in the top left corner
- Add a mock up of a login and registration screen
- prepopulate the login screen so I can login easily.
- Add a landing page and highlight our amazing SaaS application
```

### The Results

I actually tried a variety of models with varying results.

- [Gemini](./gemini/index-v2-gemini.html)
- [o3 mini](./o3-mini/index-v2-03-mini.html)
- [sonnet](./sonnet/index-v2-sonnet.html)
- [qwen](./qwen/index-v2-qwen.html)
- [GPT 4o](./gpt-4o/index-v2-gpt4o.html)

## CONTRIBUTING.md

[CONTRIBUTING.md](https://gist.github.com/242816/2fd63f5b3a95c4149f7ac02e0be870ca)

## Setting up an application

```sh
pnpm create t3-app@latest --noGit --CI --drizzle --tailwind --dbProvider sqlite --appRouter
```

## Plan our React APP

```md
## Files

You have access to the following files

- index.html which is an HTML and javascript clickable mockup of our application.
- CONTRIBUTING.md shows our desired tech stack and gives guidance on our architecture.

## Task

Let's plan the `Aircraft` functionality.
```

## Architecture Drift

## Battle of the Prompts