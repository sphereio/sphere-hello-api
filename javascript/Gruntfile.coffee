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
        files: ["Gruntfile.coffe", "src/*.coffee"]
        tasks: ["coffee"]

    connect:
      default:
        options:
          port: 3000
          base: "./"

    open:
      default:
        path: "http://localhost:<%= connect.default.options.port %>"

    shell:
      options:
        stdout: true
        stderr: true
        failOnError: true
      test:
        command: "casperjs test src/test.coffee --no-colors"

  # load plugins that provide the tasks defined in the config
  grunt.loadNpmTasks "grunt-contrib-coffee"
  grunt.loadNpmTasks "grunt-contrib-watch"
  grunt.loadNpmTasks "grunt-contrib-connect"
  grunt.loadNpmTasks "grunt-open"
  grunt.loadNpmTasks "grunt-shell"

  # register tasks
  grunt.registerTask "run", ["connect", "open", "watch"]
  grunt.registerTask "build", ["coffee"]
  grunt.registerTask "default", ["watch"]
  grunt.registerTask "test", ["build", "connect", "shell:test"]
