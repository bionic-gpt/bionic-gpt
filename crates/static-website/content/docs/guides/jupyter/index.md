# Deploying Jupyter Notebook

Your data scientists can connect to bionicGPT directly from their existing Jupyter Notebook environments using our API, providing full access to any models and datasets they are authorised to use.

For a more integrated experience that includes access to the entire system, including the underlying database, you can deploy Jupyter Notebook within the same Kubernetes cluster as bionicGPT.


create a file called `jupyter-notebook.yaml` with the following contents

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: jupyter-notebook
  namespace: bionic-gpt
spec:
  replicas: 1
  selector:
    matchLabels:
      app: jupyter-notebook
  template:
    metadata:
      labels:
        app: jupyter-notebook
    spec:
      containers:
      - name: jupyter-notebook
        image: jupyter/scipy-notebook:latest
        ports:
        - containerPort: 8888
        env:
        - name: JUPYTER_NOTEBOOK_PASSWORD
          value: "mypassword"
        - name: JUPYTER_NOTEBOOK_PORT
          value: "8888"
        volumeMounts:
        - name: notebook-volume
          mountPath: /home/jovyan/work
      volumes:
      - name: notebook-volumeGetting the Login URL
In the logs you should see something like the below

http://127.0.0.1:8888/lab?token=b2eb9f7b5e0fafaef985b734e6fc4ad8f0e58c529c15f73e
        persistentVolumeClaim:
          claimName: notebook-pvc

---
apiVersion: v1
kind: Service
metadata:
  name: jupyter-notebook
  namespace: bionic-gpt
spec:
  selector:
    app: jupyter-notebook
  ports:
  - name: http
    port: 8888
    targetPort: 8888
  type: LoadBalancer

---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: notebook-pvc
  namespace: bionic-gpt
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
```


Run this yaml file into our cluster

**Deploy**
`kubectl apply -f jupyter-notebook.yaml`

**Undeploy**
`kubectl delete -f jupyter-notebook.yaml`


**Getting the Login URL**
In the logs you should see something like the below

http://127.0.0.1:8888/lab?token=b2eb9f7b5e0fafaef985b734e6fc4ad8f0e58c529c15f73e


![Alt text](jupyter.png "Jupyter Notebook Window")





### Accessing the Database

Get the database login details using

`kubectl get secret -n bionic-gpt database-urls -o jsonpath='{.data.readonly-url}' | base64 --decode`


```
!pip install -q sqlalchemy psycopg2-binary pandas

import sqlalchemy
engine = sqlalchemy.create_engine('postgresql://bionic_readonly:6323714616921@bionic-db-cluster-rw:5432/bionic-gpt')

conn = engine.connect()

import pandas as pd
pd.read_sql("SELECT * FROM chunks",conn)


```



![Alt text](jupyter-db.png "Jupyter database access Window")


