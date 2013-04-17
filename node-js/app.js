// Module dependencies
var express     = require('express'),
    querystring = require('querystring'),
    request     = require('request'),
    Config      = require('./config')

var app = express()

// Configuration
app.configure(function(){
  app.use(express.logger('dev'))
  app.use(express.bodyParser())
  app.use(express.methodOverride())
  app.use(app.router)
})

app.configure('development', function(){
  app.use(express.errorHandler({ dumpExceptions: true, showStack: true }))
})

// Routes
app.get('/', login)

app.listen(3000, function(){
  console.log("Server listening on port 3000...")
})


function login(req, res){
  console.log("Requesting an Access Token...")

  var params = {
    'grant_type': 'client_credentials',
    'scope': 'manage_project:' + Config.project_key
  }

  var payload = querystring.stringify(params)
  var request_options = {
    uri: 'https://' + Config.client_id + ':' + Config.client_secret + '@' + 'auth.sphere.io/oauth/token',
    method: 'POST',
    body: payload,
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
      'Content-Length': payload.length
    },
    timeout: 20000
  };

  request(request_options, function(error, response, body) {
    var json_body = JSON.parse(body)
    console.log(json_body)

    if (response.statusCode == 200) {
      console.log("...authorized!")
      getProducts(req, res, json_body)
    } else {
      throw new Error('Failed to get Access Token.')
    }
  })
}


function getProducts(req, res, json_body){
  console.log("Fetching products...")

  var request_options = {
    uri: 'https://api.sphere.io' + "/" + Config.project_key + "/product-projections",
    method: 'GET',
    headers: {
      'Authorization': 'Bearer ' + json_body.access_token
    },
    timeout: 20000
  }

  request(request_options, function(error, response, body) {
    if (response.statusCode == 200) {
      products = JSON.parse(body)
      res.json(products)
    }
  })
}