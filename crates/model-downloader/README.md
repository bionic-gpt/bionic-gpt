## Download Models From Hugging Face

The text inference mebeddings docker image will always try to download models if they are not present. This causes issues on some sites and in OpenShift.

Here we download all the artifacts needed and the in the CI/CD pipeline we package thme into our own container.