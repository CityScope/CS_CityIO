#!/bin/bash

# boot nginx
sudo ln -s /usr/local/opt/nginx/homebrew.mxcl.nginx.plist /Library/LaunchDaemons/
sudo chown root:wheel /usr/local/opt/nginx/homebrew.mxcl.nginx.plist
sudo launchctl load /Library/LaunchDaemons/homebrew.mxcl.nginx.plist

echo "starting nginx server"
sleep 0.5
echo "started cityio..."
tmux new -d -s cityio '/Users/yasushi/code/go/bin/CS_CityIO_Backend'
# GO to sleep 
echo "started choiceModels in a few seconds.."
sleep 2
tmux new -d -s choiceModels '/Users/Shared/CS_choiceModels/runCM.sh'


