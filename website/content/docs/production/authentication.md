+++
title = "Authentication"
weight = 110
sort_by = "weight"
+++

Out of the box Bionic GPT authenticates all http requests coming through the [Envoy proxy](https://www.envoyproxy.io/) with [Barricade](https://github.com/purton-tech/barricade).

## Email OTP

By default we don't check the users email address on registration or sign in. However you can enable this by setting the following environment variable for Barricade.

```yml
ENABLE_EMAIL_OTP: 'true'
```

You will need to have all your SMTP configuration working as now barricade will send a one time password (OTP) to the users email address to verify that they own the address.