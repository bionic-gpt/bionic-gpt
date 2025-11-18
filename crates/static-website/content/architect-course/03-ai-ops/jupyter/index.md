# Administer the Database with pgAdmin

We can extend our `docker-compose` to include a [Jupyter Notebook](https://jupyter.org/).

Store the following in `docker-compose-jupyter.yml`

```yml
services:

  jupyter:
    image: jupyter/minimal-notebook
    ports:
      - 8888:8888
```

Then run

```sh
docker-compose -f docker-compose.yml -f docker-compose-jupyter.yml up
```

## Getting the Login URL

In the logs you should see something like the below

`http://127.0.0.1:8888/lab?token=b2eb9f7b5e0fafaef985b734e6fc4ad8f0e58c529c15f73e`

This is the authentication token. You'll need to use that URL to Login.

## Accessing the various APIs

Example accessing the embeddings API.  open up a notebook terminal and run

```sh
curl embeddings-api:80/embed     -X POST     -d '{"inputs":"What is Deep Learning?"}'     -H 'Content-Type: application/json'
```


![Alt text](jupyter-notebook.png "Jupyter Notebook")


## Accessing the Database ##

```sh
!pip install -q sqlalchemy psycopg2-binary pandas  
engine = sqlalchemy.create_engine('postgresql://postgres:testpassword@postgres:5432/bionic-gpt')
conn = engine.connect()
import sqlalchemy
import pandas as pd

pd.read_sql("SELECT * FROM information_schema.tables",conn)
```

![Alt text](jupyter-database.png "Connect to Database")


## Accessing Embeddings From Python ##

```python
url = "http://embeddings-api:80/embed"

data = {"inputs": "is US20040098780A1 in the dataset?"}
headers = {"Content-Type": "application/json"}

response = requests.post(url, json=data, headers=headers)

print(response.text[1:-1])

```

![Alt text](jupyter-embedding.png "Embedding Calls")


## Embedding Based Query on Database ##

```python
from sqlalchemy import text
em = response.text

sql = text(f"""SELECT  document_id, file_name, text, embeddings  FROM  chunks, documents 
where documents.id = document_id 

ORDER BY embeddings <-> '{em[1:-1]}'""")
# print(sql)
df = pd.read_sql(sql,conn)

df
```

![Alt text](jupyter-embedding-query.png "Database Embedding Calls")