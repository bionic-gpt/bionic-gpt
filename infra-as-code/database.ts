
import * as pulumi from "@pulumi/pulumi"
import * as k8s from "@pulumi/kubernetes"
import * as kx from "@pulumi/kubernetesx"
import * as random from "@pulumi/random"
import { Release } from "@pulumi/kubernetes/helm/v3";

export function setupDatabase(
    applicationNameSpace: k8s.core.v1.Namespace, 
    cloudnativePg: Release) {

    // Create all the role passwords
    const migrationPassword = new random.RandomPassword("migration_password", {
        length: 20,
        special: false,
    });
    const applicationPassword = new random.RandomPassword("application_password", {
        length: 20,
        special: false,
    });
    const readonlyPassword = new random.RandomPassword("readonly_password", {
        length: 20,
        special: false,
    });
    const authenticationPassword = new random.RandomPassword("authentication_password", {
        length: 20,
        special: false,
    });

    const DATABASE_NAME = "app"
    const MIGRATIONS_ROLE = "migrations"

    const migrationsSecret = new kx.Secret("migrations-secret", {
        type: "kubernetes.io/basic-auth",
        metadata: {
            namespace: applicationNameSpace.metadata.name,
            name: "migrations-secret"
        },
        stringData: {
            "username": MIGRATIONS_ROLE,
            "password": migrationPassword.result,
        }
    })

    const pgCluster = new k8s.apiextensions.CustomResource('bionic-gpt-db-cluster', {
        apiVersion: 'postgresql.cnpg.io/v1',
        kind: 'Cluster',
        metadata: {
            name: 'bionic-gpt-db-cluster',
            namespace: applicationNameSpace.metadata.name,
        },
        spec: {
            instances: 1,
            bootstrap: {
                initdb: {
                    database: DATABASE_NAME,
                    // Bootstrap uses the secrets we created
                    // above to give us a user
                    owner: migrationsSecret.stringData.username,
                    secret: {
                        name: migrationsSecret.metadata.name
                    },
                    postInitSQL: [
                        // Add users here.
                        pulumi.all([applicationPassword.result])
                            .apply(([password]) =>
                                `CREATE ROLE ft_application LOGIN ENCRYPTED PASSWORD '${password}'`),
                        pulumi.all([authenticationPassword.result])
                            .apply(([password]) =>
                                `CREATE ROLE ft_authentication LOGIN ENCRYPTED PASSWORD '${password}'`),
                        pulumi.all([readonlyPassword.result])
                            .apply(([password]) =>
                                `CREATE ROLE ft_readonly LOGIN ENCRYPTED PASSWORD '${password}'`)
                    ],
                    postInitApplicationSQL: [
                        "CREATE EXTENSION IF NOT EXISTS vector"
                    ]
                }
            },
            storage: {
                size: '1Gi'
            }
        }
    }, {
        dependsOn: cloudnativePg
    })

    let migrationsUrl = pulumi.all([migrationPassword.result, pgCluster.metadata.name])
        .apply(([password, host]) =>
            `postgres://${MIGRATIONS_ROLE}:${password}@${host}-rw:5432/${DATABASE_NAME}?sslmode=require`)

    let authenticationUrl = pulumi.all([authenticationPassword.result, pgCluster.metadata.name])
        .apply(([password, host]) =>
            `postgres://ft_authentication:${password}@${host}-rw:5432/${DATABASE_NAME}?sslmode=require`)

    let readonlyUrl = pulumi.all([readonlyPassword.result, pgCluster.metadata.name])
        .apply(([password, host]) =>
            `postgres://ft_readonly:${password}@${host}-rw:5432/${DATABASE_NAME}?sslmode=require`)

    let applicationUrl = pulumi.all([applicationPassword.result, pgCluster.metadata.name])
        .apply(([password, host]) =>
            `postgres://ft_application:${password}@${host}-rw:5432/${DATABASE_NAME}?sslmode=require`)


    // Create a database url secret so our app will work.
    new kx.Secret("database-urls", {
        metadata: {
            namespace: applicationNameSpace.metadata.name,
            name: "database-urls"
        },
        stringData: {
            "migrations-url": migrationsUrl,
            "application-url": applicationUrl,
            "authentication-url": authenticationUrl,
            "readonly-url": readonlyUrl,
        }
    })
}