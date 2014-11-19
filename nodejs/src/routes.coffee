_ = require 'underscore'
Promise = require 'bluebird'

module.exports = (app, client) ->

  # render start page
  app.get '/', (req, res) ->
    Promise.props
      products: client.productProjections.fetch()
      categories: client.categories.fetch()
    .then (result) ->

      res.render 'main',
        title: 'Hello API'
        lang: 'en'
        products: result.products.body
        categories: result.categories.body
