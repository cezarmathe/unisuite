# unisuite

terraform {
  required_providers {
    docker = {
      source = "kreuzwerker/docker"
      version = "2.9.0"
    }
  }
  required_version = ">= 0.14"
}

provider "docker" {
  host = var.docker_host
}

# Development module for using development images.
module "dev" {
  source = "./modules/dev"

  docker_host = var.docker_host
  use_module  = var.deploy_environment == "dev"

  usscraper_image_version = var.usscraper_image_version
}

# Production module for using production images.
module "prod" {
  source = "./modules/prod"

  docker_host = var.docker_host
  use_module  = var.deploy_environment == "prod"

  usscraper_image_version = var.usscraper_image_version
}

# Image IDs to use for the containers
locals {
  usscraper_image = var.deploy_environment == "dev" ? module.dev.usscraper_image_id : module.prod.usscraper_image_id
}
