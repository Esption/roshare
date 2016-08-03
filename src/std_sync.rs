// Copyright 2016 Esption
// 
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Deref;
use std::sync::{Mutex, MutexGuard, RwLock as StdRwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

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
		RoMutexGuard { g: self.m.lock().unwrap_or_else(|e| e.into_inner()) }
	}
	#[inline]
	pub fn try_read<'a>(&'a self) -> Option<RoMutexGuard<'a, T>> {
		match self.m.try_lock() {
			Ok(val) => Some(RoMutexGuard { g: val }),
			Err(err) => match err {
				TryLockError::Poisoned(p) => Some(RoMutexGuard { g: p.into_inner() }),
				_ => None
			}
		}
	}
}
/// The main reason this RwMutex exists is to wrap poison errors.
/// It's not very useful besides that.
pub struct RwMutex<T> {
	m: Mutex<T>
}
impl<T> RwMutex<T> {
	#[inline]
	pub fn new(t: T) -> Self {
		RwMutex { m: Mutex::new(t) }
	}
	#[inline]
	pub fn from(mutex: Mutex<T>) -> Self {
		RwMutex { m: mutex }
	}
	#[inline]
	pub fn lock(&self) -> MutexGuard<T> {
		self.m.lock().unwrap_or_else(|e| e.into_inner())
	}
	#[inline]
	pub fn try_lock(&self) -> Option<MutexGuard<T>> {
		match self.m.try_lock() {
			Ok(val) => Some(val),
			Err(err) => match err {
				TryLockError::Poisoned(p) => Some(p.into_inner()),
				_ => None
			}
		}
	}
}

pub struct RoLock<T> {
	rwl: StdRwLock<T>
}
impl<T> RoLock<T> {
	#[inline]
	pub fn new(t: T) -> Self {
		RoLock { rwl: StdRwLock::new(t) }
	}
	#[inline]
	pub fn from(rwl: StdRwLock<T>) -> Self {
		RoLock { rwl: rwl }
	}
	#[inline]
	pub fn read(&self) -> RwLockReadGuard<T> {
		self.rwl.read().unwrap_or_else(|e| e.into_inner())
	}
	#[inline]
	pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
		match self.rwl.try_read() {
			Ok(val) => Some(val),
			Err(err) => match err {
				TryLockError::Poisoned(p) => Some(p.into_inner()),
				_ => None
			}
		}
	}
}
/// The main reason this RwLock exists is to wrap poison errors.
/// It's not very useful besides that.
pub struct RwLock<T> {
	rwl: StdRwLock<T>
}
impl<T> RwLock<T> {
	#[inline]
	pub fn new(t: T) -> Self {
		RwLock { rwl: StdRwLock::new(t) }
	}
	#[inline]
	pub fn from(rwl: StdRwLock<T>) -> Self {
		RwLock { rwl: rwl }
	}
	#[inline]
	pub fn read(&self) -> RwLockReadGuard<T> {
		self.rwl.read().unwrap_or_else(|e| e.into_inner())
	}
	#[inline]
	pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
		match self.rwl.try_read() {
			Ok(val) => Some(val),
			Err(err) => match err {
				TryLockError::Poisoned(p) => Some(p.into_inner()),
				_ => None
			}
		}
	}
	#[inline]
	pub fn write(&self) -> RwLockWriteGuard<T> {
		self.rwl.write().unwrap_or_else(|e| e.into_inner())
	}
	#[inline]
	pub fn try_write(&self) -> Option<RwLockWriteGuard<T>> {
		match self.rwl.try_write() {
			Ok(val) => Some(val),
			Err(err) => match err {
				TryLockError::Poisoned(p) => Some(p.into_inner()),
				_ => None
			}
		}
	}
}

