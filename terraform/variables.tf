# unisuite - variables

variable "docker_host" {
  type        = string
  description = "Docker host for deployment."
  default     = "unix:///var/run/docker.sock"
}

variable "deploy_environment" {
  type        = string
  description = "Deployment environment (dev or prod)."
}

variable "usscraper_image_version" {
  type        = string
  description = "Scraper version to use for deployment."
  default     = ""
}

variable "usscraper_default_mountpoint" {
  type        = string
  description = "Host mountpoint for the default volume."
}

variable "usscraper_data_mountpoint" {
  type        = string
  description = "Host mountpoint for the data volume."
}

variable "moodle_baseurl" {
  type        = string
  description = "Moodle instance base URL."
}

variable "moodle_username" {
  type        = string
  description = "Moodle username."
}

variable "moodle_password" {
  type        = string
  description = "Moodle password."
}

variable "moodle_service_name" {
  type        = string
  description = "Moodle service name."
}

variable "usscraper_rules" {
  type        = list(string)
  description = "Scraper rules to upload."
}

variable "usscraper_log_level" {
  type        = number
  description = "Log level: from 2(CRITICAL) to 7(DEBUG). Default is 5(NOTICE)."
  default     = 5
}

variable "watchman_image_version" {
  type        = string
  description = "Watchman version to use for deployment."
  default     = ""
}

variable "syslog_data_mountpoint" {
  type        = string
  description = "Host mountpoint for the syslog data volume."
}

variable "watchman_log_level" {
  type        = string
  description = "Watchman log level."
  default     = ""
}

variable "log_level" {
  type        = string
  description = "Global log level. OFF, CRITICAL, ERROR, WARN, INFO, DEBUG, TRACE."
}

variable "asbot_image_version" {
  type        = string
  description = "Adam Smith bot version to use for deployment."
  default     = ""
}

variable "asbot_log_level" {
  type        = string
  description = "Adam Smith bot log level."
  default     = ""
}

variable "asbot_discord_token" {
  type        = string
  description = "Adam Smith Discord token."
}

variable "asbot_mevents_webhook_id" {
  type        = string
  description = "Adam Smith mevents webhook id."
}

variable "asbot_mevents_webhook_token" {
  type        = string
  description = "Adam Smith mevents webhook token."
}
