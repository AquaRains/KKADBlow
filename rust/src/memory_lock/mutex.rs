use std::hint;
use std::sync::{
    Mutex,
    MutexGuard,
};

#[allow(dead_code)]
pub fn mutex_lock<T, F, R>(m: &Mutex<T>, f: F) -> R
where
    F: FnOnce(&mut T) -> R,
{
    let result: R;

    loop {
        match m.try_lock().as_mut() {
            Ok(mut lock) => {
                result = f(&mut lock);
                break;
            }

            Err(_) => hint::spin_loop()
        }
    }
    result
}

#[allow(dead_code)]
pub fn get_mutex_lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    loop {
        match m.try_lock() {
            Ok(lock) => return lock,
            Err(_) => hint::spin_loop()
        }
    }
}