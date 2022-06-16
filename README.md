# ![Emerita](./assets/logo.png)

[Emerita](https://en.wikipedia.org/wiki/Emerita_(crustacean)) is a lightweight CNI plugin written in Rust.
It aims to be fast not only in terms of exectution time, but in terms of container to container performance too.
To do this is uses eBPF programs built and loaded with [Aya](https://aya-rs.dev).

## Prerequisites

1. Install a rust stable toolchain: `rustup install stable`
1. Install a rust nightly toolchain: `rustup install nightly`
1. Install bpf-linker: `cargo install bpf-linker`
1. [CNI Plugins](https://github.com/containernetworking/plugins/releases/tag/v1.1.1) installed in `/opt/cni/bin`

## Build eBPF

```bash
cargo xtask build-ebpf
```

## Build Userspace

```bash
cargo build
```

### Executing Manually

1. `cargo build`
2. `sudo ip netns create test`
3. Create a file called `config.json` with the following contents:
```json
{"cniVersion":"1.0.0","name":"emerita","type":"emerita","ipMasq":true,"ipam":{"type":"host-local","routes":[{"dst":"0.0.0.0/0"}],"ranges":[[{"gateway":"10.88.0.1","subnet":"10.88.0.0/16"}]]}}
```
4. `cargo build`
5. `sudo ./target/debug/emerita add -n /var/run/netns/test -c foo -i eth0 < config.json`

You can then remove the netns and `emerita0` interface to revert to normal.

### Execting using CNI

1. `git clone https://github.com/containernetworking/cni`
2. `sudo mkdir -p /opt/cni/bin`
3. `sudo cp ./target/debug/emerita /opt/cni/bin`
4. Create `/etc/cni/net.d/50-emerita.conf` with the following contents:
```json
{
  "cniVersion": "1.0.0",
  "name": "emerita",
  "plugins": [
    {
      "type": "emerita",
      "ipMasq": true,
      "ipam": {
        "type": "host-local",
        "routes": [{ "dst": "0.0.0.0/0" }],
        "ranges": [
          [
            {
              "subnet": "10.88.0.0/16",
              "gateway": "10.88.0.1"
            }
          ]
        ]
      }
    }
  ]
}
```
5. `sudo ip netns create test`
6. `sudo ./scripts/exec-plugins.sh add ctr12345 /var/run/netns/test`

## License

## emerita-ebpf

Code in the `emerita-ebpf` crate is distributed under the terms of the [GNU General Public License, Version 2].

## Other crates

Code in all other crates is distributed under the terms of either the [MIT license] or the [Apache License] (version 2.0), at your option.


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[MIT license]: LICENSE-MIT
[Apache license]: LICENSE-APACHE
[GNU General Public License, Version 2]: LICENSE-GPL
