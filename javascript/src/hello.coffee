(($, window) ->
  document = window.document

  class Hello
    constructor: (options) ->
      # we assume to have already an `access_token`
      @_options = options

    getProducts: (callback) ->
      $.ajax
        url: "https://api.sphere.io/#{@_options.project_key}/product-projections"
        type: "GET"
        headers:
          "Authorization": "Bearer #{@_options.access_token}"
        success: (data, textStatus, jqXHR) =>
          callback(undefined, data)
        error: (xhr, textStatus) => callback(xhr, undefined)

  window.Hello = Hello

)(jQuery, window)
