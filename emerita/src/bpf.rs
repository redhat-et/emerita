// SPDX-License-Identifier: (Apache-2.0 OR MIT)
// Copyright Authors of Emerita

use aya::{
    include_bytes_aligned,
    maps::{MapRefMut, SockHash},
    programs::{SkMsg, SockOps},
    Bpf,
};
use aya_log::BpfLogger;
use emerita_common::SockKey;

fn bpf() -> Result<(), anyhow::Error> {
    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/emerita"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/emerita"
    ))?;

    let sock_ops: &mut SockOps = bpf.program_mut("sockops").unwrap().try_into()?;
    sock_ops.load()?;

    let pod1_cgroup = std::fs::File::open("/sys/fs/cgroup/system.slice/runc-pod1.scope")
        .map_err(Error::InvalidCgroup)?;
    sock_ops.attach(pod1_cgroup)?;
    let pod2_cgroup = std::fs::File::open("/sys/fs/cgroup/system.slice/runc-pod2.scope")
        .map_err(Error::InvalidCgroup)?;
    sock_ops.attach(pod2_cgroup)?;
    let pod3_cgroup = std::fs::File::open("/sys/fs/cgroup/system.slice/runc-pod3.scope")
        .map_err(Error::InvalidCgroup)?;
    sock_ops.attach(pod3_cgroup)?;

    let sock_map = SockHash::<MapRefMut, SockKey>::try_from(bpf.map_mut("TCP_CONNS")?)?;

    let redir: &mut SkMsg = bpf.program_mut("emerita").unwrap().try_into()?;
    redir.load()?;
    redir.attach(&sock_map)?;

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");
}
