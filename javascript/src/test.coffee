url = 'http://localhost:3000/'

casper.test.begin 'Should fetch product projections', (test) ->
  casper.start url, ->
    test.assertSelectorHasText '#title', 'Found 5 products'
  .run -> test.done()

# Note: if you see following warning it's nothing serious (see here http://stackoverflow.com/a/27083831)
#   Unsafe JavaScript attempt to access frame with URL about:blank from frame with URL
