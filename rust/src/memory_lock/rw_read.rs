use std::hint;
use std::sync::{RwLock, RwLockReadGuard};

#[allow(dead_code)]
pub fn rw_read_lock<T, F, R>(l: &RwLock<T>, f: F) -> R
where
    F: FnOnce(&T) -> R,
{
    let r: R;
    loop {
        match l.try_read() {
            Ok(lock) => {
                r = f(&lock);
                drop(lock);
                break;
            }
            Err(_) => hint::spin_loop()
        }
    }
    r
}

#[allow(dead_code)]
pub fn get_rw_read_lock<T>(l: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    loop {
        match l.try_read() {
            Ok(lock) => return lock,
            Err(_) => hint::spin_loop()
        }
    }
}