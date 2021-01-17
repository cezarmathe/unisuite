#!/usr/bin/env bash

# moodle.sh - library for making api calls to moodle. build for usscraper.

# This library expects the following environment variables:
# - MOODLE_BASEURL   - base URL of the Moodle instance
# - MOODLE_API_TOKEN - API token to use for Moodle API calls.
# Additionally, to be able to get an API token, the following environment variables are required:
# - MOODLE_USERNAME     - username of the Moodle account to use
# - MOODLE_PASSWORD     - password of the Moodle account used for making API calls
# - MOODLE_SERVICE_NAME - service name to use then acquiring the API token.

source /usr/local/lib/usscraper.sh || exit 1

# Format the request parameters used for making Moodle API calls.
function moodle_format_req_params() {
    local params
    params="$@"; shift
    if [[ -z "${params}" ]]; then
        printf "%s" ""
        return
    fi

    for param in ${params}; do
        printf "%s" " -d ${param}"
    done
}

# Log into Moodle.
function moodle_login() {
    # Do not attempt to log in without the necessary environment variables.
    if [[ -z "${MOODLE_USERNAME}" ]]; then
        log_put "${LOG_ERROR}" "Environment variable is missing." "moodle.sh" "moodle_login" "MOODLE_USERNAME"
        printf "%s" ""
        return
    fi
    if [[ -z "${MOODLE_PASSWORD}" ]]; then
        log_put "${LOG_ERROR}" "Environment variable is missing." "moodle.sh" "moodle_login" "MOODLE_PASSWORD"
        printf "%s" ""
        return
    fi
    if [[ -z "${MOODLE_SERVICE_NAME}" ]]; then
        log_put "${LOG_ERROR}" "Environment variable is missing." "moodle.sh" "moodle_login" "MOODLE_SERVICE_NAME"
        printf "%s" ""
        return
    fi

    log_put "${LOG_DEBUG}" "Environment variables ok." "moodle.sh" "moodle_login"

    local req_res
    req_res="$(curl -s -X POST \
        -A "github.com/cezarmathe/unisuite" \
        -d username="${MOODLE_USERNAME}" \
        -d password="${MOODLE_PASSWORD}" \
        "${MOODLE_BASEURL}/login/token.php?service=${MOODLE_SERVICE_NAME}")"

    # Check if curl exited successfully.
    local exitcode="$?"
    if [[ "${exitcode}" == "0" ]]; then
        log_put "${LOG_DEBUG}" "API call ok." "moodle.sh" "moodle_login"
    else
        log_put "${LOG_DEBUG}" "API call not ok." "moodle.sh" "moodle_login" "exitcode=${exitcode}"
    fi

    printf "%s" "$(printf "%s" "${req_res}" | jq -M -r .token)"
}

# Make an API call to Moodle.
function moodle_do() {
    local function_name
    function_name="${1}"; shift
    local params
    IFS=" " params="$*"; shift

    log_put "${LOG_DEBUG}" "${function_name}, params: ${params:--}" "moodle.sh" "moodle_do"

    local req_res
    req_res="$(curl -s -X POST \
        -A "github.com/cezarmathe/unisuite" \
        -d wstoken="${MOODLE_API_TOKEN}" \
        -d wsfunction="${function_name}" \
        $(eval moodle_format_req_params "${params}") \
        "${MOODLE_BASEURL}/webservice/rest/server.php?moodlewsrestformat=json")"

    # Check if curl exited successfully.
    local exitcode="$?"
    if [[ "${exitcode}" == "0" ]]; then
        log_put "${LOG_DEBUG}" "API call ok." "moodle.sh" "moodle_do"
    else
        log_put "${LOG_DEBUG}" "API call not ok."  "moodle.sh" "moodle_do" "exitcode=${exitcode}"
    fi

    printf "%s" "${req_res}"
}
