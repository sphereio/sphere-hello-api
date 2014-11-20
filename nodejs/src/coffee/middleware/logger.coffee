useragent = require 'useragent'

expressify = (req, res, cb) ->
  status = res.statusCode
  method = req.method
  url = req.url or '-'
  referer = req.header('referer') or '-'
  ua = useragent.parse(req.header('user-agent'))
  httpVersion = "#{req.httpVersionMajor}.#{req.httpVersionMinor}"
  ip = ip or req.ip or req.socket?.remoteAddress or
        req.socket?.socket?.remoteAddresss or '127.0.0.1'

  meta =
    remoteAddress: ip
    method: method
    url: url
    referer: referer
    'user-agent': ua
    body: req.body and req.body.toString and
      req.body.toString().substring(0, Math.max(req.body.toString().length, 20))
    'http-version': httpVersion
    statusCode: status
    req: req
    res: res

  message = [
    ip
    '- -'
    method
    url
    "HTTP/#{httpVersion}"
    status
    res.get('Content-Length')
    referer
    ua.family
    "#{ua.major}.#{ua.minor}"
    ua.os
  ].join(' ')

  cb(meta, message)

module.exports = (logger) ->
  (req, res, next) ->
    expressLogger = logger.child widget_type: 'express'
    expressify req, res, (meta, message) ->
      # be less verbose regarding static resources
      if meta.url.indexOf('/assets') < 0
        expressLogger.info message
      else
        expressLogger.debug message
      res.on 'finish', -> expressLogger.debug meta, message
      next()