FROM alpine:latest

# Add packages.
RUN apk add \
        bash \
        curl \
        dumb-init \
        jq \
        perl \
        perl-utils

# Initialize directories.
RUN mkdir -p /var/usscraper && \
        mkdir -p /var/usscraper/data && \
        mkdir -p /var/usscraper/logs && \
        mkdir -p /var/usscraper/rules && \
        touch /var/usscraper/cache

# Create usscraper user.
RUN addgroup -S usscraper && adduser -h /var/usscraper -s /bin/bash -S -G usscraper usscraper

# Copy binaries.
COPY ./bin/usscraper /usr/local/bin/usscraper
COPY ./bin/usscraper_init /usr/local/bin/usscraper_init
COPY ./bin/usscraper_runner /usr/local/bin/usscraper_runner

# Copy libraries.
COPY ./lib/usscraper.sh /usr/local/lib/usscraper.sh
COPY ./lib/moodle.sh /usr/local/lib/moodle.sh

# Add usscraper crontab.
COPY ./cron/usscraper /etc/crontabs/usscraper

# Expose data volume.
VOLUME /var/usscraper

# Point out environment variables.
ENV MOODLE_BASEURL ""
ENV MOODLE_USERNAME ""
ENV MOODLE_PASSWORD ""
ENV MOODLE_SERVICE_NAME ""
ENV LOG_LEVEL ""

ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["/usr/local/bin/usscraper_init"]
