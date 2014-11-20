_ = require 'underscore'
Promise = require 'bluebird'

LANG = 'en'

module.exports = (app, client) ->

  # render start page
  app.get '/', (req, res) ->
    Promise.props
      products: client.productProjections.fetch()
      categories: client.categories.fetch()
    .then (result) ->

      res.render 'index',
        title: 'Hello API'
        lang: LANG
        products: result.products.body
        categories: result.categories.body

    .catch (e) -> res.status(400).json e.body

  app.get '/product/:id', (req, res) ->
    client.productProjections.byId(req.params.id).fetch()
    .then (result) ->

      res.render 'product',
        title: result.body.name[LANG]
        lang: LANG
        product: result.body

    .catch (e) -> res.status(400).json e.body

  app.get '/category/:id', (req, res) ->
    Promise.props
      products:
        client.productProjections
        .where "categories(id = \"#{req.params.id}\")"
        .fetch()
      categories: client.categories.fetch()
      category: client.categories.byId(req.params.id).fetch()
    .then (result) ->

      res.render 'category',
        title: 'Hello API'
        lang: LANG
        products: result.products.body
        categories: result.categories.body
        category: result.category.body

    .catch (e) -> res.status(400).json e.body
