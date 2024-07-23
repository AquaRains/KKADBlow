use std::hint;
use std::sync::{
    RwLock,
    RwLockWriteGuard,
};

#[allow(dead_code)]
pub fn rw_write_lock<T, F, R>(l: &RwLock<T>, f: F) -> R
where F: FnOnce(&mut T) -> R {
    let result: R;
    loop {
        match l.try_write().as_mut() {
            Ok(mut lock) => {
                result = f(&mut lock);
                //drop(lock);
                break;
            },
            Err(_) => hint::spin_loop()
        }
    }
    result
}

#[allow(dead_code)]
pub fn get_rw_write_lock<T>(l: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    loop {
        match l.try_write() {
            Ok(lock) => return lock,
            Err(_) => hint::spin_loop()
        }
    }
}