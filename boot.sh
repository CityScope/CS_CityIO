#!/bin/bash

# boot nginx
sudo ln -s /usr/local/opt/nginx/homebrew.mxcl.nginx.plist /Library/LaunchDaemons/
sudo chown root:wheel /usr/local/opt/nginx/homebrew.mxcl.nginx.plist
sudo launchctl load /Library/LaunchDaemons/homebrew.mxcl.nginx.plist

echo "starting nginx server"

tmux new -d -s cityio '/Users/yasushi/code/go/bin/CS_CityIO_Backend'

echo "started cityio"

tmux new -d -s choiceModels '/Users/Shared/CS_choiceModels/runCM.sh'

echo "started choiceModels"


