#!/usr/bin/env bash

set -e

ARTIFACTS=(
    usscraper
    watchman
)

function artifacts_dev() {
    for artifact in ${ARTIFACTS[*]}; do
        cd "${artifact}"
        make save-image VERSION=dev
        cd ..
    done
    chmod 644 ./shared/*.tar

    vagrant ssh --command "/home/vagrant/.bin/artifacts import"

    echo "# main_override" > terraform/modules/dev/main_override.tf
    echo "locals {" >> terraform/modules/dev/main_override.tf
    for image in ${ARTIFACTS[*]}; do
        echo "  ${image}_image_id = \"$(vagrant ssh --command "docker inspect --format '{{.Id}}' cezarmathe/${image}:dev | tr -d '\n' ")\"" >> terraform/modules/dev/main_override.tf
    done
    echo "}" >> terraform/modules/dev/main_override.tf
    return
}

function artifacts_import() {
    for artifact in ${ARTIFACTS[*]}; do
        docker image load --input "/mnt/${artifact}-dev.tar"
    done
    docker image prune --force
}

function artifacts_prod() {
    for artifact in ${ARTIFACTS[*]}; do
        cd "${artifact}"
        make push
        cd ..
    done
}

function main() {
    local environment
    environment="$1"; shift

    case "${environment}" in
    "dev")
        artifacts_dev
        ;;
    "prod")
        artifacts_prod
        ;;
    "import")
        artifacts_import
        ;;
    *)
        printf "%s\n" "No such deploymeny environment."
        exit 1
    esac
}

main $@
