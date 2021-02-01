# dev - outputs

output "usscraper_image_id" {
  value = var.use_module ? local.usscraper_image_id : ""
}

output "watchman_image_id" {
  value = var.use_module ? local.watchman_image_id : ""
}

output "asbot_image_id" {
  value = var.use_module ? local.asbot_image_id : ""
}

output "usdiff_image_id" {
  value = var.use_module ? local.usdiff_image_id : ""
}
