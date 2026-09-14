FROM python:3.13-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       libreoffice-calc libreoffice-impress libreoffice-writer \
       fonts-liberation \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY server ./server
COPY openapi ./openapi
COPY archipelago ./archipelago
COPY LICENSE ./
ENV ARCHIPELAGO_ROOT=/app/archipelago
EXPOSE 8080
CMD ["uvicorn", "server.main:app", "--host", "0.0.0.0", "--port", "8080"]
