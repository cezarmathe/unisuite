#!/usr/bin/env bash

# usscraper.sh - general library for usscraper.

USSCRAPER_VERSION="0.1.0"; export USSCRAPER_VERSION

# Logging levels.
LOG_DEBUG="7"; export LOG_DEBUG
LOG_INFO="6"; export LOG_INFO
LOG_NOTICE="5"; export LOG_NOTICE
LOG_WARN="4"; export LOG_WARN
LOG_ERROR="3"; export LOG_ERROR
LOG_CRITICAL="2"; export LOG_CRITICAL

# Write a log.
# Syntax: log_put <log_level> <message> [<strucured_data..>]
function log_put() {
    local level
    level="${1}"; shift

    # If the log level is greater that the standard one, just return.
    if [[ "${level}" -gt "${LOG_LEVEL}" ]]; then
        return
    fi

    # Get the message
    local msg
    msg="${1}"; shift

    # Get structured data items.
    local structured_data=""
    for item in "$@"; do
        structured_data="${structured_data}[${item}]"
    done
    # If there are no items, output an '-'
    if [[ -z "${structured_data}" ]]; then
        structured_data="-"
    fi

    # If the log file is unset, output to stderr.
    if [[ -z "${LOG_FILE}" ]]; then
        LOG_FILE="/dev/stderr"
    fi

    # <${FACILITY}${SEVERITY}>${VERSION} ${TIMESTAMP} ${HOSTNAME} ${APP_NAME} ${PID} ${MSGID} ${STRUCTURED_DATA} ${MSG}
    # Facility: 16 (facility local0).
    # Version: 1.
    # Message ID: - (no message IDs will be created).
    printf "<%s%s>%s %s %s %s %s %s %s %s\n" \
        "16" \
        "${level}" \
        "1" \
        "$(date +%Y-%m-%dT%T%z)" \
        "${HOSTNAME}" \
        "${0}" \
        "$$" \
        "-" \
        "${structured_data}" \
        "${msg}" \
        | tee >> "${LOG_FILE}"
}

# Get a value with a key from a file.
function kv_get() {
    local conf_file
    conf_file="${1}"; shift
    if [[ -z "${conf_file}" ]]; then
        log_put "${LOG_WARN}" "configuration file required." "usscraper.sh" "function=kv_get"
        printf "%s" ""
        return
    fi
    if [[ ! -f "${conf_file}" ]]; then
        log_put "${LOG_WARN}" "configuration file path does not point to a file." "usscraper.sh" "function=kv_get" "path=${conf_file}"
        printf "%s" ""
        return
    fi

    local key
    key="${1}"; shift
    if [[ -z "${key}" ]]; then
        log_put "${LOG_WARN}" "key required." "usscraper.sh" "function=kv_get"
        printf "%s" ""
        return
    fi

    local line
    line="$(cat "${conf_file}" | grep ${key})"
    if [[ -z "${line}" ]]; then
        log_put "${LOG_WARN}" "key not found." "usscraper.sh" "function=kv_get" "key=${key}"
        printf "%s" ""
        return
    fi

    local value
    value="$(printf "%s" "${line}" | cut -d '=' -f 2 | cut -d ' ' -f 2-)"
    if [[ -z ${value} ]]; then
        log_put "${LOG_WARN}" "key has no value." "usscraper.sh" "function=kv_get" "key=${key}"
        printf "%s" ""
        return
    fi

    printf "%s" "${value}"
}

# Set a value for a key in a file.
function kv_set() {
    local conf_file
    conf_file="${1}"; shift
    if [[ -z "${conf_file}" ]]; then
        log_put "${LOG_WARN}" "configuration file required." "usscraper.sh" "function=kv_set"
        printf "%s" ""
        return
    fi
    if [[ ! -f "${conf_file}" ]]; then
        log_put "${LOG_WARN}" "configuration file path does not point to a file." "usscraper.sh" "function=kv_set" "path=${conf_file}"
        printf "%s" ""
        return
    fi

    local key
    key="${1}"; shift
    if [[ -z "${key}" ]]; then
        log_put "${LOG_WARN}" "key required." "usscraper.sh" "function=kv_set"
        printf "%s" ""
        return
    fi

    local value
    value="${1}"; shift
    if [[ -z "${value}" ]]; then
        log_put "${LOG_WARN}" "value required." "usscraper.sh" "function=kv_set"
        printf "%s" ""
        return
    fi

    local line
    line="$(cat "${conf_file}" | grep "${key}")"
    if [[ -z "${line}" ]]; then
        printf "%s\n" "${key} = ${value}" >> "${conf_file}"
        return
    fi

    sed -i '' -e "s/${line}/${key} = ${value}/g" "${conf_file}" > /dev/null
}

# Get a value from the configuration file.
CONFIG_GET='kv_get /etc/usscraper.conf'; export CONFIG_GET
# Get a value from the cache.
CACHE_GET='kv_get /var/usscraper/cache'; export CACHE_GET
# Set a value in the cache.
CACHE_SET='kv_set /var/usscraper/cache'; export CACHE_SET
