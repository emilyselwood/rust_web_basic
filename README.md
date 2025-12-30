# a basic rust web server project starting point

Really basic project starting point.

* Config loading and defaults
* Logging to rolling files
* Index, js, css, and image endpoints
* Build script
  * Copies resources directory
  * Downloads fonts once
* "Hot" reloading of resources in debug mode.

## How to use

1) Copy the repo content into a fresh repo. You don't need the history.
1) Open `src/main.rs` near the top is a constant called `APPLICATION_NAME` change the value to the name of your application. This will change the name of the folder used for the configuration.
1) Open `src/config.rs` near the bottom is a set of default values for the configuration. Set as needed.
1) Run the application with `cargo run` the first build will take a while, by default the webserver should end up available at `http://localhost:8080/`
1) Build your thing

## Project layout.

The configuration objects are in `src/config.rs` All the existing entries are in use but 

Logging config is mostly hard coded in `src/main.rs` The defaults should keep you going for a long while.

Static resources (html, js, images, fonts, css, etc) are kept in `resources/static`. The `open` folder is designed for things that are not security sensitive and can be loaded by anyone. Paths to these folders are built in the handler functions near the bottom of `src/server.rs` Note: currently there is no secure section because it is not setup at all out of the box.

End points are configured in `src/server.rs`
