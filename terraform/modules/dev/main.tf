# dev

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

resource "docker_image" "usscraper" {
  count = var.use_module ? 1 : 0
  name  = "cezarmathe/usscraper:${var.usscraper_image_version}"
}
