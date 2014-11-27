![SPHERE.IO icon](https://admin.sphere.io/assets/images/sphere_logo_rgb_long.png)

# Go Hello API

An example using the Go language to authenticate your application to [SPHERE.IO](http://sphere.io) and get the products in your project.

## Setup
Provide a `config.json` file with your SPHERE.IO project credentials

```json
{
  "client_id": "",
  "client_secret": "",
  "project_key": ""
}
```

Now, assuming you have `go` installed, and `$GOPATH` configured, install the script dependecies and compile it

```bash
$ go get github.com/jmoiron/jsonq
$ go build hello.go
```

## Run
```bash
$ ./hello
```
