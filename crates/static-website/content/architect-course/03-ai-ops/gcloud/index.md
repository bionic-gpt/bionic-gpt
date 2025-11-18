# Prerequisites

* Download and install [Google Cloud SDK](https://cloud.google.com/sdk/docs/install). Note that if you install `gcloud` using a package manager (as opposed to downloading and installing it manually), some of the commands will not be supported.

* Install kubectl command line tool by running the following command:

    ```sh
    gcloud components install kubectl
    ```

## Create a GKE cluster

Each cluster brings up three nodes each of the type `n1-standard-1` for the Kubernetes masters. You can directly create a cluster with the desired machine type using the --machine-type option. In the following example, you are going to create a node-pool with `n1-standard-8` type nodes for the Bionic install.

* Choose the zone in which you want to run the cluster. In this example, you are going to deploy the Kubernetes masters using the default machine type `n1-standard-1` in the zone `us-west1-a`, and add a node pool with the desired node type and node count in order to deploy the Bionic universe. You can view the list of zones by running the following command:

    ```sh
    gcloud compute zones list
    ```

    ```sh
    NAME                       REGION                   STATUS
    ...
    us-west1-b                 us-west1                 UP
    us-west1-c                 us-west1                 UP
    us-west1-a                 us-west1                 UP
    ...
    ```

* Create a Kubernetes cluster on GKE by running the following in order to create a cluster in the desired zone:

    ```sh
    gcloud container clusters create bionic --zone us-west1-b
    ```

## Setup your access

Navigate to the IAM & Admin Page:
    Go to the IAM & Admin page in the Google Cloud Console.

Select the Project:
    Make sure you have selected the correct project where your GKE cluster resides.

Find the User:
    Find the user you want to grant permissions to. If the user is not already listed, click on "Add" at the top of the page to add them.

Edit Roles:
    Click on the pencil icon next to the user's name to edit their roles.
    Add the "Kubernetes Engine Admin" role or a custom role that includes the necessary permissions (container.roles.update, etc.).

Save Changes:
    Click "Save" to apply the changes.

## Install Bionic

Follow the "Install Bionic" guide to continue the installation.

## Delete the Cluster

```sh
gcloud container clusters delete bionic --zone us-west1-b
```