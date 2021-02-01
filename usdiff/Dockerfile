FROM alpine:latest

# Add packages.
RUN apk add \
        bash \
        dumb-init \
        su-exec

# Default usdiff user and group ids.
ARG UID=1000
ARG GID=1000

# Create usdiff user.
RUN addgroup --gid ${GID} -S usdiff && adduser --uid ${UID} -h /var/usdiff -s /bin/bash -S -G usdiff usdiff

# Copy binaries.
COPY ./usdiff /usr/local/bin/usdiff

# Expose data volume.
VOLUME /var/usscraper/data

ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["su-exec", "usdiff:usdiff", "/usr/local/bin/usdiff"]
