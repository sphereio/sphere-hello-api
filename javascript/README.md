Javascript Hello on SPHERE.IO
=============================

A Javascript example to authenticate your application to [SPHERE.IO](http://sphere.io) and get the products in your project.

## Setup

Create a property file with the credentials
```
CLIENT_ID='client_id'
CLIENT_SECRET='client_secret'
PROJECT_KEY='project_key'
```

## Get access token

As CORS are not allowed to the Auth service, we get the `access_token` with a bash script, simply run
```bash
$ cd bin
$ ./main -f ../config.me
```

This will print the credentials into `config.js`, loaded then HTML webpage.

## Run

You can run a simple server which will open automatically the page
```bash
$ grunt run
```
