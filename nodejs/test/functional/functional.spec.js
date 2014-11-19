var Config, Promise, SphereClient, client, fs, helper, http, _;

fs = require('fs');

_ = require('underscore');

http = require('http');

Promise = require('bluebird');

SphereClient = require('sphere-node-sdk').SphereClient;

helper = require('../SpecHelper');

Config = require('../../config');

client = new SphereClient(Config);

describe('Functional Spec', function() {
  var checkCORSHeaders;
  checkCORSHeaders = function(res) {
    expect(res.headers['access-control-allow-origin']).toBe('*');
    expect(res.headers['access-control-allow-headers']).toBe('Accept, Content-Type, Origin');
    return expect(res.headers['access-control-allow-methods']).toBe('GET, POST, OPTIONS');
  };
  beforeEach(function(done) {
    return Promise.props({
      products: client.productProjections.perPage(1).fetch(),
      categories: client.categories.perPage(1).fetch()
    }).then((function(_this) {
      return function(result) {
        _this.product = _.first(result.products.body.results);
        _this.category = _.first(result.categories.body.results);
        return done();
      };
    })(this))["catch"](function(e) {
      return done(e);
    });
  });
  describe(':: GET /', function() {
    return it('should render index page', function(done) {
      return helper.http.get('/').then(function(result) {
        expect(result.response.statusCode).toBe(200);
        expect(result.response.headers['content-type']).toBe('text/html; charset=utf-8');
        checkCORSHeaders(result.response);
        expect(result.body).toContain('Hello API');
        return done();
      })["catch"](function(error) {
        return done(error);
      });
    });
  });
  describe(':: GET /product/:id', function() {
    it('should render product view', function(done) {
      return helper.http.get("/product/" + this.product.id).then((function(_this) {
        return function(result) {
          expect(result.response.statusCode).toBe(200);
          expect(result.response.headers['content-type']).toBe('text/html; charset=utf-8');
          checkCORSHeaders(result.response);
          expect(result.body).toContain(_this.product.name.en);
          return done();
        };
      })(this))["catch"](function(error) {
        return done(error);
      });
    });
    return it('should return 400 (json)', function(done) {
      return helper.http.get("/product/" + this.category.id).then(function(result) {
        expect(result.response.statusCode).toBe(400);
        expect(result.response.headers['content-type']).toBe('application/json; charset=utf-8');
        checkCORSHeaders(result.response);
        expect(result.body.message).toContain('not found');
        return done();
      })["catch"](function(error) {
        return done(error);
      });
    });
  });
  return describe(':: GET /category/:id', function() {
    it('should render category view', function(done) {
      return helper.http.get("/category/" + this.category.id).then((function(_this) {
        return function(result) {
          expect(result.response.statusCode).toBe(200);
          expect(result.response.headers['content-type']).toBe('text/html; charset=utf-8');
          checkCORSHeaders(result.response);
          expect(result.body).toContain(_this.category.name.en);
          return done();
        };
      })(this))["catch"](function(error) {
        return done(error);
      });
    });
    return it('should return 400 (json)', function(done) {
      return helper.http.get("/category/" + this.product.id).then(function(result) {
        expect(result.response.statusCode).toBe(400);
        expect(result.response.headers['content-type']).toBe('application/json; charset=utf-8');
        checkCORSHeaders(result.response);
        expect(result.body.message).toContain('not found');
        return done();
      })["catch"](function(error) {
        return done(error);
      });
    });
  });
});
