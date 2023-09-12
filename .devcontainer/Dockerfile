FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# az cli
RUN sudo apt-get update; sudo apt-get install ca-certificates curl apt-transport-https lsb-release gnupg
RUN sudo mkdir -p /etc/apt/keyrings
RUN curl -sLS https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor | sudo tee /etc/apt/keyrings/microsoft.gpg > /dev/null; sudo chmod go+r /etc/apt/keyrings/microsoft.gpg
RUN AZ_REPO=$(lsb_release -cs); echo "deb [arch=`dpkg --print-architecture` signed-by=/etc/apt/keyrings/microsoft.gpg] https://packages.microsoft.com/repos/azure-cli/ $AZ_REPO main" | sudo tee /etc/apt/sources.list.d/azure-cli.list
RUN sudo apt-get update ; apt-get install azure-cli
# other stuff
RUN sudo apt-get update ; sudo apt-get install -y vim tmux openssh-server

ENV TZ=Europe/Berlin
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN git clone https://github.com/cdriehuys/axum-jwks.git
WORKDIR axum-jwks
RUN git checkout v0.5.0
