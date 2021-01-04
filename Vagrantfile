# -*- mode: ruby -*-
# vi: set ft=ruby :

mirror_hostname = "ftp.halifax.rwth-aachen.de"
alpine_version  = ["3", "8"]

Vagrant.configure("2") do |config|
  config.vm.box = "generic/alpine#{alpine_version.join("")}"

  config.vm.synced_folder "./srv", "/srv"

  config.vm.provision "shell", inline: <<-SHELL
    echo https://#{mirror_hostname}/alpine/edge/main > /etc/apk/repositories
    echo https://#{mirror_hostname}/alpine/edge/community >> /etc/apk/repositories
    echo https://#{mirror_hostname}/alpine/edge/testing >> /etc/apk/repositories

    apk update
    apk add docker \
        docker-openrc \
        docker-volume-local-persist \
        docker-volume-local-persist-openrc

    rc-update add docker
    rc-update add docker-volume-local-persist
    rc-service docker start
    rc-service docker-volume-local-persist start

    adduser vagrant docker
  SHELL
end
