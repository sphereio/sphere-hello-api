module.exports = (grunt)->
  # project configuration
  grunt.initConfig
    # load package information
    pkg: grunt.file.readJSON 'package.json'

    coffee:
      default:
        src: "src/hello.coffee"
        dest: "src/hello.js"

    # watching for changes
    watch:
      default:
        files: ["src/*.coffee"]
        tasks: ["coffee"]

    connect:
      default:
        options:
          port: 3000
          base: "./"

    open:
      default:
        path: "http://localhost:<%= connect.default.options.port %>"

  # load plugins that provide the tasks defined in the config
  grunt.loadNpmTasks "grunt-contrib-coffee"
  grunt.loadNpmTasks "grunt-contrib-watch"
  grunt.loadNpmTasks "grunt-contrib-connect"
  grunt.loadNpmTasks "grunt-open"

  # register tasks
  grunt.registerTask "run", ["connect", "open", "watch"]
  grunt.registerTask "build", ["coffee"]
  grunt.registerTask "default", ["watch"]