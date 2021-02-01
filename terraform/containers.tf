# unisuite - containers

resource "docker_container" "syslog" {
  name  = "syslog"
  image = docker_image.syslog.latest

  capabilities {
    add  = ["NET_BIND_SERVICE"]
  }

  networks_advanced {
    name = docker_network.syslog.name
  }

  upload {
    file = "/etc/syslog-ng/syslog-ng.conf"
    content = file("${path.module}/syslog-ng.conf")
  }

  # data volume
  volumes {
    volume_name    = docker_volume.syslog_data.name
    container_path = "/var/log/unisuite"
  }

  must_run = true
  restart  = "unless-stopped"
  start    = true
}

resource "docker_container" "usscraper" {
  name  = "usscraper"
  image = local.usscraper_image

  env = [
    "MOODLE_BASEURL=${var.moodle_baseurl}",
    "MOODLE_USERNAME=${var.moodle_username}",
    "MOODLE_PASSWORD=${var.moodle_password}",
    "MOODLE_SERVICE_NAME=${var.moodle_service_name}",
    "LOG_LEVEL=${var.usscraper_log_level != "" ? var.usscraper_log_level : var.log_level}",
  ]

  # upload rules
  dynamic "upload" {
    for_each = var.usscraper_rules

    content {
      file    = "/var/usscraper/rules/${upload.value}.rule"
      content = file("${path.module}/../usscraper/rules/${upload.value}.rule")
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

resource "docker_container" "watchman" {
  name  = "watchman"
  image = local.watchman_image

  env = [
    "WATCHMAN_ASBOT_ADDRESS=http://${docker_container.asbot.network_data[0].ip_address}:${random_integer.asbot_grpc_port.result}",
    "WATCHMAN_RULES=${join(",", var.usscraper_rules)}",
    "SYSLOG=${docker_container.syslog.network_data[0].ip_address}:514",
    "LOG_LEVEL=${var.watchman_log_level != "" ? var.watchman_log_level : var.log_level}",
  ]

  networks_advanced {
    name = docker_network.syslog.name
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

resource "docker_container" "asbot" {
  name  = "asbot"
  image = local.asbot_image

  env = [
    "SYSLOG=${docker_container.syslog.network_data[0].ip_address}:514",
    "LOG_LEVEL=${var.asbot_log_level != "" ? var.asbot_log_level : var.log_level}",
    "ASBOT_ADDRESS=http://0.0.0.0:${random_integer.asbot_grpc_port.result}",
    "ASBOT_DISCORD_TOKEN=${var.asbot_discord_token}",
    "ASBOT_DISCORD_MOODLE_WEBHOOK_ID=${var.asbot_mevents_webhook_id}",
    "ASBOT_DISCORD_MOODLE_WEBHOOK_TOKEN=${var.asbot_mevents_webhook_token}",
  ]

  networks_advanced {
    name = docker_network.syslog.name
  }

  must_run = true
  restart  = "unless-stopped"
  start    = true
}

resource "docker_container" "usdiff" {
  name  = "usdiff"
  image = local.usdiff_image

  env = [
    "SYSLOG=${docker_container.syslog.network_data[0].ip_address}:514",
    "LOG_LEVEL=${var.usdiff_log_level != "" ? var.usdiff_log_level : var.log_level}",
    "USDIFF_ADDRESS=http://0.0.0.0:${random_integer.usdiff_grpc_port.result}",
  ]

  networks_advanced {
    name = docker_network.syslog.name
  }

  must_run = true
  restart  = "unless-stopped"
  start    = true
}
