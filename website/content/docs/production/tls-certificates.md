+++
title = "TLS Termination"
weight = 70
sort_by = "weight"
+++

We use [envoy](https://www.envoyproxy.io/) to proxy requests for auth as well as adding [CSP](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) headers. So it makes sense to terminate TLS with envoy.

## Certificates

You're going to need certificates that are valid with your companies browsers. Either by getting them signed by an external entity such as [Let's Encrypt](https://letsencrypt.org/) or some internal process.

One possible tool to use is [cert-manager](https://cert-manager.io/)

## Add Certificates to a Secret


## Passing the secrets to envoy