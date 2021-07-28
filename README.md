# Midori

[![CI][ci-badge]][ci-url]
[![Codacy][codacy-badge]][codacy-url]
[![License][mit-badge]][mit-url]
![Activity][activity-img]

[ci-badge]: https://github.com/zephyrchien/midori/workflows/ci/badge.svg
[ci-url]: https://github.com/zephyrchien/midori/actions

[codacy-badge]: https://app.codacy.com/project/badge/Grade/908ed7e0dd5f4bec8984856931021165
[codacy-url]: https://www.codacy.com/gh/zephyrchien/midori/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=zephyrchien/midori&amp;utm_campaign=Badge_Grade

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/zephyrchien/midori/blob/master/LICENSE

[activity-img]: https://img.shields.io/github/commit-activity/m/zephyrchien/midori?color=green&label=commit

## Protocols
- [x] [TCP][tcp-doc-url]
- [x] [UDS][uds-doc-url]
- [ ] UDP
- [x] [TLS][tls-doc-url]
- [x] [WebSocket][ws-doc-url]
- [x] [HTTP2][h2-doc-url]
- [ ] gRPC
- [ ] QUIC

[doc-url]: https://github.com/zephyrchien/midori/tree/master/docs

[tcp-doc-url]: https://github.com/zephyrchien/midori/blob/master/docs/tcp.md

[uds-doc-url]: https://github.com/zephyrchien/midori/blob/master/docs/uds.md

[tls-doc-url]: https://github.com/zephyrchien/midori/blob/master/docs/tls.md

[ws-doc-url]: https://github.com/zephyrchien/midori/blob/master/docs/ws.md

[h2-doc-url]: https://github.com/zephyrchien/midori/blob/master/docs/h2.md

## Usage
```bash
midori -c config.json
```

## Quick Start
Get started with a simple TCP relay(supports zero-copy on linux).

First, create a config file. Just specify the listen and remote address.
```json
// config.json
{
    "endpoints":[
        {
            "listen": "0.0.0.0:5000",
            "remote": "1.2.3.4:8080"
        },
        {
            "listen": "0.0.0.0:10000",
            "remote": "www.example.com:443"
        },
    ]
}
```

Then launch these 2 endpoints:
```shell
midori -c config.json
```

Note: Almost all kinds of address are supported, including `ipv4`, `ipv6`, `domain name` and `unix socket path`.

<br>

## Full Configuration
<details>
<summary>show example</summary>
<pre><code>
{
  "dns_mode": "ipv4_then_ipv6",
  "endpoints": [
    {
      "listen": {
        "addr": "0.0.0.0:5000",
        "net": "tcp",
        "trans": {
          "proto": "ws",
          "path": "/"
        },
        "tls": {
          "cert": "x.crt",
          "key": "x.pem",
          "versions": "tlsv1.3, tlsv1.2",
          "aplns": "http/1.1",
          "ocsp": "x.ocsp"
        }
      },
      "remote": {
        "addr": "www.example.com:443",
        "net": "tcp",
        "trans": {
          "proto": "h2",
          "path": "/",
          "server_push": false
        },
        "tls": {
          "roots": "firefox",
          "versions": "tlsv1.3, tlsv1.2",
          "sni": "www.example.com",
          "aplns": "h2",
          "skip_verify": false,
          "enable_sni": true
        }
      }
    }
  ]
}
</code></pre>
</details>

### Global
Currently, the configuration file only consists of 2 fields:
```shell
{
    "dns_mode": "", // and other global params
    "endpoints": []
}
```

### DNS Mode
The `trust-dns` crate supports these strategies:
- ipv4_only
- ipv6_only
- ipv4_then_ipv6 (*default*)
- ipv6_then_ipv4
- ipv4_and_ipv6

### Endpoint(s)
Each endpoint contains an associated pair of `listen` and `remote`.
```bash
// endpoint
{
    "listen": "",
    "remote": ""
}
```

Below is the options of `listen` or `remote`. **Each field has a default value except for `addr`**. <br>

Moreover, `trans` and `tls` also support more complicated params(e.g. `path`, `sni`, `ocsp`..). [See Protocol Docs for more details][doc-url].
```bash
// listen or remote
{
    "addr": "",  // must
    "net": "",  // tcp(deafult), uds, udp
    "trans": "",  // plain(default), ws, h2..
    "tls": ""  // none(default)
}
```

Note that all the protocols can be applied to both sides of `listen` and `remote`. You could either use `net` directly or combine them with `transport`.
| net | tcp | uds | udp |
| :---: | :---: | :---: | :---: |
| tls | O | O |
| ws | O | O |
| h2 | O | O |
| grpc | O | O |
| quic | | | O |

## License
[The MIT License (MIT)](https://github.com/zephyrchien/midori/blob/master/LICENSE)