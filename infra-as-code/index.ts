import * as k8s from "@pulumi/kubernetes"
//import { setupDatabase } from './database'
import { setupCluster } from './cluster-setup'


// Add a postgres operator and anything else apllications need
const cloudnativePg = setupCluster()

// Setup a namespace for our application
const applicationNameSpace = new k8s.core.v1.Namespace('bionic-gpt', {
    metadata: {
        name: 'bionic-gpt'
    },
})