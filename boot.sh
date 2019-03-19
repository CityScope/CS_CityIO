#!/bin/bash

# boot nginx
sudo ln -s /usr/local/opt/nginx/homebrew.mxcl.nginx.plist /Library/LaunchDaemons/
sudo chown root:wheel /usr/local/opt/nginx/homebrew.mxcl.nginx.plist
sudo launchctl load /Library/LaunchDaemons/homebrew.mxcl.nginx.plist

echo "starting nginx server"
sleep 0.5
echt "started cityio..."
tmux new -d -S /tmp/cityio-session cityio '/Users/yasushi/code/go/bin/CS_CityIO_Backend'
chmod 777 /tmp/cityio-session
# GO to sleep 
echo "started choiceModels in a few seconds.."
sleep 2
tmux new -d -S /tmp/choiceModels-session choiceModels '/Users/Shared/CS_choiceModels/runCM.sh'
chmod 777 /tmp/choiceModels-session


