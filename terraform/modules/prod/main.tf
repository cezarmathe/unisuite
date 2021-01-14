# prod

terraform {
  required_providers {
    docker = {
      source = "kreuzwerker/docker"
      version = "2.9.0"
    }
  }
  required_version = ">= 0.14"
}

provider "docker" {
  host = var.docker_host
}

data "docker_registry_image" "usscraper" {
  count = var.use_module ? 1 : 0
  name = "cezarmathe/usscraper:${var.usscraper_image_version}"
}

resource "docker_image" "usscraper" {
  count         = var.use_module ? 1 : 0
  name          = data.docker_registry_image.usscraper[0].name
  pull_triggers = [data.docker_registry_image.usscraper[0].sha256_digest]
}

data "docker_registry_image" "watchman" {
  count = var.use_module ? 1 : 0
  name  = "cezarmathe/watchman:${var.watchman_image_version}"
}

resource "docker_image" "watchman" {
  count         = var.use_module ? 1 : 0
  name          = data.docker_registry_image.watchman[0].name
  pull_triggers = [data.docker_registry_image.watchman[0].sha256_digest]
}

data "docker_registry_image" "asbot" {
  count = var.use_module ? 1 : 0
  name  = "cezarmathe/asbot:${var.asbot_image_version}"
}

resource "docker_image" "asbot" {
  count         = var.use_module ? 1 : 0
  name          = data.docker_registry_image.asbot[0].name
  pull_triggers = [data.docker_registry_image.asbot[0].sha256_digest]
}
