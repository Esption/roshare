## What this is
A wrapper around various RwLock / Mutex implementations to give the ability to have certain shares read-only.
For std::sync, this will also wrap poison errors, as my guess is most people just do `.unwrap_or_else(|e| e.into_inner())` anyway.

## What this supports
Currently,
* std::sync::{Mutex, RwLock}
* spin
* seqlock

If you want this to support any others then just open an issue for it.

## How to use

As of right now, std::sync has it's own types for writes (`roshare::std_sync::{RwLock, RwMutex}`) which will wrap the poison errors for writable mutex/rwlock.
Otherwise, everything has it's own RoLock / RoMutex (when applicable) and wraps the functions for them to enforce that no writes can happen.
Spin and seqlock are lacking their own RwLock / RwMutex, because there's no point in wrapping them.

