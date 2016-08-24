#[macro_use] extern crate lazy_static;

#[cfg(test)]
#[allow(unused_variables, non_upper_case_globals)]
mod tests {
    use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
    use std::sync::{Mutex, MutexGuard};
    use std::ops::{Drop};

    static global_id: AtomicIsize = ATOMIC_ISIZE_INIT;
    lazy_static! {
        static ref mutex : Mutex<()> = Mutex::new( () );
    }
    fn reset_counter_and_begin<'a>() -> MutexGuard<'a, ()> {
        let l = mutex.lock();
        global_id.store(0, Ordering::SeqCst);
        l.unwrap()
    }

    fn inc() -> isize {
        global_id.fetch_add(1, Ordering::SeqCst)
    }
    fn get() -> isize {
        global_id.load(Ordering::SeqCst)
    }


    //#[derive(Copy)]
    struct Counter {
        c:  isize,
        id: isize,
    }

    impl Counter {
        fn new() -> Counter {
            Counter {
                c: 0,
                id: inc(),
            }
        }
        fn dummy_borrow(& self) {  }
        fn dummy_mut(&mut self) { self.id += 0; }
    }

    impl Clone for Counter {
        fn clone(&self) -> Counter {
            Counter {
                id: inc(),
                c: self.c + 1,
            }
        }
    }

    //*
    impl Copy for Counter {
    }

    /*/
    impl Drop for Counter {
        fn drop(&mut self) {
            println!("Count({id}): {count}", count=self.c, id=self.id);
        }
    } // */

    #[test]
    fn it_works() {
        let l = reset_counter_and_begin();
        {
            let c = Counter::new();
            c.dummy_borrow();
        }
        assert_eq!(1, get());
    }

    #[test]
    fn iterate_let() {
        let l = reset_counter_and_begin();
        {
            let v = vec![ Counter::new() ];

            for e in v {
                e.dummy_borrow();
            }
        }
        assert_eq!(1, get())
    }

    #[test]
    fn iterate_let_mut() {
        let l = reset_counter_and_begin();
        {
            let v = vec![ Counter::new() ];

            for mut e in v {
                e.dummy_mut();
            }
        }
        assert_eq!(1, get())
    }

    #[test]
    fn iterate_let_mut_push() {
        let l = reset_counter_and_begin();
        {
            let c = Counter::new();
            let mut v = vec![ Counter::new() ];
            v.push(c);

            for mut e in v {
                e.dummy_mut();
            }
        }
        assert_eq!(2, get())
    }

    #[test]
    fn clone() {
        let l = reset_counter_and_begin();
        let o = Counter::new();
        let mut c = o.clone();

        c.id = 42;

        assert_eq!(0, o.c);
        assert_eq!(1, c.c);

        assert_eq!(0, o.id);
        assert_eq!(42, c.id);

        assert_eq!(2, get());
    }

    #[test]
    /// Copy does not involve clone
    fn copy() {
        let l = reset_counter_and_begin();
        let o = Counter::new();
        let mut c = o;

        c.id = 42;

        assert_eq!(0, o.c);
        assert_eq!(0, c.c);

        assert_eq!(0, o.id);
        assert_eq!(42, c.id);

        assert_eq!(1, get());
    }


}
