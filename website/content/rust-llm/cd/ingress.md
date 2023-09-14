+++
title = "Cloudflare as Ingress"
description = "Cloudflare as Ingress"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 30
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

[cloudfared](https://github.com/cloudflare/cloudflared) is a tool we can use to connect our cluster securely to the outside world.

Cloudflare also gives us the option to use [Quick Tunnels](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/do-more-with-tunnels/trycloudflare/) so we don't even have to setup a Cloudflare account at this stage.

Cloudflare will handle TLS termination for us, saving us some setup on our cluster.

## Setting up the tunnel

Create a `infra-as-code/cloudflare.ts` and add the following

```typescript
import * as kx from "@pulumi/kubernetesx"
import * as pulumi from "@pulumi/pulumi"

export function cloudflareTunnel(
    namespace: pulumi.Output<string>,
    url: string) {
    const cloudflaredPod = new kx.PodBuilder({
        containers: [{
            name: "cloudflare-tunnel",
            image: "cloudflare/cloudflared:latest",
            command: ["cloudflared", "tunnel", "--url", url],
        }]
    })

    let deployName = pulumi.interpolate `${namespace}-cloudflare-tunnel`
    
    new kx.Deployment("cloudflare-tunnel", {
        metadata: {
            name: deployName,
            namespace: namespace
        },
        spec: cloudflaredPod.asDeploymentSpec({ replicas: 1 })
    })
}
```

## Creating a service

We need to create a [Kubernetes Service](https://kubernetes.io/docs/concepts/services-networking/service/) so that the Cloudflare tunnel can see out application.

Add the following to bottom of `index.ts`

```typescript
new k8s.core.v1.Service("application", {
    metadata: {
        name: "application",
        namespace: applicationNameSpace.metadata.name
    },
    spec: {
        ports: [
            { port: 3000, targetPort: 3000 }
        ],
        type: "ClusterIP",
        selector: {
            app: deployment.metadata.name
        }
    }
})

cloudflareTunnel(applicationNameSpace.metadata.name, "http://application:3000")
```

At the top of `index.ts` import our `cloudflareTunnel` function.

```typescript
import { cloudflareTunnel } from './cloudflare'
```

## Getting our external URL


![Cloudflare URL](../cloudflare-url.png)