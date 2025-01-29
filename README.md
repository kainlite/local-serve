## Basic file explorer

This is not a webserver per se, but a file share over HTTP.

The main idea is to serve the current folder, this is intended to be used to serve any folder on the system, there are
no security measures, so be aware of leaving this running in a computer connected to any network, but for simple file
sharing it is more than enough, just remember to kill it afterwards.

It can be installed with cargo by doing:
```bash
cargo install local-serve
```

Usage:
```bash
Usage: local-serve [OPTIONS]

Options:
  -d, --dir <DIR>    Directory to serve files from [default: .]
  -p, --port <PORT>  Port to listen on [default: 3000]
  -h, --help         Print help
  -V, --version      Print version
```

Example:
```bash
local-serve
2025-01-29T22:05:13.092748Z  INFO local_serve: 🚀 File server starting on http://0.0.0.0:3000
2025-01-29T22:05:13.092766Z  INFO local_serve: 📂 Serving files from: .
2025-01-29T22:05:13.092786Z  INFO local_serve: listening on 0.0.0.0:3000
2025-01-29T22:06:02.424260Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2025-01-29T22:06:02.424388Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=0 ms status=200
2025-01-29T22:06:04.524044Z DEBUG request{method=GET uri=/src/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2025-01-29T22:06:04.524268Z DEBUG request{method=GET uri=/src/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=0 ms status=200
2025-01-29T22:06:05.796686Z DEBUG request{method=GET uri=/src/main.rs version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2025-01-29T22:06:05.796887Z DEBUG request{method=GET uri=/src/main.rs version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=0 ms status=200
```

This is basically an alternative for:
```bash
# Python version
python3 -m http.server

# Ruby version
ruby -run -ehttpd . -p8000

# PHP version
php -S 127.0.0.1:8000
```

And many more as you can find [here](https://gist.github.com/willurd/5720255), however sometimes you don't have an
interpreter, so a binary can do the trick.
