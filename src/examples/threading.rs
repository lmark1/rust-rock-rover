extern crate rayon;

use examples::threading::rayon::iter::IntoParallelRefMutIterator;
use examples::threading::rayon::iter::ParallelIterator;

use std::sync::atomic::AtomicI32;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::RwLock;

/*
 * Magical multithreading aliasing examples.
 */
pub fn magical_multithreading_aliasing() {
    {
        // No multithreading
        let mut v = vec![1, 2, 3, 4, 5];
        for x in &mut v {
            *x += 1;
        }

        assert_eq!(v[0], 2);
        assert_eq!(v[4], 6);
    }

    {
        // Serial foreach, no multitreading
        let mut v = vec![1, 2, 3, 4, 5];
        v.iter_mut().for_each(|x| {
            *x += 1;
        });
        assert_eq!(v[0], 2);
        assert_eq!(v[4], 6);
    }

    {
        // Serial foreach, YES multithreading
        let mut v = vec![1, 2, 3, 4, 5];
        v.iter_mut().for_each(|x| {
            *x += 1;
        });
        assert_eq!(v[0], 2);
        assert_eq!(v[4], 6);
    }
}

/*
 * Tragical multithreading aliasing examples.
 * - modifiyng a local captured value is forbidden and wrong.
 */
#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn tragical_multithreading_aliasing() {
    {
        // No multithreading
        let mut v = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        for x in &mut v {
            sum += *x;
        }

        assert_eq!(sum, 15);
    }

    {
        // Serial foreach, no multitreading
        let mut v = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        v.iter_mut().for_each(|x| {
            sum += *x;
        });

        assert_eq!(sum, 15);
    }

    {
        // Serial foreach, YES multithreading
        let mut v = vec![1, 2, 3, 4, 5];

        let mut sum_vulgaris = 0;
        let mut sum = RwLock::new(0);
        v.par_iter_mut().for_each(|x| {
            // This does not work! Mutable borrow in parallel Fn
            // sum_vulgaris += *x;
        });
    }
}

/*
 * Thread synchronization using rwlock variables.
 */
#[allow(unused_variables)]
pub fn sync_shared_state_rwlock() {
    // Serial foreach, YES multithreading
    let mut v = vec![1, 2, 3, 4, 5];

    let sum_vulgaris = 0;
    let sum = RwLock::new(0);
    v.par_iter_mut().for_each(|x| {
        // This does not work!
        // sum_vulgaris += *x;

        let mut rwlock_sum = sum.write().unwrap();
        *rwlock_sum += *x;
        // dbg!(*rwlock_sum);
    });

    assert_eq!(*sum.read().unwrap(), 15);
}

/*
 * Thread synchronization using mutex variables.
 */
pub fn sync_shared_state_mutex() {
    let mut v = vec![1, 2, 3, 4, 5];

    let mut_sum = Mutex::new(0);
    v.par_iter_mut().for_each(|x| {
        let mut mutex_sum: MutexGuard<i32> = mut_sum.lock().unwrap();
        *mutex_sum += *x;
    });

    assert_eq!(*mut_sum.lock().unwrap(), 15);
}

/*
 * Thread synchronization using atomic variables.
 */
pub fn sync_shared_state_atomic() {
    let mut v = vec![1, 2, 3, 4, 5];

    let mut atomic_sum = AtomicI32::new(0);
    v.par_iter_mut().for_each(|x| {
        atomic_sum.fetch_add(*x, std::sync::atomic::Ordering::Relaxed);
    });

    assert_eq!(*atomic_sum.get_mut(), 15);
}
