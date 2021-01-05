# unisuite - containers

resource "docker_container" "usscraper" {
  name  = "usscraper"
  image = local.usscraper_image

  env = [
    "MOODLE_BASEURL=${var.moodle_baseurl}",
    "MOODLE_USERNAME=${var.moodle_username}",
    "MOODLE_PASSWORD=${var.moodle_password}",
    "MOODLE_SERVICE_NAME=${var.moodle_service_name}",
    "LOG_LEVEL=${var.usscraper_log_level}"
  ]

  # upload rules
  dynamic "upload" {
    for_each = var.usscraper_rules

    content {
      file    = "/var/usscraper/rules/${upload.value}.rule"
      content = file("${path.module}/../scraper/rules/${upload.value}.rule")
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
