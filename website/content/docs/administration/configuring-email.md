+++
title = "Configure Email"
weight = 60
sort_by = "weight"
+++

The following environment variables configure how Bionic sends out email for invitations.

| Item                          | Explanation                                         | Example                         |
|-------------------------------|-----------------------------------------------------|---------------------------------|
| SMTP_HOST                     | Hostname of the SMTP server                         | `smtp.example.com`              |
| SMTP_USERNAME                 | Username for SMTP authentication                    | `user@example.com`              |
| SMTP_PASSWORD                 | Password for SMTP authentication                    | `password123`                   |
| SMTP_PORT                     | Port number for SMTP communication                  | `587`                           |
| INVITE_DOMAIN                 | Domain used for sending invitations                 | `example.com`                   |
| INVITE_FROM_EMAIL_ADDRESS     | Email address from which invitations are sent       | `invite@example.com`            |

## For docker-compose.yml

The env vars will need to be added to your docker compose under the `app` section.

Something like

```yml
    environment:
      - SMTP_HOST=smtp.example.com
      - SMTP_USERNAME=user@example.com
      - SMTP_PASSWORD=password123
      - SMTP_PORT=587
      - INVITE_DOMAIN=example.com
      - INVITE_FROM_EMAIL_ADDRESS=invite@example.com
```