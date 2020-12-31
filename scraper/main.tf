# unisuite

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
  name = "cezarmathe/usscraper:${var.image_version}"
}

resource "docker_image" "usscraper" {
  name          = data.docker_registry_image.usscraper.name
  pull_triggers = [data.docker_registry_image.usscraper.sha256_digest]
}

resource "docker_volume" "usscraper_default" {
  name        = "usscraper_default"
  driver      = "local-persist"
  driver_opts = {
    mountpoint = var.default_mountpoint
  }
}

resource "docker_volume" "usscraper_data" {
  name        = "usscraper_data"
  driver      = "local-persist"
  driver_opts = {
    mountpoint = var.data_mountpoint
  }
}

resource "docker_container" "usscraper" {
  name  = "usscraper"
  image = docker_image.usscraper.latest

  env = [
    "MOODLE_BASEURL=${var.moodle_baseurl}",
    "MOODLE_USERNAME=${var.moodle_username}",
    "MOODLE_PASSWORD=${var.moodle_password}",
    "MOODLE_SERVICE_NAME=${var.moodle_service_name}",
    "LOG_LEVEL=${var.log_level}"
  ]

  # upload rules
  dynamic "upload" {
    for_each = var.rules

    content {
      file    = "/var/usscraper/rules/${upload.value}.rule"
      content = file("${path.module}/rules/${upload.value}.rule")
    }
  }

  # default volume
  volumes {
    volume_name    = docker_volume.usscraper_default.name
    container_path = "/var/usscraper"
  }
  # data volume
  volumes {
    volume_name    = docker_volume.usscraper_data.name
    container_path = "/var/usscraper/data"
  }

  must_run = true
  restart  = "unless-stopped"
  start    = true
}
