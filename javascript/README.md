![SPHERE.IO icon](https://admin.sphere.io/assets/images/sphere_logo_rgb_long.png)

# Javascript Hello API

A Javascript example to authenticate your application to [SPHERE.IO](http://sphere.io) and get the products in your project.

## Get access token
As CORS are not allowed to the Auth service, we get the `access_token` with a bash script, simply run

```bash
$ ./bin/get_token --client_id "${CLIENT_ID}" --client_secret "${CLIENT_SECRET}" --project_key "${PROJECT_KEY}"
```

This will print the credentials into `token.js`, loaded then from HTML page.

## Run

You can run a simple server which will open automatically the page
```bash
$ grunt run
```
