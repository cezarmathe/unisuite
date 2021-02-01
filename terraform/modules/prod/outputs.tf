# prod - outputs

output "usscraper_image_id" {
  value = var.use_module ? docker_image.usscraper[0].latest : ""
}

output "watchman_image_id" {
  value = var.use_module ? docker_image.watchman[0].latest : ""
}

output "asbot_image_id" {
  value = var.use_module ? docker_image.asbot[0].latest : ""
}

output "usdiff_image_id" {
  value = var.use_module ? docker_image.usdiff[0].latest : ""
}
