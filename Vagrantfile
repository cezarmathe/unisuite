# -*- mode: ruby -*-
# vi: set ft=ruby :

mirror_hostname = "ftp.halifax.rwth-aachen.de"
alpine_version  = ["3", "12"]

Vagrant.configure("2") do |config|
  config.vm.box = "generic/alpine#{alpine_version.join("")}"

  config.vm.synced_folder "./shared/artifacts", "/mnt", create: true

  config.vm.provision "file", source: "./scripts/artifacts.sh", destination: "/home/vagrant/.bin/artifacts"
  config.vm.provision "shell" do |s|
    s.args   = %x{ssh-add -L}
    s.inline = <<-SHELL
      echo $@ >> /home/vagrant/.ssh/authorized_keys

      echo https://#{mirror_hostname}/alpine/v#{alpine_version.join(".")}/main > /etc/apk/repositories
      echo https://#{mirror_hostname}/alpine/v#{alpine_version.join(".")}/community >> /etc/apk/repositories
      echo https://#{mirror_hostname}/alpine/edge/testing >> /etc/apk/repositories

      apk update
      apk add "docker>18.09.8-r0" \
          "docker-openrc>18.09.8-r0" \
          docker-volume-local-persist \
          docker-volume-local-persist-openrc

      rc-update add docker
      rc-update add docker-volume-local-persist
      rc-service docker start
      rc-service docker-volume-local-persist start

      adduser vagrant docker

      mkdir -p /srv/file
      chown -R vagrant:vagrant /srv/file
    SHELL
  end
end
