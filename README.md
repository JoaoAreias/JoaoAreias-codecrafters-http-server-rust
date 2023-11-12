# Build Your Own HTTP server - Challenge.

This is my implementation for the Code crafter's Build Your Own HTTP server" Challenge.
In this challenge I've implemented a simple HTTP server in Rust capable of:

- Handling GET requests
- Handling concurrent connections
- Receiving and saving a file through a post request and saving it on a directory

You can build the project through Cargo through:

`cargo build`

To run the server please use:

`./your_server.sh --directory <directory>`

> :warning: **This is s toy project and should not be used in production**

For more information and to check out their project please see their links bellow.

---

[![progress-banner](https://backend.codecrafters.io/progress/http-server/bf2ad5a6-9d71-4b03-b0ee-c1b9ad60df89)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/courses/http-server/overview).

[HTTP](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol) is the
protocol that powers the web. In this challenge, you'll build a HTTP/1.1 server
that is capable of serving multiple clients.

Along the way you'll learn about TCP servers,
[HTTP request syntax](https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html),
and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.