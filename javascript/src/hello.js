(function() {
  (function($, window) {
    var Hello, document;
    document = window.document;
    Hello = (function() {
      function Hello(options) {
        this._options = options;
      }

      Hello.prototype.getProducts = function(callback) {
        var _this = this;
        return $.ajax({
          url: "https://api.sphere.io/" + this._options.project_key + "/product-projections",
          type: "GET",
          headers: {
            "Authorization": "Bearer " + this._options.access_token
          },
          success: function(data, textStatus, jqXHR) {
            return callback(void 0, data);
          },
          error: function(xhr, textStatus) {
            return callback(xhr, void 0);
          }
        });
      };

      return Hello;

    })();
    return window.Hello = Hello;
  })(jQuery, window);

}).call(this);
