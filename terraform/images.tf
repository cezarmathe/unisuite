# unisuite - images

data "docker_registry_image" "syslog" {
  name = "balabit/syslog-ng:latest"
}

resource "docker_image" "syslog" {
  name          = data.docker_registry_image.syslog.name
  pull_triggers = [data.docker_registry_image.syslog.sha256_digest]
}
