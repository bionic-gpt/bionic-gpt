# Glossary


### Chain of Thought Prompting
A technique in prompt engineering where the model is encouraged to think step-by-step, breaking down the reasoning process to arrive at an answer, which can improve the accuracy of complex tasks.

*Example: In a math problem, the model might break down the steps needed to solve an equation, explaining each step before arriving at the final answer.*

### Context Size
The maximum amount of text, including user input and the model's response, that a language model can process in a single interaction. It determines how much of the conversation history the model can use to generate a relevant and coherent response.

*Example: If a model has a context size of 2,000 tokens, it can consider the last 2,000 tokens of the conversation, which might include recent user inputs and the model's previous responses.*

### Embedding
A numerical representation of words, phrases, or other data in a lower-dimensional space, enabling the model to perform tasks like similarity searches, clustering, or classification more effectively by capturing the semantic meaning of the text.

*Example: The word "king" might be represented as a vector in a multi-dimensional space close to words like "queen" or "royalty," reflecting their semantic relationships.*

### Foundation Models
Large-scale models trained on diverse datasets that can be fine-tuned for a variety of specific tasks. They serve as the base for many applications in AI, including natural language processing and computer vision.

*Example: OpenAI's GPT-4 or Google's BERT, which can be adapted for tasks like text generation, translation, or sentiment analysis.*

### Generative AI
A type of artificial intelligence that can generate new content, such as text, images, or music, based on patterns learned from existing data.

*Example: DALL-E generates images from textual descriptions, and GPT-3 generates text based on a prompt.*

### Grounding
The process of ensuring that an AI model’s outputs are based on factual and relevant data, often by linking responses to external knowledge sources or datasets.

*Example: A chatbot grounded in a company’s internal documents will refer to specific policies and procedures when answering employee questions.*

### Generative Pre-trained Transformer (GPT)
A type of large language model that uses the Transformer architecture and is pre-trained on large datasets. It can generate coherent text based on a given prompt and is widely used in various NLP tasks.

*Example: GPT-4, which can generate text responses in a conversational setting or produce creative writing based on prompts.*

### Hallucination
When a language model generates text or information that appears plausible but is actually false or fabricated, often due to limitations in the model’s training data or understanding.

*Example: A model might confidently state incorrect historical facts or create fictional events when asked about a less-known topic.*

### Inference
The process by which a machine learning model makes predictions or generates outputs based on new input data. It refers to the application of a trained model to perform tasks like classification, generation, or decision-making.

*Example: An image recognition model infers that a given image contains a cat based on its learned patterns from training data.*

### Inference Engine
The component or system that executes the inference process, applying trained models to new data to generate predictions, classifications, or other outputs.

*Example: TensorFlow Serving is an inference engine that serves trained models and handles requests for predictions.*

### Large Language Model (LLM)
A type of AI model designed to understand and generate human language. These models are trained on vast amounts of text data and can perform a wide range of natural language processing tasks.

*Example: GPT-4 and BERT are examples of large language models used in various applications from chatbots to translation services.*

### Machine Learning
A branch of artificial intelligence focused on developing algorithms that allow computers to learn from and make predictions or decisions based on data. It encompasses various techniques, including supervised, unsupervised, and reinforcement learning.

*Example: A spam filter that learns to classify emails as spam or not spam based on past examples.*

### Prompt Engineering
The practice of designing and optimising prompts given to language models to elicit the most accurate and relevant responses. It involves crafting specific input text that guides the model to produce the desired output.

*Example: A prompt like "Write a short story about a dragon" guides the model to generate a creative narrative focused on that theme.*

### Reinforcement Learning
A type of machine learning where an agent learns to make decisions by taking actions in an environment to maximise cumulative rewards. It is often used to improve models' performance over time.

*Example: AlphaGo, which uses reinforcement learning to improve its gameplay and eventually became capable of defeating human champions in Go.*

### Retrieval-Augmented Generation (RAG)
A technique that combines retrieval of relevant information from a dataset with generative capabilities to produce responses. RAG models retrieve documents or data chunks to ground the generative process, enhancing the accuracy and relevance of the output.

*Example: A RAG model might retrieve relevant medical research papers and then generate a summary or answer based on the retrieved data.*

### Sentiment Analysis
The process of analysing text to determine the sentiment expressed, such as positive, negative, or neutral. It’s commonly used in social media monitoring, customer feedback analysis, and other applications.

*Example: Analysing tweets to determine public sentiment towards a new product launch.*

### System Prompt
A predefined instruction given to a language model that guides its behaviour or sets the tone for how it should respond to user inputs. It defines the context or role the model should assume during the interaction.

*Example: A system prompt might instruct a model to behave as a helpful customer service agent when interacting with users.*


### Token
A token is a unit of text used by language models in processing and generating language. It can be as short as a single character or as long as an entire word or phrase, depending on the language model's tokenisation process. Tokens are the basic building blocks that the model uses to understand and generate responses, with the total number of tokens in a prompt or response contributing to the context size.

*Example: In the sentence "OpenAI creates advanced AI models," the words "OpenAI," "creates," "advanced," "AI," and "models" might each be considered separate tokens. However, in some cases, "AI" could be split into two tokens, depending on the tokenisation method.*


### Transformer
A neural network architecture designed to handle sequential data, such as text, by relying on self-attention mechanisms to capture relationships between words in a sentence. Transformers are the foundation of many modern NLP models, including GPT.

*Example: The Transformer architecture is the basis for models like GPT, BERT, and T5, enabling them to understand context and generate coherent text.*

### Transparency
In AI, transparency refers to the clarity and openness with which the functioning, decision-making processes, and limitations of a model are communicated. It ensures users understand how and why a model produces certain outputs.

*Example: Providing users with explanations for how an AI model arrived at a particular decision or output, such as why a loan application was approved or denied.*

### Unsupervised Learning
A type of machine learning where the model learns patterns and relationships in data without being provided labelled examples. The model tries to identify structure in the data on its own, often used for clustering and anomaly detection.

*Example: Clustering customers into different segments based on purchasing behaviour without pre-labeled categories.*

### User Prompt
The input or query provided by a user to a language model, which the model then processes to generate a response. The prompt guides the model's output and can include specific instructions or questions.

*Example: A user asking "What is the capital of France?" serves as the prompt for the model to generate the response "Paris."*

### Zero-Shot Learning
A capability of some models to perform tasks without having been explicitly trained on those tasks. Instead, the model uses its generalised knowledge to infer the correct behaviour or response.

*Example: A model that correctly classifies images of animals it has never seen before based on its understanding of general features from other training data.*

