# #!/bin/bash
# clean dist folders
sudo rm -rf dist
sudo rm -rf frontend/dist
# build the dist for public url 
sudo parcel build frontend/index.html --public-url https://cityscope.media.mit.edu/CS_CityIO/
# make sure to add dist 
git add dist -f
#commit the GH pages changes 
git commit -m "gh-pages commit"
#push to subtree remote 
git push origin `git subtree split --prefix dist master`:gh-pages --force
# revert last commits 
git reset --hard