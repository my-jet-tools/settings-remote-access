# Settings Remote Access

Runs in a remote data-center and gives its services settings from the central
[settings-service](../settings-service) with the secrets' **remote** values.

```
service in remote DC ──GET /settings/{product}/{template}──▶ settings-remote-access
                                                               │  env-info: $ENV_INFO
                                                               ▼
                                         https://domain@ip:port/settings/{product}/{template}
                                                        (settings-service)
```

- Only `GET /settings/{product}/{template}` is proxied. The settings-service
  admin API, UI and MCP are not reachable through this service.
- Every request to settings-service carries the `env-info` header with the
  value of the `ENV_INFO` environment variable of this service. Whatever
  `env-info` the client sent is ignored — the data-center is identified by this
  service, not by its clients.
- settings-service treats an `env-info` that does not match its
  `local_env_prefixes` as remote and substitutes `remote_value` of each secret
  (see settings-service README, *Remote Secret*).
- `settings_service_url` may use the FlUrl form `https://domain@ip:port`: the
  socket is opened to `ip:port`, while `domain` stays the Host header and the TLS
  SNI / certificate name. No DNS is needed in the remote data-center.
- The upstream status code, `content-type` and body are returned as is. If
  settings-service can not be reached the response is `502`.

## Configuration

Environment variable (required, the service does not start without it):

```
ENV_INFO=AWS-US-PROD
```

It must not start with any of settings-service `local_env_prefixes`.

`~/.settings-remote-access`:

```yaml
http_port: 8000
settings_service_url: https://settings.example.com@15.0.0.5:443
request_timeout_sec: 10
```

| Field | Default | |
|---|---|---|
| `http_port` | `8000` | |
| `settings_service_url` | — | `https://domain@ip:port` or a plain url |
| `request_timeout_sec` | `10` | for connecting and for reading the body |

## Client side

Services in the remote data-center point `SETTINGS_URL` at this service:

```
SETTINGS_URL=http://settings-remote-access:8000/settings/{product}/{template}
```

`ENV_INFO` is not needed on the clients.
