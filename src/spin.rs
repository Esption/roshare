// Copyright 2016 Esption
// 
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate spin;

use std::ops::Deref;
use self::spin::{Mutex, MutexGuard, RwLock as SpinRwLock, RwLockReadGuard};

pub struct RoMutex<T> {
	m: Mutex<T>
}
pub struct RoMutexGuard<'a, T: 'a> {
	g: MutexGuard<'a, T>
}
impl<'a, T> Deref for RoMutexGuard<'a, T> {
	type Target = T;
	
	#[inline]
	fn deref(&self) -> &T {
		&self.g
	}
}
impl<T> RoMutex<T> {
	#[inline]
	pub fn new(t: T) -> Self {
		RoMutex { m: Mutex::new(t) }
	}
	#[inline]
	pub fn from(mutex: Mutex<T>) -> Self {
		RoMutex { m: mutex }
	}
	#[inline]
	pub fn read<'a>(&'a self) -> RoMutexGuard<'a, T> {
		RoMutexGuard { g: self.m.lock() }
	}
	#[inline]
	pub fn try_read<'a>(&'a self) -> Option<RoMutexGuard<'a, T>> {
		self.m.try_lock().map(|e| RoMutexGuard { g: e })
	}
}

pub struct RoLock<T> {
	rwl: SpinRwLock<T>
}
impl<T> RoLock<T> {
	#[inline]
	pub fn new(t: T) -> Self {
		RoLock { rwl: SpinRwLock::new(t) }
	}
	#[inline]
	pub fn from(rwl: SpinRwLock<T>) -> Self {
		RoLock { rwl: rwl }
	}
	#[inline]
	pub fn read(&self) -> RwLockReadGuard<T> {
		self.rwl.read()
	}
	#[inline]
	pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
		self.rwl.try_read()
	}
}

