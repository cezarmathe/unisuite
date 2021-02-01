# unisuite - networks

resource "docker_network" "syslog" {
  name = "syslog"
}

resource "random_integer" "asbot_grpc_port" {
  min     = 49152
  max     = 65535
}

resource "random_integer" "usdiff_grpc_port" {
  min     = 49152
  max     = 65535
}
