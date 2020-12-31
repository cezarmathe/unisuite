# unisuite

terraform {
  required_version = ">= 0.14"
}

module "scraper" {
  source = "./scraper"

  # Use a main_override.tf.
  image_version       = ""
  default_mountpoint  = ""
  data_mountpoint     = ""
  moodle_baseurl      = ""
  moodle_username     = ""
  moodle_password     = ""
  moodle_service_name = ""
  rules               = []
}
