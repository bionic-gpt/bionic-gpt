+++
title = "TLS Termination"
weight = 90
sort_by = "weight"
+++

We use [envoy](https://www.envoyproxy.io/) to proxy requests for auth as well as adding [CSP](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) headers. So it makes sense to terminate TLS with envoy.

## Certificates

You're going to need certificates that are valid with your companies browsers. Either by getting them signed by an external entity such as [Let's Encrypt](https://letsencrypt.org/) or some internal process.

One possible tool to use is [cert-manager](https://cert-manager.io/)

## Add Certificates to a Secret

If you're using cert manager this should already happen for you.

## Passing the secrets to envoy

Todo - Create a docker for this.

```yml
static_resources:
  listeners:
  - address:
      socket_address:
        address: 0.0.0.0
        port_value: 443
    filter_chains:
    - filters:
      - name: envoy.http_connection_manager
        config:
          codec_type: auto
          stat_prefix: ingress_http
          route_config:
            name: local_route
            virtual_hosts:
            - name: backend
              domains:
              - "example.com"
              routes:
              - match:
                  prefix: "/"
                route:
                  cluster: envoy
          http_filters:
          - name: envoy.router
            config: {}
      tls_context:
        common_tls_context:
          tls_certificates:
            - certificate_chain:
                filename: "/etc/example-com.crt"
              private_key:
                filename: "/etc/example-com.key"
  clusters:
  - name: envoy
    connect_timeout: 0.25s
    type: strict_dns
    lb_policy: round_robin
    http2_protocol_options: {}
    hosts:
    - socket_address:
        address: service1
        port_value: 80
admin:
  access_log_path: "/dev/null"
  address:
    socket_address:
      address: 0.0.0.0
      port_value: 8001
```

## Pass the secrets into envoy

We can pass the secrets into envoy as config vars.