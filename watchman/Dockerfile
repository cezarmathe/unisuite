FROM alpine:latest

# Add packages.
RUN apk add \
        bash \
        dumb-init \
        su-exec

# Initialize directories.
RUN mkdir -p /var/usscraper && \
        mkdir -p /var/usscraper/data

# Default watchman user and group ids.
ARG UID=1000
ARG GID=1000

# Create watchman user.
RUN addgroup --gid ${GID} -S watchman && adduser --uid ${UID} -h /var/watchman -s /bin/bash -S -G watchman watchman

# Debug / release binary
ARG ARTIFACT=debug

# Copy binaries.
COPY ./watchman /usr/local/bin/watchman

# Expose data volume.
VOLUME /var/usscraper/data

ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["su-exec", "watchman:watchman", "/usr/local/bin/watchman"]
