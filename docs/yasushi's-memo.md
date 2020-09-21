# ssl stuff
MIT issued ssl cert needs to be concatenated manually:

```cat server_domain_cert.cer server_domain_interm.cer >> bundle.crt```
```vim bundle.crt```

and make sure the ---END CERTIFICATE--- and ---BEGIN CERTIFICATE--- has
a new line between it.

# running scripts on boot in MacOS
[answer](https://superuser.com/questions/229773/run-command-on-startup-login-mac-os-x)

but better if I write a bash script that makes a tmux session and run in the ```main``` session...