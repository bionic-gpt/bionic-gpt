+++
title = "Quick Installation (Linux/OSX)"
weight = 20
sort_by = "weight"
+++

## The Bionic Installer

The bionic installer is a bash script that simplifies a lot of the setup. To install it run the following.

```sh
curl -LO https://raw.githubusercontent.com/bionic-gpt/bionic-gpt/main/bionic.sh && chmod +x ./bionic.sh && sudo mv bionic.sh /usr/bin/bionic
```

And run

```sh
bionic reqs
```

This will show you which required dependencies need to be installed.

## Run the Install (Linux)

The following will install `k3s` as our kubernetes engine and then install bionic into the cluster.

```sh
bionic install --k3s
```

## Run the Install (OSX)

For OSX you'll need to install docker desktop and enable a Kubernetes cluster.

```sh
bionic install
```

## The Finished Result

After a while of container creation you should see all the pods running and then be able to access Bionic.


![Alt text](../bionic-startup-k9s.png "Bionic K9s")

## Run the User Interface

You can then access the front end from `http://localhost:7800` and you'll be redirected to a registration screen.

## Registration

The first user to register with **BionicGPT** will become the system administrator. The information is kept local to your machine and your data is not sent anywhere.

![Alt text](../initial-screen.png "Start Screen")
