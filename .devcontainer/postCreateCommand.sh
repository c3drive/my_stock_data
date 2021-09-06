#!/bin/bash
echo "Installing Developer Requirements"

# AWS KEY
cp -rf /workspace/.aws ~/

# Docker Install
apt-get update
apt-get install -y apt-transport-https ca-certificates curl gnupg lsb-release
curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo  "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
apt-get update
apt-get install -y docker-ce docker-ce-cli containerd.io
#apt-cache madison docker-ce
apt-get install -y docker-ce=5:20.10.8~3-0~debian-bullseye docker-ce-cli=5:20.10.8~3-0~debian-bullseye containerd.io
# service docker start

# AWS SAM CLI Install
wget  -P /tmp https://github.com/aws/aws-sam-cli/releases/latest/download/aws-sam-cli-linux-x86_64.zip
unzip /tmp/aws-sam-cli-linux-x86_64.zip -d /tmp/sam-installation
/tmp/sam-installation/install
rm -rf /tmp/aws-sam-cli-linux-x86_64.zip
