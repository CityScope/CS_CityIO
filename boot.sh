#!/bin/bash

if [ "$(whoami)" != "root" ]; then
  echo "please RUN boot.sh with SUDO"
  exit 1
fi

# boot nginx
sudo ln -s /usr/local/opt/nginx/homebrew.mxcl.nginx.plist /Library/LaunchDaemons/
sudo chown root:wheel /usr/local/opt/nginx/homebrew.mxcl.nginx.plist
sudo launchctl load /Library/LaunchDaemons/homebrew.mxcl.nginx.plist

tmux -S /tmp/shared kill-server

echo "starting nginx server"
sleep 0.5

tmux -S /tmp/shared new-session -d  -s cityio
tmux -S /tmp/shared send-keys '/Users/yasushi/code/go/bin/CS_CityIO_Backend' C-m
# tmux -S /tmp/shared detach -s cityio
echo "started cityio... attach using following command"
echo "tmux -S /tmp/shared a -t cityio"

sleep 2

tmux -S /tmp/shared new-session -d  -s choiceModels
tmux -S /tmp/shared send-keys '/Users/Shared/CS_choiceModels/runCM.sh' C-m
# jtmux -S /tmp/shared detach -s choiceModels
echo "started mocho..."
echo "tmux -S /tmp/shared a -t choiceModels"

chmod 777 /tmp/shared

