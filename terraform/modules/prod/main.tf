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
  name = "cezarmathe/usscraper:${var.usscraper_image_version}"
}

resource "docker_image" "usscraper" {
  count         = var.use_module ? 1 : 0
  name          = data.docker_registry_image.usscraper.name
  pull_triggers = [data.docker_registry_image.usscraper.sha256_digest]
}
