<iframe width="560" height="315" src="https://www.youtube.com/embed/fBC-5MYemao?si=ZoL6It4xYK1pXs0D" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

## Tutorial

Firstly if you've followed the enterprise setup into Kubernetes you can install Airbyte using helm.

```sh
helm repo add airbyte https://airbytehq.github.io/helm-charts
```

And install into a namespace called `airbyte`.

```sh
helm install airbyte --create-namespace --namespace airbyte airbyte/airbyte
```

## Open a port in K9s

In K9s move to the namespace `airbyte` to select a namespace type `:ns`.

You can now connect to Airbyte using from localhost `http://localhost:8000`

## Install RabbitMQ

We use [RabbitMQ](https://www.rabbitmq.com/) as a way to communicate with Airbyte

Run the following...

```sh
echo "
apiVersion: v1
kind: Namespace
metadata:
  name: rabbitmq

---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rabbitmq-deployment
  namespace: rabbitmq
spec:
  replicas: 1
  selector:
    matchLabels:
      app: rabbitmq
  template:
    metadata:
      labels:
        app: rabbitmq
    spec:
      containers:
      - name: rabbitmq
        image: \"rabbitmq:3-management\"
        ports:
        - containerPort: 5672
        - containerPort: 15672
        env:
        - name: RABBITMQ_DEFAULT_USER
          value: \"admin\"
        - name: RABBITMQ_DEFAULT_PASS
          value: \"admin\"

---
apiVersion: v1
kind: Service
metadata:
  name: rabbitmq-service
  namespace: rabbitmq
spec:
  selector:
    app: rabbitmq
  ports:
    - name: rabbitmq-port
      protocol: TCP
      port: 5672
      targetPort: 5672
    - name: rabbitmq-management-port
      protocol: TCP
      port: 15672
      targetPort: 15672
  type: ClusterIP
" | kubectl apply -f -
```

## Create a rabbit MQ queue.

Go to the `rabbitmq` namespace in `K9s` and hit `shift+f` and open a port into the RabbitMQ admin interface which runs on port 15672.

You can then access the admin interface via `http://localhost:15672`.

1. Create a queue called `bionic-pipeline`
1. Create a binding to the queue from `amq.topic` to `*.bionic-pipeline`

## Testing the Queue (Optional)

```sh
curl -i -u admin:admin -H "Content-Type: application/json" -X POST -d '{"properties":{},"routing_key":"123456.bionic-pipeline","payload":"Your_Message_Content","payload_encoding":"string"}' http://localhost:15672/api/exchanges/%2F/amq.topic/publish
```

If the message was successfully routed you should see

```json
{"routed":true}
```

## Create an Airbyte -> RabbitMQ destination

1. From the Airbyte UI create a destination where the routing key is `API_KEY.bionic-pipeline` where `API_KEY` is the key we created in the bionic user interface for a document pipeline.
1. The destination should be set to `rabbitmq-service.rabbitmq.svc.cluster.local`.
1. Switch off SSL
1. The username is `admin`
1. The password is `admin`
1. Exchange is `amq.topic`

## Github connection

1. Get an access token from Github
1. Create a Github source and add the access token.

## Install Bionic RabbitMQ Listener

```sh
echo '
apiVersion: batch/v1
kind: CronJob
metadata:
  name: rabbitmq-cronjob
  namespace: bionic-gpt
spec:
  schedule: "*/1 * * * *"
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: rabbitmq-container
            image: ghcr.io/bionic-gpt/bionicgpt-rabbitmq:latest
            env:
              - name: RABBITMQ_URL
                value: "http://rabbitmq-service.rabbitmq.svc.cluster.local:15672/api/queues/%2f/bionic-pipeline/get"
              - name: USERNAME
                value: "admin"
              - name: PASSWORD
                value: "admin"
              - name: UPLOAD_URL
                value: "http://bionic-gpt.bionic-gpt.svc.cluster.local:7903/v1/document_upload"
          restartPolicy: Never
' | kubectl apply -f -
```