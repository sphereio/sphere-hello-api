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
          src: ['lib']
        }]

    # compiles coffeescript
    coffee:
      options:
        bare: true
        sourceMap: false
      default:
        files: grunt.file.expandMapping(['**/*.coffee'], 'lib/',
          flatten: false
          cwd: 'src'
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

  grunt.registerTask 'build', ['clean', 'coffee']
  grunt.registerTask 'serve', ['build', 'express', 'watch']
