use anyhow::{bail, Result};

mod bpf;
use bpf::*;


fn bump_memlock_rlimit()->Result<()>{
    let rlimit = libc::rlimit {
        rlim_cur:128 <<20,
        rlim_max: 128 <<20,
    };
    if unsafe {libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit)}!=0 {
        bail!("Failed to increase rlimit")
    }
    Ok(())
}

fn main()->Result<()> {
    let _ = bump_memlock_rlimit();
    let exec_builder = ExecSkelBuilder::default();
    let mut exec = exec_builder.open()?.load()?;
    exec.attach()?;
    println!("hello world");

    let secs = std::time::Duration::from_secs(20);
    std::thread::sleep(secs);

    Ok(())
}
