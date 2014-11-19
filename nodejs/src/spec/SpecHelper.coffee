fs = require 'fs'
_ = require 'underscore'
request = require 'request'
Promise = require 'bluebird'

BASE_DOMAIN = 'http://localhost:3000'

class Requester

  options: (path, method) ->
    uri: BASE_DOMAIN + path
    json: true
    method: method
    rejectUnauthorized: false

  get: (path) ->
    new Promise (resolve, reject) =>
      request @options(path, 'GET'), (e, r, b) ->
        if e
          console.error e
          reject e
        else
          resolve
            response: r
            body: b

  post: (path, body) ->
    new Promise (resolve, reject) =>
      request _.extend({}, @options(path, 'POST'), {body: body}), (e, r, b) ->
        if e
          console.error e
          reject e
        else
          resolve
            response: r
            body: b

  upload: (path, filePath) ->
    new Promise (resolve, reject) =>
      r = request @options(path, 'POST'), (e, r, b) ->
        if e
          console.error e
          reject e
        else
          resolve
            response: r
            body: b
      form = r.form()
      form.append 'csvFile', fs.createReadStream(filePath)

module.exports =
  http: new Requester
