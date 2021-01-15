# unisuite - networks

resource "docker_network" "syslog" {
  name = "syslog"

  internal = true
}

resource "random_integer" "asbot_grpc_port" {
  min     = 49152
  max     = 65535
}
