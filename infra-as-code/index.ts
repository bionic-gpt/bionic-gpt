import * as k8s from "@pulumi/kubernetes"
//import { setupDatabase } from './database'
import { setupCluster } from './cluster-setup'
import { setupDatabase } from './database'
import * as kx from "@pulumi/kubernetesx"
import * as random from "@pulumi/random"


// Add a postgres operator and anything else apllications need
const cloudnativePg = setupCluster()

// Setup a namespace for our application
const applicationNameSpace = new k8s.core.v1.Namespace('bionic-gpt', {
    metadata: {
        name: 'bionic-gpt'
    },
})

setupDatabase(applicationNameSpace, cloudnativePg)

const cookieKey = new random.RandomId("cookie-encryption", {
    byteLength: 8
});

// Create a cookie encryption secret
new kx.Secret('bionic-gpt-cookie-encryption', {
    metadata: {
        namespace: applicationNameSpace.metadata.name,
        name: "cookie-encryption"
    },
    stringData: {
        "cookie-encryption-key": cookieKey.hex,
    }
})