# cityIO API Frontend 
## Live Dashboard for MIT CityScope Worldwide Deployments


![alt text](/docs/cityIOfe.png "cityIO frontend")

This repo provides the code for a front-end interface of the cityIO backend server. This code can be used to display an overview of current active CityScope deployments around the world and show a real-time dashboard of the cityIO API Data Visualization for each table. 

![snapshot of activity March 2019](/docs/2019-03-22.png "cityIO frontend")



## Installation
Important:
With default setting this frontend use the MIT public server backend, which list all basename.
If you intend to use this server:
- be sure to have an unique tablename but that garantee nothing!
- dont upload unique and precious json there, cause everyone can update it without your consentment. (When it happens, this should change your lego table visualisation even when you dont move lego on it!)

### Dependencies for test, develop and push to git 
#### [PRs are welcomed!]

- from a dev environment: an up to date environnement: Debian Buster, Ubuntu 18.10
- get git (optionnal if you won't commit to this repo) 
- Get [nodejs] (probably adding the latest node repo for your distrib to your apt sources.list)
- Get [parcelJS](https://parceljs.org/) (`npm install -g parcel-bundler`)
- Clone repo (using git or downloading it above)
- run using `parcel index.html`
- edit at will and PR

#### for forking and deploying on your own server/GH-pages 

[See doc here](https://github.com/CityScope/CS_CityIO_Frontend/blob/master/docs/deploy.md)

### dependencies for prod env
a stable tested environnement atm: debian Stretch, Ubuntu 18.04
- get git (optionnal if you won't commit to this repo) 
- Get [nodejs] (probably from the apt repo)
- Get [parcelJS](https://parceljs.org/) (`npm install -g parcel-bundler`)
- Clone repo (using git or downloading it above)
- run using `parcel index.html`, this should `npm install .` if you haven't done it already. 

____
Maintained by [Ariel Noyman](http://arielnoyman.com)

[Repo Contributors](https://github.com/CityScope/CS_CityIO_Frontend/graphs/contributors)
