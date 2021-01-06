# dev - outputs

output "usscraper_image_id" {
  value = var.use_module ? local.usscraper_image_id : ""
}

output "watchman_image_id" {
  value = var.use_module ? local.watchman_image_id : ""
}
