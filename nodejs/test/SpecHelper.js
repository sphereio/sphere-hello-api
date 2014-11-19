var BASE_DOMAIN, Promise, Requester, fs, request, _;

fs = require('fs');

_ = require('underscore');

request = require('request');

Promise = require('bluebird');

BASE_DOMAIN = 'http://localhost:3000';

Requester = (function() {
  function Requester() {}

  Requester.prototype.options = function(path, method) {
    return {
      uri: BASE_DOMAIN + path,
      json: true,
      method: method,
      rejectUnauthorized: false
    };
  };

  Requester.prototype.get = function(path) {
    return new Promise((function(_this) {
      return function(resolve, reject) {
        return request(_this.options(path, 'GET'), function(e, r, b) {
          if (e) {
            console.error(e);
            return reject(e);
          } else {
            return resolve({
              response: r,
              body: b
            });
          }
        });
      };
    })(this));
  };

  Requester.prototype.post = function(path, body) {
    return new Promise((function(_this) {
      return function(resolve, reject) {
        return request(_.extend({}, _this.options(path, 'POST'), {
          body: body
        }), function(e, r, b) {
          if (e) {
            console.error(e);
            return reject(e);
          } else {
            return resolve({
              response: r,
              body: b
            });
          }
        });
      };
    })(this));
  };

  Requester.prototype.upload = function(path, filePath) {
    return new Promise((function(_this) {
      return function(resolve, reject) {
        var form, r;
        r = request(_this.options(path, 'POST'), function(e, r, b) {
          if (e) {
            console.error(e);
            return reject(e);
          } else {
            return resolve({
              response: r,
              body: b
            });
          }
        });
        form = r.form();
        return form.append('csvFile', fs.createReadStream(filePath));
      };
    })(this));
  };

  return Requester;

})();

module.exports = {
  http: new Requester
};
