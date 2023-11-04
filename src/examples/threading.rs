extern crate rayon;

use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use std::sync::atomic::AtomicI32;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::RwLock;
use std::thread;

/*
 * Magical multithreading aliasing examples.
 */
pub fn magical_multithreading_aliasing() {
    {
        // No multithreading
        let mut v = [1, 2, 3, 4, 5];
        for x in &mut v {
            *x += 1;
        }

        assert_eq!(v[0], 2);
        assert_eq!(v[4], 6);
    }

    {
        // Serial foreach, no multitreading
        let mut v = [1, 2, 3, 4, 5];
        v.iter_mut().for_each(|x| {
            *x += 1;
        });
        assert_eq!(v[0], 2);
        assert_eq!(v[4], 6);
    }

    {
        // Serial foreach, YES multithreading
        let mut v = [1, 2, 3, 4, 5];
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
        let mut v = [1, 2, 3, 4, 5];
        let mut sum = 0;
        for x in &mut v {
            sum += *x;
        }

        assert_eq!(sum, 15);
    }

    {
        // Serial foreach, no multitreading
        let mut v = [1, 2, 3, 4, 5];
        let mut sum = 0;
        v.iter_mut().for_each(|x| {
            sum += *x;
        });

        assert_eq!(sum, 15);
    }

    {
        // Serial foreach, YES multithreading
        let mut v = [1, 2, 3, 4, 5];

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
    let mut v = [1, 2, 3, 4, 5];

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
    let mut v = [1, 2, 3, 4, 5];

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
    let mut v = [1, 2, 3, 4, 5];

    let mut atomic_sum = AtomicI32::new(0);
    v.par_iter_mut().for_each(|x| {
        atomic_sum.fetch_add(*x, std::sync::atomic::Ordering::Relaxed);
    });

    assert_eq!(*atomic_sum.get_mut(), 15);
}

/*
 * Arc mutex string example.
 */
#[allow(unused_must_use)]
pub fn freestyle_mutltithreading() {
    // let my_string: Mutex<String> = Mutex::new("abcd".to_string());
    let arc_mutex_string: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

    let mut thread_handles = Vec::new();

    for _ in 1..10 {
        // Need to first make a local clone of the shared_ptr in order to move it to
        // the thread. Moving arc_mutex_string to a thread would work for only
        // one thread.
        // The ownership of the arc_clone is transferred to the thread which
        // mentions its name.
        let arc_clone = arc_mutex_string.clone();

        let thread_handle = thread::spawn(move || {
            // let mut mut_guard = my_string.lock().unwrap();
            // mut_guard.push_str("some_characters");

            // Do the Arc!
            let mut guard = arc_clone.lock().unwrap();
            guard.push_str("some_chars");
        });

        thread_handles.push(thread_handle);
    }

    // Rust is worried about the following.
    // If the outer function somhow returns before joining threads (e.g. spawning
    // of a thread could totaly throw an exeption, in c++ if there is an exeption
    // in the thread,c++ calls std::terminate killing the entire program, depending on
    // the order of initialization if threads are declared after resources they
    // use then it's fine, but if they are declared before then it may be a
    // use-after-free before threads can actually terminate) then
    // my_string would be destroyed and its reference in the closure would
    // cause use-after-free which is a compiler error.
    //
    // WE know that join is happening and everything is fine. Rust doesn't.
    //
    // To circumvent this problem it's beneficial to use std::shared_ptr<..> in C++ and
    // capture it by VALUE in lambda [=], or use it's equivalent in Rust - Arc.
    // Similar to Arc, the Box is unique_ptr equivalent.

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }

    {
        dbg!(arc_mutex_string.lock().unwrap());
    }
}
