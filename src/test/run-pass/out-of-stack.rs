// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-fast

#[feature(asm)];

use std::io::Process;
use std::os;
use std::str;

// lifted from the test module
pub fn black_box<T>(dummy: T) { unsafe { asm!("" : : "r"(&dummy)) } }

fn silent_recurse() {
    let buf = [0, ..1000];
    black_box(buf);
    silent_recurse();
}

fn loud_recurse() {
    println!("hello!");
    loud_recurse();
}

fn main() {
    let args = os::args();
    if args.len() > 1 && args[1].as_slice() == "silent" {
        silent_recurse();
    } else if args.len() > 1 && args[1].as_slice() == "loud" {
        loud_recurse();
    } else {
        let silent = Process::output(args[0], [~"silent"]).unwrap();
        assert!(!silent.status.success());
        let error = str::from_utf8_lossy(silent.error);
        assert!(error.as_slice().contains("has overflowed its stack"));

        let loud = Process::output(args[0], [~"loud"]).unwrap();
        assert!(!loud.status.success());
        let error = str::from_utf8_lossy(silent.error);
        assert!(error.as_slice().contains("has overflowed its stack"));
    }
}
