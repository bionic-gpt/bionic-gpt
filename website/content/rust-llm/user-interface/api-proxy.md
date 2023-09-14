+++
title = "API Proxy"
description = "T"
draft = false
weight = 30
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

If we have code on the client side that calls our API then we need to somehow proxy requests from the browser to the API server that runs in our backend.

This means we expose our API to the open internet (or the open intranet if you're completely on prem).

This gives us the following problems

1. How do we secure this API so that only authorized users access the API end points.
1. How do we route both incoming HTTP requests for our we application and requests for the API to the correct servers.

## Envoy as an API Proxy

With [Envoy](https://www.envoyproxy.io/) we get a solution to our problems and much more. In this case we'll set up Envoy to route our requests and later with use it to enforce authentication.