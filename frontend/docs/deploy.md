## how to build with `parcel` for GitHub pages

### Building production into `dist` folder

<!-- `parcel build index.html --public-url https://cityscope.github.io/CS_CityIO_Frontend/` -->

`parcel build index.html --public-url https://cityscope.media.mit.edu/CS_CityIO_Frontend/`

Note: in some cases might need this to fix missing plugin note:
`npm install babel-plugin-transform-runtime`

## Deploying `dist` to GitHub Pages

### Step 1

Remove the `dist` directory from the project’s `.gitignore` (or skip and force-add afterwards).

### Step 2

Make sure git knows about your subtree (the subfolder with your site).

```sh
git add dist
```

or force-add it if you don't want to change your `.gitignore`

```sh
git add dist -f
```

Commit!

```sh
git commit -m "gh-pages commit"
```

### Step 3

Use subtree push to send it to the `gh-pages` branch on GitHub.

```sh
git subtree push --prefix dist origin gh-pages
```

If this gets an error [see below], try `force` push:

```sh
git push origin `git subtree split --prefix dist master`:gh-pages --force
```

---

# How to fix

### `Updates were rejected because a pushed branch tip is behind its remote`

##### full error msg:

```
$ git subtree push --prefix dist origin gh-pages
git push using:  origin gh-pages
To https://github.com/RELNO/cityIO_Forntend.git
 ! [rejected]        a19d9bc6e8046b507cde9154ec94daad3e7aeefa -> gh-pages (non-fast-forward)
error: failed to push some refs to 'https://github.com/RELNO/cityIO_Forntend.git'
hint: Updates were rejected because a pushed branch tip is behind its remote
hint: counterpart. Check out this branch and integrate the remote changes
hint: (e.g. 'git pull ...') before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
```

### Setup

```$ rm -rf dist
$ echo "dist/" >> .gitignore

$ git worktree add dist gh-pages
```

### Making changes

```
$ make # or what ever you run to populate dist
$ cd dist
$ git add --all
$ git commit -m "Deploy to gh-pages"
$ git push origin gh-pages [or: git push -f origin <branch> for force commit]
$ cd ..
```

### Notes

git worktree feature has its own garbage collection so if dist is deleted it will not affect much and can be recreated as needed. If you want it to go away you can use `git worktree prune` See man pages on it.

https://gist.github.com/cobyism/4730490#gistcomment-2337463
