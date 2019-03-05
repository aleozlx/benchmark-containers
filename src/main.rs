#[macro_use]
extern crate clap;
extern crate sysinfo;

use nix::Error;
use nix::unistd::*;
use nix::unistd::ForkResult::*;
use nix::sys::signal::*;
use nix::sys::wait::*;
use libc::_exit;
use sysinfo::{ProcessExt, SystemExt};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek};
use std::thread;
use std::time::Duration;

/// Multiply number of processes exponentially
#[allow(unconditional_recursion)]
fn forkbomb() -> ! {
    loop {
        if let Ok(_) = fork() {
            forkbomb();
        }
    }
}

/// Multiply number of threads exponentially
fn threadbomb() {
    loop {
        thread::spawn(move || { threadbomb(); });
    }
}

/// Consume virtual memory as quickly as possible
fn oom() {
    for tid in 0..8 {
        thread::spawn(move || {
            let mut chunks = Vec::new();
            loop {
                chunks.push(vec![0xccu8; tid<<20]);
            }
        });
    }
    loop { thread::sleep(Duration::from_millis(1)); }
}

/// Take and free all the physical memory over and over
fn mem() {
    let nproc = 8;
    let system = sysinfo::System::new();
    let total_mem = (system.get_total_memory()<<10) as usize;
    for _ in 0..nproc {
        thread::spawn(move || {
            loop{
                let mut chunks = Vec::new();
                loop {
                    if chunks.len()*(32<<20) < total_mem/nproc {
                        chunks.push(vec![0xccu8; 32<<20]);
                    }
                    else {
                        break;
                    }
                }
            }
        });
    }
    loop { thread::sleep(Duration::from_millis(1)); }
}

/// Maximize CPU utilization on all cores (user time)
///   (may be throttled by power management)
fn cpu() {
    let nproc = 640;
    for _ in 0..nproc {
        thread::spawn(move || {
            let mut rabbits = [43; 512];
            loop {
                for i in 0..rabbits.len()-1 {
                    rabbits[i+1] = (rabbits[i] * 7 + 13) % 13;
                }
            }
        });
    }
    loop { thread::sleep(Duration::from_millis(1)); }
}

/// Maximize block I/O
fn blk() {
    let mut b_in = vec![0u8; 20<<20].into_boxed_slice();
    let mut b_out = vec![97u8; 20<<20].into_boxed_slice();
    {
        let mut out = File::create("/tmp/benchmark.bin").unwrap();
        for _ in 0..((600<<20)/(20<<20)) {
            out.write(b_out.as_ref()).unwrap();
        }
    }
    thread::spawn(move || {
        loop {
            let mut f_in = OpenOptions::new().read(true).open("/dev/urandom").unwrap();
            for _ in 0..((600<<20)/(20<<20)) {
                f_in.read_exact(b_in.as_mut()).unwrap();
            }
        }
    });
    thread::spawn(move || {
        loop {
            let mut f_out = OpenOptions::new().write(true).open("/tmp/benchmark.bin").unwrap();
            for _ in 0..((600<<20)/(20<<20)) {
                f_out.write(b_out.as_ref()).unwrap();
                f_out.flush().unwrap();
            }
            for i in 0..b_out.len() {
                b_out[i] ^= 0xa5u8;
            }
            f_out.sync_data().unwrap();
        }
    });
    loop { thread::sleep(Duration::from_millis(1)); }
}

/// Maximize context switching (system time)
///   (usually not throttled by power management)
fn cs() {
    let nproc = (1<<16)-1;
    for tid in 0..nproc {
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(tid%16));
            }
        });
    }
    loop { thread::sleep(Duration::from_millis(1)); }
}

fn main() {
    let args = clap_app!(playbook =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: crate_description!())
            (@arg BENCHMARK: +required "type of benchmark to run")
        ).get_matches();

    match args.value_of("BENCHMARK").unwrap() {
        "forkbomb" => forkbomb(),
        // "threadbomb" => threadbomb(),
        "oom" => oom(),
        "mem" => mem(),
        "cpu" => cpu(),
        // "blk" => blk(),
        "cs" => cs(),
        _ => { println!("Valid options are: forkbomb, oom, mem, cpu, cs"); }
    }
}
