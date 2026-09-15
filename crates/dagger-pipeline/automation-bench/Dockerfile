FROM python:3.13-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY server ./server
COPY openapi ./openapi
COPY routes.json .
COPY automationbench-commit.txt .
COPY automationbench /opt/automationbench/automationbench
ENV AUTOMATIONBENCH_PATH=/opt/automationbench
EXPOSE 8080
CMD ["uvicorn", "server.app:app", "--host", "0.0.0.0", "--port", "8080"]
