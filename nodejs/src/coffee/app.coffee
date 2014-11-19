path = require 'path'
domain = require 'domain'
express = require 'express'
Logger = require './logger'
{SphereClient} = require 'sphere-node-sdk'

APP_DIR = path.join(__dirname, '../')
pkg = require "#{APP_DIR}package.json"

server = null
gracefullyExiting = false

app = express()
env = app.get 'env'

{port, logStream} = switch env
  when 'production'
    port: 3200
    logStream: [
      {level: 'info', path: "/var/log/#{pkg.name}/log"}
    ]
  else
    port: 3000
    logStream: [
      {level: 'info', stream: process.stdout}
    ]

logger = new Logger
  name: pkg.name
  streams: logStream

server = require('http').createServer(app)
logger.info "Starting express application on port #{port} (#{env})"

handleTearDown = ->
  gracefullyExiting = true
  logger.info 'Attempting gracefully shutdown of server, waiting for remaining connections to complete.'

  server.close ->
    logger.info 'No more connections, shutting down server.'
    process.exit()
  setTimeout ->
    logger.error 'Could not close connections in time, forcefully shutting down.'
    process.exit(1)
  , 30 * 1000 # 30s

process.on 'SIGINT', handleTearDown
process.on 'SIGTERM', handleTearDown

app.set 'port', port
app.set 'views', "#{APP_DIR}views"
app.set 'view engine', 'jade'
app.set 'trust proxy', true
app.use '/assets', express.static("#{APP_DIR}assets")
app.use require('./middleware/logger')(logger)
app.use (req, res, next) ->
  requestDomain = domain.create()
  requestDomain.add(req)
  requestDomain.add(res)
  requestDomain.on 'error', next
  requestDomain.run(next)
app.use (req, res, next) -> # enable CORS
  res.header 'Access-Control-Allow-Origin', '*'
  res.header 'Access-Control-Allow-Methods', 'GET, POST, OPTIONS'
  res.header 'Access-Control-Allow-Headers', 'Accept, Content-Type, Origin'
  if req.method is 'OPTIONS'
    res.status(200).send()
  else
    next()
app.use (req, res, next) ->
  return next() unless gracefullyExiting
  res.setHeader 'Connection', 'close'
  res.status(502).send message: 'Server is in the process of restarting.'

# see list of middlewares for express 4.x
# https://github.com/senchalabs/connect#middleware
app.use require('serve-favicon')("#{APP_DIR}assets/images/favicon.ico")
app.use require('multer')()
app.use require('body-parser').json()
app.use require('body-parser').urlencoded(extended: false)
app.use require('cookie-parser')()
app.use require('cookie-session')({secret: 'iamasecret'})
app.use require('compression')()
app.use (err, req, res, next) ->
  logger.error err
  res.status(500).send message: 'Oops, something went wrong!'

client = new SphereClient require('../config')
require('./routes')(app, client)

# starts server
server.listen port
logger.info "Listening for HTTP on http://localhost:#{port}"

module.exports = app
