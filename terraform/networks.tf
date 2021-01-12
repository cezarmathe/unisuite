# unisuite - networks

resource "docker_network" "syslog" {
  name = "syslog"

  internal = true
}
