[![progress-banner](https://backend.codecrafters.io/progress/http-server/b3f48794-d610-4d13-971d-aa461f9b2bc7)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is my implementation of CodeCrafter's
["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/courses/http-server/overview).

Implemented using Rust. The server is able to handle concurrent connections using Tokio threads and async file, read and write operations.

Supports:

1. GET requests

   - `/` returns a 200 OK response
   - `/echo/<filepath>` echoes back `filepath` as a response with 200 OK
   - `/user-agent` respones with the request `User-Agent` in the body with 200 OK
   - `/files/<filepath>` returns the file located at `/directory/filepath` with 200 OK if it exists, otherwise 404 Not Found (directory specified in `./your_server.sh --directory <directory>`)

2. POST requests:
   - `/files/<filepath>` creates a file in `/directory/filepath` with the contents specified in the request body

Learned a lot about the following Rust-specific areas

- Async programming with Tokio (file reading, writing/reading over a TCP connection)
- Concurrency with Tokio threads
- Iterators
- Traits (implemented `FromStr` and `ToString` on response and request structs)
- Error Handling (`Err` variant of `Result`, `None` variant of `Option`, error propagation, etc)
- Pattern Matching using `match`
- Shared variables across threads using Atomically Referenced Counters (`Arc`)
- Enums
