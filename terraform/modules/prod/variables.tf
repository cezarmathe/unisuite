# prod - variables

variable "docker_host" {
  type        = string
  description = "Docker host for deployment."
  default     = "unix:///var/run/docker.sock"
}

variable "use_module" {
  type        = bool
  description = "Flag that specifies if this module should take care of the Docker images."
  default     = false
}

variable "usscraper_image_version" {
  type        = string
  description = "Scraper version to use for deployment."
}
