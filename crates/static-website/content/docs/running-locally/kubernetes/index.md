# Kubernetes on Docker Desktop

You probably already have Kubernetes installed without realising it. Most Docker Desktop setups include a built-in single-node cluster.

This gives us a quick way to test a more production style Bionic install.

## 1. Switch on Kubernetes in Docker Desktop

![Alt text](docker-kubernetes.png "Docker Desktop kubernetes")

Docker Desktop with Kubernetes enabled is the easiest option for macOS, Windows, and most Linux desktops.

## 2. Install Bionic with the Stack CLI

We use [Stack](https://stack-cli.com/) which is a Kubernetes Operator for installing applications in a secure and repeatable way. Stack manages the application and generates all required secrets.

1. **Grab the CLI.**

   ```bash
   curl -fsSL https://stack-cli.com/install.sh | bash
   ```

2. **Bootstrap the platform operators into your cluster.**

   ```bash
   stack init
   ```

   This command installs CloudNativePG, Keycloak, ingress, the Stack controller, and custom resource definitions that describe your applications.

   ```bash
   🔌 Connecting to the cluster...
   ✅ Connected
   🐘 Installing Cloud Native Postgres Operator (CNPG)
   ⏳ Waiting for Cloud Native Postgres Controller Manager
   🛡️ Installing Keycloak Operator
   📦 Creating namespace keycloak
   ⏳ Waiting for Keycloak Operator to be Available
   📦 Creating namespace stack-system
   📜 Installing StackApp CRD
   ⏳ Waiting for StackApp CRD
   🔐 Setting up roles
   🤖 Installing the operator into stack-system
   🗄️ Ensuring Keycloak database in namespace keycloak
   ✅ Keycloak database created.
   🛡️ Ensuring Keycloak instance in namespace keycloak
   ```

3. **Apply the demo StackApp manifest.**

   ```bash
   curl -fsSL https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/infra-as-code/stack.yaml \
     -o demo.stack.yaml
   ```

   And then

   ```
   stack deploy --manifest demo.stack.yaml --profile dev
   ```

   You should see

   ```bash
   🔌 Connecting to the cluster...
   ✅ Connected
   📜 Installing StackApp CRD
   ⏳ Waiting for StackApp CRD
   📦 Creating namespace bionic-gpt
   🚀 Applied StackApp `bionic-gpt` in namespace `bionic-gpt`
   ```

## 3. Done

Hopefully if everything went well you should be able to access bionic on `http://localhost:30000`.