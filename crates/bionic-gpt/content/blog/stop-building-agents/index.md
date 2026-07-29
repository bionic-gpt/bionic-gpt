## Introduction

What I hope to show in this article is that if you're considering building an AI agent or even an AI platform you should first be aware of the capabilities of your existing tools.

## Chat UI Capabilities have accelerated lately

The models have gotten better but most importantly agentic sandboxes have been integrated into most of the chat user interfaces.

Here's a prompt you can run, it's basically a bash script. It shows that the providers are running a sandboxed Linux environment every time you run  a prompt.

```
Run the script below in your sandbox

message="I'm running in a bash sandbox"

printf '< %s >\n' "$message"
printf '        \   ^__^\n'
printf '         \  (oo)\_______\n'
printf '            (__)\       )\/\\\n'
printf '                ||----w |\n\n'

printf 'User:     '; whoami
printf 'Host:     '; hostname
printf 'System:   '; uname
printf 'Folder:   '; pwd
printf '\nWorkspace:\n'
tree
```

The screenshots show this prompt running in each of the main stream chat UI's.

<div class="not-prose my-8 grid grid-cols-1 gap-4 md:grid-cols-3">
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="sandbox-chat-gpt.jpeg" alt="ChatGPT sandbox interface screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">ChatGPT: chat plus sandbox</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="sandbox-mistral-vibe.jpeg" alt="Mistral Vibe sandbox interface screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Mistral Vibe: chat plus sandbox</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="sandbox-bionic-gpt.jpeg" alt="Bionic GPT sandbox interface screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Bionic GPT: chat plus sandbox</figcaption>
  </figure>
</div>

## Memory 

```txt
What do you know about me?
```

## Projects - screenshots - bigger uptake

<div class="not-prose my-8 grid grid-cols-1 gap-4 md:grid-cols-3">
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="projects-chat-gpt.png" alt="ChatGPT Projects screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">ChatGPT Projects</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="projects-mistral-vibe.png" alt="Mistral Vibe Projects screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Mistral Vibe Projects</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="projects-bionic-gpt.png" alt="Bionic GPT Projects screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Bionic GPT Projects</figcaption>
  </figure>
</div>

## Tools - Integrations/etc

<div class="not-prose my-8 grid grid-cols-1 gap-4 md:grid-cols-3">
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="tools-chat-gpt.png" alt="ChatGPT Tools screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">ChatGPT Tools</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="tools-mistral-vibe.png" alt="Mistral Vibe Tools screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Mistral Vibe Tools</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="tools-bionic-gpt.png" alt="Bionic GPT Tools screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Bionic GPT Tools</figcaption>
  </figure>
</div>

## Skills

<div class="not-prose my-8 grid grid-cols-1 gap-4 md:grid-cols-3">
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="skills-chat-gpt.png" alt="ChatGPT Skills screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">ChatGPT Skills</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="skills-mistral-vibe.png" alt="Mistral Vibe Skills screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Mistral Vibe Skills</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="skills-bionic-gpt.png" alt="Bionic GPT Skills screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Bionic GPT Skills</figcaption>
  </figure>
</div>

## Datasets - RAG

<div class="not-prose my-8 grid grid-cols-1 gap-4 md:grid-cols-3">
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="library-chat-gpt.jpeg" alt="ChatGPT Library screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">ChatGPT Library</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="library-mistral-vibe.png" alt="Mistral Vibe Library screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Mistral Vibe Library</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="library-bionic-gpt.png" alt="Bionic GPT Library screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Bionic GPT Library</figcaption>
  </figure>
</div>

## Named Agents - Deprecated?

<div class="not-prose my-8 grid grid-cols-1 gap-4 md:grid-cols-3">
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="gpts.webp" alt="ChatGPT GPTs" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">ChatGPT GPTs</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="mistral-agents.png" alt="Mistral Agents" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Mistral Agents</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="bionic-assistants.png" alt="Bionic Assistants" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Bionic Assistants</figcaption>
  </figure>
</div>

## The complete Agent


![Sandbox](the-sandbox.png "Sandbox")

## Scheduled Tasks

<div class="not-prose my-8 grid grid-cols-1 gap-4 md:grid-cols-2">
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="tasks-chat-gpt.png" alt="ChatGPT Tasks screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">ChatGPT Tasks</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="tasks-mistral-vibe.png" alt="Mistral Vibe Tasks screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Mistral Vibe Tasks</figcaption>
  </figure>
</div>

## Responses i.e. Graphs, Gen UI, Maps?

<div class="not-prose my-8 grid grid-cols-1 gap-4 md:grid-cols-3">
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="canvas-chat-gpt.png" alt="ChatGPT Canvas screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">ChatGPT Canvas</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="canvas-mistral-vibe.png" alt="Mistral Vibe Canvas screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Mistral Vibe Canvas</figcaption>
  </figure>
  <figure class="overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-sm">
    <img class="aspect-[16/10] w-full object-cover object-top" src="canvas-bionic-gpt.png" alt="Bionic GPT Canvas screenshot" />
    <figcaption class="px-3 py-2 text-sm font-semibold text-slate-600">Bionic GPT Canvas</figcaption>
  </figure>
</div>

## Multi use case. 

People trigger multiple use cases in 1 chat.

## Use case evelauation

![Use case evaluation ladder](use-case-evaluation.svg "Use case evaluation ladder")

## Conclusion

I suspect that most organisations don't have the problem that they need to build AI agents. They have the problem that most of their employees are not fully up to speed yet with the capabilities of their existing AI Chat UI.