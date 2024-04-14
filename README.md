This is a simple web server written in rust and [Rocket](https://rocket.rs/). It's based off the [TODO app example](https://github.com/rwf2/Rocket/blob/master/examples/todo/README.md).
It has built-in sqlite integration, meaning we can host it on any file system or cloud provider without needing database integrations.

It currently only supports adding, removing, and viewing people from a waitlist.
You can view the waitlist either by accessing the sqlite database, or by navigating to the index route of the server.
