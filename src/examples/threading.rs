extern crate rayon;

use examples::threading::rayon::iter::IntoParallelRefMutIterator;
use examples::threading::rayon::iter::ParallelIterator;

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

        println!("Sum no multithreading: {}", sum);
    }

    {
        // Serial foreach, no multitreading
        let mut v = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        v.iter_mut().for_each(|x| {
            sum += *x;
        });

        println!("Sum serial, no multi: {}", sum);
    }

    {
        // Serial foreach, YES multithreading
        let mut v = vec![1, 2, 3, 4, 5];

        let mut sum_vulgaris = 0;
        let mut sum: RwLock<i32> = RwLock::new(0);

        v.par_iter_mut().for_each(|x| {

            // This does not work!
            // sum_vulgaris += *x;

            let mut temp_sum = sum.write().unwrap();
            *temp_sum += *x;
            dbg!(*temp_sum);
        });

        println!("Sum serial, multi-thread safe with RwLock: {}", sum.read().unwrap());
    }
}
