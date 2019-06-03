# #!/bin/bash
sudo rm -rf dist

# build the dist for public url 
sudo parcel build frontend/index.html --public-url https://cityscope.media.mit.edu/CS_cityio/
# make sure to add dist 
git add dist -f

date=`date +%Y-%m-%d`
msg="gh-pages commit at "
cMsg=$msg$date
echo $cMsg
#commit the GH pages changes 
git commit -m cMsg
#push to subtree remote 
git push origin `git subtree split --prefix dist master`:gh-pages --force
