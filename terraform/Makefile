# unisuite deployment makefile

# Deployment environment
# dev / prod
ENVIRONMENT = dev

default: deploy-auto

deploy:
	terraform workspace select $(ENVIRONMENT)
	terraform apply \
		-var-file=env/$(ENVIRONMENT).tfvars
.PHONY: deploy

deploy-auto:
	terraform workspace select $(ENVIRONMENT)
	terraform apply \
		-var-file=env/$(ENVIRONMENT).tfvars \
		--auto-approve
.PHONY: deploy-auto

destroy:
	terraform workspace select $(ENVIRONMENT)
	terraform destroy -var-file=env/$(ENVIRONMENT).tfvars
.PHONY: destroy

init:
	terraform init
	terraform workspace new dev
	terraform workspace new prod
	cp env/vars.tfvars env/dev.tfvars
	cp env/vars.tfvars env/prod.tfvars
.PHONY: init
