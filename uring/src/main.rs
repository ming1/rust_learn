use anyhow::Result;
use io_uring::{cqueue, opcode, squeue, types, IoUring};
use std::alloc::{alloc, dealloc, Layout};
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::{fs, io};

#[macro_use]
extern crate nix;

// Generate ioctl function
const BLKGETSIZE64_CODE: u8 = 0x12; // Defined in linux/fs.h
const BLKGETSIZE64_SEQ: u8 = 114;
ioctl_read!(ioctl_blkgetsize64, BLKGETSIZE64_CODE, BLKGETSIZE64_SEQ, u64);

fn file_size(f: &std::fs::File) -> Result<u64> {
    if let Ok(meta) = f.metadata() {
        if meta.file_type().is_block_device() {
            let fd = f.as_raw_fd();
            let mut cap = 0u64;
            let cap_ptr = &mut cap as *mut u64;

            unsafe {
                ioctl_blkgetsize64(fd, cap_ptr).unwrap();
            }

            Ok(cap)
        } else if meta.file_type().is_file() {
            Ok(f.metadata().unwrap().len())
        } else {
            Err(anyhow::anyhow!("unsupported file"))
        }
    } else {
        Err(anyhow::anyhow!("no file meta got"))
    }
}

const BS: usize = 4096;
const QD: usize = 128;

fn alloc_buf(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 4096).unwrap();
    unsafe { alloc(layout) as *mut u8 }
}

fn dealloc_buf(ptr: *mut u8, size: usize) {
    let layout = Layout::from_size_align(size, 4096).unwrap();
    unsafe { dealloc(ptr as *mut u8, layout) };
}

fn main() -> io::Result<()> {
    let p = std::env::args().nth(1).expect("no file path given");
    let mut ring = IoUring::<squeue::Entry, cqueue::Entry>::builder()
        .setup_cqsize(128)
        .build(128)?;

    println!("uring params {:?},  file path {}", ring.params(), p);
    let fd = fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECT)
        .open(p)?;

    let file_size = file_size(&fd).unwrap() & (!BS as u64);

    let mut ios = Vec::<*mut u8>::new();
    for _i in 0..QD {
        ios.push(alloc_buf(BS));
    }

    let mut off = 0_u64;
    let mut done = 0;
    let mut now = std::time::Instant::now();

    loop {
        for i in 0..QD {
            let sqe = &opcode::Read::new(types::Fd(fd.as_raw_fd()), ios[i], BS as u32)
                .offset(off)
                .build()
                .user_data(i as u64);
            unsafe {
                ring.submission()
                    .push(sqe)
                    .expect("submission queue is full");
            }
            off += BS as u64;
        }

        ring.submit_and_wait(QD)?;

        let cqes: Vec<cqueue::Entry> = ring.completion().map(Into::into).collect();

        assert_eq!(cqes.len(), QD);

        if off >= file_size {
            off = 0;
        }
        done += QD;

        if now.elapsed().as_secs() >= 1 {
            println!("read: BS {} IOPS {}K/sec", BS, done / 1000);
            done = 0;
            now = std::time::Instant::now();
        }
    }

    for i in 0..QD {
        dealloc_buf(ios[i], BS);
    }

    Ok(())
}
