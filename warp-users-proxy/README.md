# warp-users-proxy

This program implements two endpoints:

* `GET api/v1/users`, which returns a truncated JSON payload retrieved from `https://jsonplaceholder.typicode.com/users`

* `POST api/v1/users`, which requires a JSON-encoded payload with valid `email` and `name` parameters

## License

Copyright 2020 Nicholas Young. Released under the [Mozilla Public License ("MPL"), version 2.0](../LICENSE).
