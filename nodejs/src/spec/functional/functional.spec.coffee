fs = require 'fs'
_ = require 'underscore'
http = require 'http'
Promise = require 'bluebird'
{SphereClient} = require 'sphere-node-sdk'
helper = require '../SpecHelper'
Config = require '../../config'

client = new SphereClient Config

describe 'Functional Spec', ->

  checkCORSHeaders = (res) ->
    expect(res.headers['access-control-allow-origin']).toBe '*'
    expect(res.headers['access-control-allow-headers']).toBe 'Accept, Content-Type, Origin'
    expect(res.headers['access-control-allow-methods']).toBe 'GET, POST, OPTIONS'

  beforeEach (done) ->
    Promise.props
      products: client.productProjections.perPage(1).fetch()
      categories: client.categories.perPage(1).fetch()
    .then (result) =>
      @product = _.first result.products.body.results
      @category = _.first result.categories.body.results
      done()
    .catch (e) -> done(e)

  describe ':: GET /', ->

    it 'should render index page', (done) ->
      helper.http.get '/'
      .then (result) ->
        expect(result.response.statusCode).toBe 200
        expect(result.response.headers['content-type']).toBe 'text/html; charset=utf-8'
        checkCORSHeaders(result.response)
        expect(result.body).toContain 'Hello API'
        done()
      .catch (error) -> done(error)

  describe ':: GET /product/:id', ->

    it 'should render product view', (done) ->
      helper.http.get "/product/#{@product.id}"
      .then (result) =>
        expect(result.response.statusCode).toBe 200
        expect(result.response.headers['content-type']).toBe 'text/html; charset=utf-8'
        checkCORSHeaders(result.response)
        expect(result.body).toContain @product.name.en
        done()
      .catch (error) -> done(error)

    it 'should return 400 (json)', (done) ->
      helper.http.get "/product/#{@category.id}" # use a non-product UUID to test not found result
      .then (result) ->
        expect(result.response.statusCode).toBe 400
        expect(result.response.headers['content-type']).toBe 'application/json; charset=utf-8'
        checkCORSHeaders(result.response)
        expect(result.body.message).toContain 'not found'
        done()
      .catch (error) -> done(error)

  describe ':: GET /category/:id', ->

    it 'should render category view', (done) ->
      helper.http.get "/category/#{@category.id}"
      .then (result) =>
        expect(result.response.statusCode).toBe 200
        expect(result.response.headers['content-type']).toBe 'text/html; charset=utf-8'
        checkCORSHeaders(result.response)
        expect(result.body).toContain @category.name.en
        done()
      .catch (error) -> done(error)

    it 'should return 400 (json)', (done) ->
      helper.http.get "/category/#{@product.id}" # use a non-category UUID to test not found result
      .then (result) ->
        expect(result.response.statusCode).toBe 400
        expect(result.response.headers['content-type']).toBe 'application/json; charset=utf-8'
        checkCORSHeaders(result.response)
        expect(result.body.message).toContain 'not found'
        done()
      .catch (error) -> done(error)
