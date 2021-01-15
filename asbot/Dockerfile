FROM alpine:latest

# Add packages.
RUN apk add \
        bash \
        dumb-init \
        openssl \
        su-exec

# Default asbot user and group ids.
ARG UID=1000
ARG GID=1000

# Create asbot user.
RUN addgroup --gid ${GID} -S asbot && adduser --uid ${UID} -h /var/asbot -s /bin/bash -S -G asbot asbot

# Copy binaries.
COPY ./asbot /usr/local/bin/asbot

# Expose data volume.
VOLUME /var/usscraper/data

ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["su-exec", "asbot:asbot", "/usr/local/bin/asbot"]
