# dev

terraform {
  required_providers {
    docker = {
      source = "kreuzwerker/docker"
      version = "2.9.0"
    }
  }
  required_version = ">= 0.14"
}

# These will be overriden in a main_override.tf file.
# The file will have the image IDs generated by the vm_import_dev_image.sh script.
locals {
  usscraper_image_id = ""
  watchman_image_id  = ""
  asbot_image_id     = ""
}
