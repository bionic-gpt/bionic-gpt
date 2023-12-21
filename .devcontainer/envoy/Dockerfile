FROM envoyproxy/envoy:v1.28.0

RUN mkdir -p /etc/envoy
COPY ./envoy.yaml /etc/envoy/envoy.yaml
COPY ./token-secret.yaml /etc/envoy/token-secret.yaml
COPY ./hmac-secret.yaml /etc/envoy/hmac-secret.yaml

CMD ["/usr/local/bin/envoy","-c","/etc/envoy/envoy.yaml","--service-cluster","envoy","--service-node","envoy","--log-level","debug"]