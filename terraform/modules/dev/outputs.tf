# dev - outputs

output "usscraper_image_id" {
  value = var.use_module ? docker_image.usscraper[0].latest : ""
}
