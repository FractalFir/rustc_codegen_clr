#[test]
fn should_pass() {}
#[test]
#[should_panic]
fn should_panic() {
    panic!();
}
fn main() {
    select_nth_unstable();
}

fn select_nth_unstable() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    use rand::seq::SliceRandom;
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut v = [0; 500];
    for pivot in 0..v.len() {
        println!("{}:{}", file!(), line!());
        v.select_nth_unstable_by(pivot, |_, _| {
            *[Less, Equal, Greater].choose(&mut rng).unwrap()
        });
        v.sort();
        for i in 0..v.len() {
            println!("{}:{}", file!(), line!());
            assert_eq!(v[i], i as i32);
        }
    }

    for len in (2..21).chain(500..501) {
        let mut orig = vec![0; len];
        println!("{}:{}", file!(), line!());
        for &modulus in &[5, 10, 1000] {
            for _ in 0..10 {
                println!("{}:{}", file!(), line!());
                for i in 0..len {
                    println!("{}:{}", file!(), line!());
                    orig[i] = rng.gen::<i32>() % modulus;
                }

                let v_sorted = {
                    let mut v = orig.clone();
                    v.sort();
                    v
                };

                // Sort in default order.
                for pivot in 0..len {
                    println!("{}:{}", file!(), line!());
                    let mut v = orig.clone();
                    v.select_nth_unstable(pivot);

                    assert_eq!(v_sorted[pivot], v[pivot]);
                    for i in 0..pivot {
                        for j in pivot..len {
                            assert!(v[i] <= v[j]);
                        }
                    }
                }

                // Sort in ascending order.
                for pivot in 0..len {
                    println!("{}:{}", file!(), line!());
                    let mut v = orig.clone();
                    let (left, pivot, right) = v.select_nth_unstable_by(pivot, |a, b| a.cmp(b));

                    assert_eq!(left.len() + right.len(), len - 1);

                    for l in left {
                        assert!(l <= pivot);
                        for r in right.iter_mut() {
                            assert!(l <= r);
                            assert!(pivot <= r);
                        }
                    }
                }

                // Sort in descending order.
                let sort_descending_comparator = |a: &i32, b: &i32| b.cmp(a);
                let v_sorted_descending = {
                    let mut v = orig.clone();
                    v.sort_by(sort_descending_comparator);
                    v
                };

                for pivot in 0..len {
                    println!("{}:{}", file!(), line!());
                    let mut v = orig.clone();
                    v.select_nth_unstable_by(pivot, sort_descending_comparator);

                    assert_eq!(v_sorted_descending[pivot], v[pivot]);
                    for i in 0..pivot {
                        for j in pivot..len {
                            assert!(v[j] <= v[i]);
                        }
                    }
                }
            }
        }
    }

    // Sort at index using a completely random comparison function.
    // This will reorder the elements *somehow*, but won't panic.
    let mut v = [0; 500];
    for i in 0..v.len() {
        println!("{}:{}", file!(), line!());
        v[i] = i as i32;
    }

    // Should not panic.
    [(); 10].select_nth_unstable(0);
    [(); 10].select_nth_unstable(5);
    [(); 10].select_nth_unstable(9);
    [(); 100].select_nth_unstable(0);
    [(); 100].select_nth_unstable(50);
    [(); 100].select_nth_unstable(99);

    let mut v = [0xDEADBEEFu64];
    v.select_nth_unstable(0);
    assert!(v == [0xDEADBEEF]);
    println!("v:{v:?}");
}
