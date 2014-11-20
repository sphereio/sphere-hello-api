module.exports = (grunt) ->

  # Load grunt tasks automatically
  require('load-grunt-tasks')(grunt)

  # project configuration
  grunt.initConfig

    # load package information
    pkg: grunt.file.readJSON 'package.json'

    # make sure code style is correct
    coffeelint:
      options: grunt.file.readJSON('node_modules/sphere-coffeelint/coffeelint.json')
      default: ['Gruntfile.coffee', './**/*.coffee']

    # empties folders to start fresh
    clean:
      default:
        files: [{
          dot: true
          src: ['lib', 'test']
        }]

    # compiles coffeescript
    coffee:
      options:
        bare: true
        sourceMap: false
      default:
        files: grunt.file.expandMapping(['**/*.coffee'], 'lib/',
          flatten: false
          cwd: 'src/coffee'
          ext: '.js'
          rename: (dest, matchedSrcPath) ->
            dest + matchedSrcPath
          )
      test:
        files: grunt.file.expandMapping(['**/*.spec.coffee'], 'test/',
          flatten: false
          cwd: 'src/spec'
          ext: '.spec.js'
          rename: (dest, matchedSrcPath) ->
            dest + matchedSrcPath
          )
      testHelpers:
        files: grunt.file.expandMapping(['**/*Helper.coffee'], 'test/',
          flatten: false
          cwd: 'src/spec'
          ext: '.js'
          rename: (dest, matchedSrcPath) ->
            dest + matchedSrcPath
          )

    # starts express server and set the environment
    express:
      options:
        port: 3000
        debug: false
      default:
        options:
          node_env: 'development'
          script: 'lib/app.js'
      test:
        options:
          output: 'Listening' # waits for matching message before passing control back to grunt
          node_env: 'test'
          script: "lib/app.js"

    # watching for changes
    watch:
      options:
        spawn: false # Without this option specified express won't be reloaded
      default:
        files: [
          'Gruntfile.coffee',
          'src/**/*.coffee',
          './**/*.jade'
        ]
        tasks: ['build', 'express']

    shell:
      options:
        stdout: true
        stderr: true
        failOnError: true
      jasmine:
        command: 'jasmine-node --verbose --captureExceptions test'

  grunt.registerTask 'build', ['clean', 'coffee']
  grunt.registerTask 'run', ['build', 'express:default']
  grunt.registerTask 'serve', ['run', 'watch']
  grunt.registerTask 'test', ['build', 'express:test', 'shell:jasmine']
