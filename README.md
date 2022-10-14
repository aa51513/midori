# Roma（All roads lead to Rome）
[Not a misspelling, roma means Rome in Italian]
[![CI][ci-badge]][ci-url]
[![Codacy][codacy-badge]][codacy-url]
[![License][mit-badge]][mit-url]
![Activity][activity-img]

[ci-badge]: https://github.com/aa51513/roma/workflows/ci/badge.svg
[ci-url]: https://github.com/aa51513/roma/actions

[codacy-badge]: https://app.codacy.com/project/badge/Grade/908ed7e0dd5f4bec8984856931021165
[codacy-url]: https://www.codacy.com/gh/aa51513/roma/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=aa51513/roma&amp;utm_campaign=Badge_Grade

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/aa51513/roma/blob/master/LICENSE

[activity-img]: https://img.shields.io/github/commit-activity/m/aa51513/roma?color=green&label=commit

## Protocols
- [x] [TCP][tcp-doc-url]
- [x] [UDS][uds-doc-url]
- [x] [UDP][udp-doc-url]
- [x] [TLS][tls-doc-url]
- [x] [WebSocket][ws-doc-url]
- [x] [HTTP2][h2-doc-url]
- [ ] KCP
- [ ] gRPC
- [x] [QUIC][quic-doc-url]

[doc-url]: https://github.com/aa51513/roma/tree/master/docs

[tcp-doc-url]: https://github.com/aa51513/roma/blob/master/docs/tcp.md

[uds-doc-url]: https://github.com/aa51513/roma/blob/master/docs/uds.md

[udp-doc-url]: https://github.com/aa51513/roma/blob/master/docs/udp.md

[tls-doc-url]: https://github.com/aa51513/roma/blob/master/docs/tls.md

[ws-doc-url]: https://github.com/aa51513/roma/blob/master/docs/ws.md

[h2-doc-url]: https://github.com/aa51513/roma/blob/master/docs/h2.md

[quic-doc-url]: https://github.com/aa51513/roma/blob/master/docs/quic.md

## Build
```shell
git clone https://github.com/aa51513/roma
cd roma
cargo build --release
```
### Optional Features
- `uds` -- enable unix domain socket
- `udp` -- enable udp
- `tls` -- enable tls(rustls)
- `ws` -- enable websocket
- `h2c` -- enable http2
- `quic` -- enable quic
- `full` -- enable all above (*default*)
```shell
# tcp only
cargo build --release --no-default-features

# with tls support
cargo build --release --no-default-features --features tls

# with other protocols
cargo build --release --no-default-features --features tls,ws,h2c
```
## Usage
```shell
roma [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <file>    specify a config file
```

## Quick Start
Let's start with a simple TCP relay(supports zero-copy on linux). Just create a config file and then specify the listen and remote address:

```json
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

Launch these 2 endpoints:
```shell
roma -c config.json
```

Almost all kinds of address are supported, including `ipv4`, `ipv6`, `domain name` and `unix socket path`.

## Log
This program is equipped with a light-weight logger, which is disabled by default. You can provide env variables to enable it.

Supported log levels:
- Off
- Error
- Warn
- Info
- Debug
- Trace

Example:
```shell
RUST_LOG=debug roma
```

## Full Configuration
<details>
<summary>show example</summary>
<pre><code>
{
    "dns_mode": "ipv4_then_ipv6",
    "dns_servers": [{
            "addr": "8.8.8.8:53",
            "protocol": "tcp"
        }, {
            "addr": "114.114.114.114:53",
            "protocol": "udp"
        }
    ],
    "endpoints": [{
            "listen": {
                "addr": "0.0.0.0:5000",
                "net": "tcp",
                "trans": {
                    "proto": "ws",
                    "path": "/"
                },
                "tls": {
                    "cert": "client.pem",
                    "key": "client.key",
                    "versions": ["tlsv1.3", "tlsv1.2"],
                    "aplns": "http/1.1"
                }
            },
            "remote": {
                "addr": "www.baidu.com:443",
                "net": "tcp",
                "trans": {
                    "proto": "h2",
                    "path": "/",
                    "server_push": false
                },
                "tls": {
                    "roots": "firefox",
                    "versions": ["tlsv1.3", "tlsv1.2"],
                    "sni": "www.baidu.com",
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
Currently, the configuration file only consists of 3 fields:
```shell
{
    "dns_mode": "", // and other global params
    "dns_servers": [], // dns server info
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

### Dns Server(s) optional!!
Each dns server contains an associated pair of `addr` and `protocol`:
```bash
{
    "addr": "",
    "protocol": ""
}
```
Options of `addr` & `protocol`:

```bash
{
    "addr": "",  // must be with port such as 127.0.0.1:5353
    "protocol": ""  // udp(default),tcp
}
```
default Dns Servers is 
```bash
8.8.8.8:53,
8.8.4.4:53,
[2001:4860:4860::8888]:53,
[2001:4860:4860::8844]:53
with udp protocol
```

### Endpoint(s)
Each endpoint contains an associated pair of `listen` and `remote`:
```bash
{
    "listen": "",
    "remote": ""
}
```

Options of `listen` & `remote`:

```bash
{
    "addr": "",  // must
    "net": "",  // tcp(deafult), uds, udp
    "trans": "",  // plain(default), ws, h2..
    "tls": ""  // none(default)
}
```
Not all fields above are required. If not specified, the default value will be applied. `trans` and `tls` have more complicated params. [See protocol docs for more details][doc-url].

You can freely combine `net`, `trans` and `tls`. For example, tcp + ws + tls = wss; uds + h2 + tls = h2(over uds).

All possible combinations:
| net | tls| trans | result |
| :---: | :---: | :---: | :---: |
| tcp/uds | none   | plain | plain tcp/uds      |
| tcp/uds | rustls | plain | tls over tcp/uds   |
| tcp/uds | none   | ws    | ws over tcp/uds    |
| tcp/uds | rustls | ws    | wss over tcp/uds   |
| tcp/uds | none   | h2    | h2c over tcp/uds   |
| tcp/uds | rustls | h2    | http2  over tcp/uds|
| tcp/uds | none   | grpc  | grpc over tcp/uds  |
| tcp/uds | rustls | grpc  | grpc over tcp/uds  |
| udp     | none   | plain | plain udp          |
| udp     | none   | kcp   | kcp                |
| udp     | rustls | quic  | quic               |


## License
[The MIT License (MIT)](https://github.com/aa51513/roma/blob/master/LICENSE)
