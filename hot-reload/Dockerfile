# Use the Debian 12-slim base image
FROM debian:12-slim

# Set environment variables to avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install necessary packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends inotify-tools && \
    rm -rf /var/lib/apt/lists/*

# Create necessary directories
RUN mkdir -p /app /var/log

# Set working directory
WORKDIR /app

# Copy your executable into the container
# Replace 'your_executable' with the actual executable file name
COPY target/debug/web-server /app/web-server
COPY crates/web-assets/dist /workspace/crates/web-assets/dist/
COPY crates/web-assets/images /workspace/crates/web-assets/images

# Ensure the executable has execution permissions
RUN chmod +x /app/web-server

# Copy the update script into the container
COPY hot-reload/update_exec.sh /usr/local/bin/update_exec.sh

# Ensure the script has execution permissions
RUN chmod +x /usr/local/bin/update_exec.sh

# Expose any necessary ports (modify as needed)
# For example, if your executable runs a server on port 8080:
# EXPOSE 8080

# Set the update script as the entry point
ENTRYPOINT ["/usr/local/bin/update_exec.sh"]