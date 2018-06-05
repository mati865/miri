// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(exceeding_bitshifts)]
#![allow(const_err)]

fn main() {
    let _n = 2i64 << -1; //~ ERROR constant evaluation error [E0080]
    //~^ NOTE attempt to shift left with overflow
}
