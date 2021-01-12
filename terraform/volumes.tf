# unisuite - volumes

resource "docker_volume" "usscraper_default" {
  name        = "usscraper_default"
  driver      = "local-persist"
  driver_opts = {
    mountpoint = var.usscraper_default_mountpoint
  }
}

resource "docker_volume" "usscraper_data" {
  name        = "usscraper_data"
  driver      = "local-persist"
  driver_opts = {
    mountpoint = var.usscraper_data_mountpoint
  }
}

resource "docker_volume" "syslog_data" {
  name        = "syslog_data"
  driver      = "local-persist"
  driver_opts = {
    mountpoint = var.syslog_data_mountpoint
  }
}
