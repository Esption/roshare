// Copyright 2016 Esption
// 
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate seqlock;

use std::marker::Copy;
use self::seqlock::SeqLock;

pub struct RoSeqLock<T: Copy> {
	rwl: SeqLock<T>
}
impl<T: Copy> RoSeqLock<T> {
	#[inline]
	pub fn new(t: T) -> Self {
		RoSeqLock { rwl: SeqLock::new(t) }
	}
	#[inline]
	pub fn from(rwl: SeqLock<T>) -> Self {
		RoSeqLock { rwl: rwl }
	}
	#[inline]
	pub fn read(&self) -> T {
		self.rwl.read()
	}
}

