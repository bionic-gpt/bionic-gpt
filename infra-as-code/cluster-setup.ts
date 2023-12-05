import * as k8s from "@pulumi/kubernetes"
import { Release } from "@pulumi/kubernetes/helm/v3";

export function setupCluster() : Release {
    // Setup a namespace for Cloud Native Pg https://github.com/cloudnative-pg/cloudnative-pg
    const databaseNameSpace = new k8s.core.v1.Namespace('cloud-native-pg', {
        metadata: {
            name: 'cloud-native-pg'
        },
    })

    // Install the Postgres operator from a helm chart
    const cloudnativePg = new k8s.helm.v3.Release("cloudnative-pg", {
        chart: "cloudnative-pg",
        namespace: databaseNameSpace.metadata.name,
        repositoryOpts: {
            repo: "https://cloudnative-pg.github.io/charts",
        }
    });

    return cloudnativePg
}