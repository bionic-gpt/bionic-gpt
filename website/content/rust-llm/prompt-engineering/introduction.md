+++
title = "Introduction"
description = "T"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

![Architecture](../prompt-engineering.webp)

## Example Prompt

https://github.com/bhaskatripathi/pdfGPT/blob/main/api.py

```
prompt += (
        "Instructions: Compose a comprehensive reply to the query using the search results given. "
        "Cite each reference using [ Page Number] notation (every result has this number at the beginning). "
        "Citation should be done at the end of each sentence. If the search results mention multiple subjects "
        "with the same name, create separate answers for each. Only include information found in the results and "
        "don't add any additional information. Make sure the answer is correct and don't output false content. "
        "If the text does not relate to the query, simply state 'Text Not Found in PDF'. Ignore outlier "
        "search results which has nothing to do with the question. Only answer what is asked. The "
        "answer should be short and concise. Answer step-by-step. \n\nQuery: {question}\nAnswer: "
    )
```

## Todo

- Figure out how to connect to LLM
- What do we need in terms of token limits (Falcon 40b 2048 token limit)
- MPT Story writer 65k tokens
- MPT 30b 8k