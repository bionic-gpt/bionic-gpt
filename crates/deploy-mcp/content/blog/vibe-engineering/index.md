## Vibe Engineering Definition

The term "vibe engineering" comes form an article by [Simon Willison](https://simonwillison.net/2025/Oct/7/vibe-engineering/). He doesn't exactly give a defination but I put the article in Chat-GPT and this is the defintion I got out.

> Vibe engineering is the practice where experienced software engineers leverage large-language-model tools and agentic coding loops to build production-quality software with full accountability, rather than simply handing off tasks to AI and hoping it works.


[![Alt text for image](vibe-engineering-blog-article.png)](https://simonwillison.net/2025/Oct/7/vibe-engineering/)

So rather than Vibe Coding where we don't look at the code, we're going to put best practices in place.

Tjhe article resonated with me as thats how I've been working for the last few months.

## It's the Software Development Life Cycle

Setting up a software engineering best practices is difficlut and I rarely see it done well.

I've seen projects actively recruiting developers and they are elaving mone yon the table ith poor best practices.

I'll also take some real world examples to make it real.

![Dev Ops](ci-cd.jpg "Dev Ops")

## The Pull Request

When you think about it. All we want to do is create more pull requests.

- [Example from Codex](https://github.com/openai/codex/pull/5504)


![Pull Request](pull-request.png "Pull Request")

## Preparing Enterprise Projects for AI

### Best Practices

- [AGENTS.md](https://agents.md/)
- Devconatiner 

## Using AI for planning

Example prompt

> I want to add payments to this application. Come up with a techincal specification and suggestions for how we can best do this.

![Dev Ops](codex.png "Codex")

- The agent has access to your code
- Given your requirment it can create a technical design plan
- You can feed back into this plan.

In reality I don't dso thid very often. Mostly I take a prompt and get the agent to generate code.


## The code loop

![Dev Ops](codex.png "Codex")

- We're always using version control and can revert the code at anytime.
- The agent has access to all the developer tools
- We can see what code has chnaged so far and request chnages.
- When we are happy the agent can create a pull request.

### Some best practices

- Devcontainer
- Code review

## Build (the CI in CI/CD)

- The pull request is the quality gate keeper
- At this stage our Ci CD pipeline kiks off
- Its the same for humans as AI

### Best Practices

- Be able to run the pipeline locally.

## Test

It would be nice.

https://github.com/openai/codex/blob/main/.github/workflows/rust-release.yml

## Best Parcytice Take Aways

- Devcontainer
- Agents.md
- CI/CD
