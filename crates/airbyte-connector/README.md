## Install

```
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
          restartPolicy: Never
' | kubectl apply -f -
```

## Mirrord

For mirrord install as a Job so we can capture the pod.

```
echo '
apiVersion: v1
kind: Pod
metadata:
  name: do-nothing-pod
  namespace: bionic-gpt
spec:
  containers:
  - name: dummy-container
    image: busybox
    command: ["/bin/sh", "-c"]
    args: ["sleep 3600"]  # Sleep for 1 hour (you can adjust the duration)
' | kubectl apply -f -

```

```
mirrord exec -n bionic-gpt -t pod/do-nothing-pod cargo run -- --bin rabbit-mq
```