// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct C {
    x: uint,
}

fn C(x: uint) -> C {
    C {
        x: x
    }
}

fn f<T:Copy>(_x: T) {
}

#[deny(non_implicitly_copyable_typarams)]
pub fn main() {
    f(C(1u));
}
