+++
title = "Configure Email"
weight = 55
sort_by = "weight"
+++

## Creating an SMTP secret

Bionic needs to connect with your SMTP provider. Out of the box we install [MailHog](https://github.com/mailhog/MailHog) which is an SMTP testing server.

You can open a port to MailHog using K9s and check the email coming from Bionic with your browser.

To use another provider you'll need to override the Kubernetes secret that we create on installation.

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: smtp-secrets
  namespace: bionic-gpt
type: Opaque
data:
    invite-from-email-address: support@application.com
    smtp-host: mailhog
    smtp-password: thisisnotused
    smtp-port: 1025
    smtp-tls-off: true
    smtp-username: thisisnotused
    invite-domain: http://your-hostname.com

```