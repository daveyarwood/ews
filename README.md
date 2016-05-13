# electronic worksheet system (ews)

*TODO: explain*

## Development

### Prerequisites

* [node](http://nodejs.org)
* [npm](http://npmjs.com)
* [boot](http://boot-clj.com)

### OK, now what?

All you have to do is run `boot build`.

Or you can run `boot dev`, which watches and re-builds each time a file changes.

What this does is:

* Compiles ClojureScript -> JavaScript, resulting in a single file `target/main.js`.
* Uses `npm link` to create an `ews` executable in your `$PATH`.

