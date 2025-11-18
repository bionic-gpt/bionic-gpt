# Creating an SMTP secret

Bionic needs to connect with your SMTP provider. Out of the box we install [MailHog](https://github.com/mailhog/MailHog) which is an SMTP testing server.

You can open a port to MailHog using K9s and check the email coming from Bionic with your browser.

To use another provider you'll need to override the Kubernetes secret that we create on installation.

## Creating the secret

The below is an example using the data we already setup when you install Bionic. You'll need to look at the parameters and set them as the same for you provider.

```sh
kubectl delete secret smtp-secrets -n bionic-gpt
kubectl create secret generic smtp-secrets -n bionic-gpt \
    --from-literal=invite-from-email-address=support@application.com \
    --from-literal=smtp-host=mailhog \
    --from-literal=smtp-password=thisisnotused \
    --from-literal=smtp-port=1025 \
    --from-literal=smtp-tls-off=true \
    --from-literal=smtp-username=thisisnotused \
    --from-literal=invite-domain=http://your-hostname.com
```

## Restart the Bionic Server

```sh
kubectl rollout restart deployment bionic-gpt -n bionic-gpt
```

You should now be able to see Bionic sending email to your provider.