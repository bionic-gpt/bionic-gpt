# Visualising RAG Data

You will need to setup Jupyter Notebook to run the code below. See [Jupyter Notebook setup](/docs/guides/jupyter)

The example code below assumes you already have your documents loaded into the bionicGPT database.
We will create a scatter graph showing the document chunks of data together with the 4 most 'relevant' chunks based on the query you specified.
At the end we should have a diagram like this.
This can be a useful technique for data scientists to debug RAG settings.

![Alt text](vis-rag-scatter-example.png "Scatter example")



### Setup and Retrieve Query Embeddings ###

```sh
!pip install -q sqlalchemy psycopg2-binary pandas
!pip install -q matplotlib seaborn scikit-learn
```

```python
import sqlalchemy
import pandas as pd
import requests
from sqlalchemy import text
import json
import numpy as np
from sklearn.manifold import TSNE
from sklearn.cluster import KMeans
import seaborn as sns
import matplotlib.pyplot as plt
```

```python
url = "http://embeddings-api:80/embeddings"

data = {"input": "what is tokenising","model" : "bge-small-en-v1.5"}
headers = {"Content-Type": "application/json"}

response = requests.post(url, json=data, headers=headers)
```

Get the database login details using

`kubectl get secret -n bionic-gpt database-urls -o jsonpath='{.data.readonly-url}' | base64 --decode`

```python
engine = sqlalchemy.create_engine('postgresql://bionic_readonly:6323714616921@bionic-db-cluster-rw:5432/bionic-gpt')
conn = engine.connect()
```
![Alt text](vis-rag-1.png "Setup")


### Retrieve Document Data from Database ###

We will now use the query embeddings from above to retrieve all document chunks from the data ordered by 'similarity'

```python
query_embedding = response.json()['data'][0]['embedding']


# query_embedding = response.text # From above embedding call

sql = text(f"""SELECT  document_id, file_name, text, embeddings  FROM  chunks, documents
where documents.id = document_id
and embeddings is not null
ORDER BY embeddings <-> '{query_embedding}'""")

df = pd.read_sql(sql,conn)
df
```

![Alt text](vis-rag-ret-chunks.png "Retrieve chunks")


### Convert Data Retrieved into 2 Dimensional Data ###

```python
df['embeddings_vec'] = df['embeddings'].apply(lambda x: [float(y) for y in json.loads(x)])
embeddings_list = df['embeddings_vec'].tolist()

df
```

```python
# Convert chunk embeddings
x = np.array(df['embeddings_vec'].to_list(), dtype=np.float32)

# Reduce emeddings dimensions to 2 for plotting purposes
tsne = TSNE(random_state=0, n_iter=1000)
tsne_results = tsne.fit_transform(x)


df_tsne = pd.DataFrame(tsne_results, columns=['x','y'])
df_tsne['doc'] = df['file_name']
df_tsne

```

![Alt text](vis-rag-convert.png "Data Conversion")


### Plot Results ###

Different colours refer to the different documents uploaded.
The 4 circles in blue highlight the 4 most 'relevant' chunks based on the query used above.

```python
fig, ax = plt.subplots(figsize=(10,8)) # Set figsize
sns.set_style('darkgrid', {"grid.color": ".6", "grid.linestyle": ":"})
sns.scatterplot(data=df_tsne, x='x', y='y', hue='doc', palette='hls')

# Highlight the most 'relevant' chunks for query provided
sns.scatterplot(data=df_tsne.head(5), x='x', y='y', marker="$\circ$", ec="face", hue='doc', s=30)

sns.move_legend(ax, "upper left", bbox_to_anchor=(1, 1))
plt.title('Scatter plot of document chunks in bionic database');
plt.xlabel('x');
plt.ylabel('y');
plt.axis('equal')
```

![Alt text](vis-rag-plot.png "Data Conversion")