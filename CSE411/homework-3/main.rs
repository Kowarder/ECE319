#![feature(test)]
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::sync::Barrier;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use threadpool::ThreadPool;
extern crate test;
use test::Bencher;
use rand::Rng;


fn gaussian_elimination(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>,String> {
    let n = b.len();

    // Forward Elimination (Eliminate variables below the diagonal)
    for pivot_row in 0..n {
        // Find the pivot element
        let mut pivot_index = pivot_row;
        for i in (pivot_row + 1)..n {
            if a[i][pivot_row].abs() > a[pivot_index][pivot_row].abs() {
                pivot_index = i;
            }
        }

        // Swap rows if necessary
        if pivot_index != pivot_row {
            a.swap(pivot_row, pivot_index);
            b.swap(pivot_row, pivot_index);
        }

        let pivot_value = a[pivot_row][pivot_row];

        if pivot_value == 0.0 {
            return Err("Matrix is singular or poorly conditioned.".to_string());
        }

        // Normalize the pivot row
        for j in pivot_row..n {
            a[pivot_row][j] /= pivot_value;
        }
        b[pivot_row] /= pivot_value;

        // Eliminate variables below the pivot row
        for i in (pivot_row + 1)..n {
            let factor = a[i][pivot_row];
            for j in pivot_row..n {
                a[i][j] -= factor * a[pivot_row][j];
            }
            b[i] -= factor * b[pivot_row];
        }
    }

    // Backward Substitution (Solve for variables)
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
    }

    Ok(x)
}

fn guess_thread(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = b.len();
    let thread_num = 4;
    let pool = ThreadPool::new(thread_num);

    // Forward Elimination 
    for pivot_row in 0..n {
        
        let mut pivot_index = pivot_row;
        for i in (pivot_row + 1)..n {
            if a[i][pivot_row].abs() > a[pivot_index][pivot_row].abs() {
                pivot_index = i;
            }
        }

        // Swap rows if necessary
        if pivot_index != pivot_row {
            a.swap(pivot_row, pivot_index);
            b.swap(pivot_row, pivot_index);
        }

        let pivot_value = a[pivot_row][pivot_row];

        if pivot_value == 0.0 {
            return Err("Matrix is singular or poorly conditioned.".to_string());
        }
        // Normalize the pivot row
        for j in pivot_row..n {
            a[pivot_row][j] /= pivot_value;
        }
        b[pivot_row] /= pivot_value;

        // Using Arc and Mutex for shared access across threads
        let a_shared = Arc::new(Mutex::new(a));
        let b_shared = Arc::new(Mutex::new(b));

        
        for i in (pivot_row + 1)..n {
            let a_arc_clone = a_shared.clone();
            let b_arc_clone = b_shared.clone();

            pool.execute(move || {
                let mut a_lock = a_arc_clone.lock().unwrap();
                let mut b_lock = b_arc_clone.lock().unwrap();
                let factor = a_lock[i][pivot_row];
                for j in pivot_row..n {
                    a_lock[i][j] -= factor * a_lock[pivot_row][j];
                }
                b_lock[i] -= factor * b_lock[pivot_row];
            });
        }

        // Ensure all tasks in the pool are completed before proceeding.
        pool.join();
        a = a_shared.lock().unwrap().clone();
        b = b_shared.lock().unwrap().clone();
    }

    // Backward Substitution 
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
    }

    Ok(x)
}

fn guess_thread_back(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = b.len();

    // Forward Elimination
    for pivot_row in 0..n {
        let mut pivot_index = pivot_row;
        for i in (pivot_row + 1)..n {
            if a[i][pivot_row].abs() > a[pivot_index][pivot_row].abs() {
                pivot_index = i;
            }
        }

        // Swap rows if necessary
        if pivot_index != pivot_row {
            a.swap(pivot_row, pivot_index);
            b.swap(pivot_row, pivot_index);
        }

        let pivot_value = a[pivot_row][pivot_row];

        if pivot_value == 0.0 {
            return Err("Matrix is singular or poorly conditioned.".to_string());
        }
        // Normalize the pivot row
        for j in pivot_row..n {
            a[pivot_row][j] /= pivot_value;
        }
        b[pivot_row] /= pivot_value;
        // Splitting the matrix into chunks of 2 rows each
        let chunk_size = 2;
        let num_chunks = (n - pivot_row - 1 + chunk_size - 1) / chunk_size;

        let (tx, rx) = mpsc::channel();
        
        for chunk in 0..num_chunks {
            let start = pivot_row + 1 + chunk * chunk_size;
            let end = std::cmp::min(start + chunk_size, n);

            let a_clone = a.clone();
            let b_clone = b.clone();
            let tx = tx.clone();

            thread::spawn(move || {
                let mut local_a = a_clone;
                let mut local_b = b_clone;
                
                for i in start..end {
                    let factor = local_a[i][pivot_row];
                    for j in pivot_row..n {
                        local_a[i][j] -= factor * local_a[pivot_row][j];
                    }
                    local_b[i] -= factor * local_b[pivot_row];
                }

                tx.send((local_a, local_b, start, end)).unwrap();
            });
        }

        for _ in 0..num_chunks {
            let (local_a, local_b, start, end) = rx.recv().unwrap();
            for i in start..end {
                a[i] = local_a[i].clone();
                b[i] = local_b[i];
            }
        }
    }

    // Backward Substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
    }

    Ok(x)
}


fn guess_rayon_map(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = b.len();
    
    for pivot_row in 0..n {
        
        let mut pivot_index = pivot_row;
        for i in (pivot_row + 1)..n {
            if a[i][pivot_row].abs() > a[pivot_index][pivot_row].abs() {
                pivot_index = i;
            }
        }

        // Swap rows if necessary
        if pivot_index != pivot_row {
            a.swap(pivot_row, pivot_index);
            b.swap(pivot_row, pivot_index);
        }

        let pivot_value = a[pivot_row][pivot_row];
        if pivot_value == 0.0 {
            return Err("Matrix is singular or poorly conditioned.".to_string());
        }

        // Normalize the pivot row
        for j in pivot_row..n {
            a[pivot_row][j] /= pivot_value;
        }
        b[pivot_row] /= pivot_value;

        let pivot_row_a = a[pivot_row].clone();

        // Compute the row modifications in parallel and collect them
        let changes: Vec<_> = (pivot_row + 1..n).into_par_iter().map(|i| {
            let factor = a[i][pivot_row];
            let row_change: Vec<_> = (pivot_row..n).map(|j| factor * pivot_row_a[j]).collect();
            let b_change = factor * b[pivot_row];
            (i, row_change, b_change)
        }).collect();

        // Apply the computed changes
        for (i, row_change, b_change) in changes {
            for (j, change) in row_change.into_iter().enumerate() {
                a[i][pivot_row + j] -= change;
            }
            b[i] -= b_change;
        }
    }
    // Backward Substitution (Solve for variables)
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
    }

    Ok(x)
}

fn guess_rayon_map_threadpool(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = b.len();
    let thread_num = 4;
    
    
    for pivot_row in 0..n {
        
        let mut pivot_index = pivot_row;
        for i in (pivot_row + 1)..n {
            if a[i][pivot_row].abs() > a[pivot_index][pivot_row].abs() {
                pivot_index = i;
            }
        }

        // Swap rows if necessary
        if pivot_index != pivot_row {
            a.swap(pivot_row, pivot_index);
            b.swap(pivot_row, pivot_index);
        }

        let pivot_value = a[pivot_row][pivot_row];
        if pivot_value == 0.0 {
            return Err("Matrix is singular or poorly conditioned.".to_string());
        }

        // Normalize the pivot row
        for j in pivot_row..n {
            a[pivot_row][j] /= pivot_value;
        }
        b[pivot_row] /= pivot_value;

        let pivot_row_a = a[pivot_row].clone();
        let pool = ThreadPoolBuilder::new().num_threads(thread_num).build().unwrap();

        // Compute the row modifications in parallel and collect them
        let changes: Vec<_> = pool.install(|| {
            (pivot_row + 1..n).into_par_iter().map(|i| {
                let factor = a[i][pivot_row];
                let row_change: Vec<_> = (pivot_row..n).map(|j| factor * pivot_row_a[j]).collect();
                let b_change = factor * b[pivot_row];
                (i, row_change, b_change)
            }).collect()
        });


        // Apply the computed changes
        for (i, row_change, b_change) in changes {
            for (j, change) in row_change.into_iter().enumerate() {
                a[i][pivot_row + j] -= change;
            }
            b[i] -= b_change;
        }
    }
    // Backward Substitution (Solve for variables)
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
    }

    Ok(x)
}



// #[bench]
// fn thread_4_256(b:&mut Bencher) {
//     let mut a:Vec<Vec<f64>> = Vec::new();
//     let mut r:Vec<f64> = Vec::new();
//     let mut random = rand::thread_rng();
//     for _ in 0..256 {
//         let mut v:Vec<f64> = Vec::new();
//         for _ in 0..256 {
//             v.push(random.gen_range(-50.0, 50.0));
//         }
//         a.push(v);
//     }
//     for _ in 0..256 {
//         r.push(random.gen_range(-50.0, 50.0));
//     }

//     b.iter(||{
//         guess_thread(a.clone(), r.clone());
//     })
// }
#[bench]
fn thread_4_1024(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..1024 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..1024 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..1024 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_thread(a.clone(), r.clone());
    })
}
#[bench]
fn thread_4_2048(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..2048 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..2048 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..2048 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_thread(a.clone(), r.clone());
    })
}

// #[bench]
// fn rayon_map_threadpool_4_512(b:&mut Bencher) {
//     let mut a:Vec<Vec<f64>> = Vec::new();
//     let mut r:Vec<f64> = Vec::new();
//     let mut random = rand::thread_rng();
//     for _ in 0..512 {
//         let mut v:Vec<f64> = Vec::new();
//         for _ in 0..512 {
//             v.push(random.gen_range(-50.0, 50.0));
//         }
//         a.push(v);
//     }
//     for _ in 0..512 {
//         r.push(random.gen_range(-50.0, 50.0));
//     }

//     b.iter(||{
//         guess_rayon_map_threadpool(a.clone(), r.clone());
//     })
// }
// #[bench]
// fn rayon_map_threadpool_4_2048(b:&mut Bencher) {
//     let mut a:Vec<Vec<f64>> = Vec::new();
//     let mut r:Vec<f64> = Vec::new();
//     let mut random = rand::thread_rng();
//     for _ in 0..2048 {
//         let mut v:Vec<f64> = Vec::new();
//         for _ in 0..2048 {
//             v.push(random.gen_range(-50.0, 50.0));
//         }
//         a.push(v);
//     }
//     for _ in 0..2048 {
//         r.push(random.gen_range(-50.0, 50.0));
//     }

//     b.iter(||{
//         guess_rayon_map_threadpool(a.clone(), r.clone());
//     })
// }
// #[bench]
// fn rayon_map_threadpool_4_4096(b:&mut Bencher) {
//     let mut a:Vec<Vec<f64>> = Vec::new();
//     let mut r:Vec<f64> = Vec::new();
//     let mut random = rand::thread_rng();
//     for _ in 0..4096 {
//         let mut v:Vec<f64> = Vec::new();
//         for _ in 0..4096 {
//             v.push(random.gen_range(-50.0, 50.0));
//         }
//         a.push(v);
//     }
//     for _ in 0..4096 {
//         r.push(random.gen_range(-50.0, 50.0));
//     }

//     b.iter(||{
//         guess_rayon_map_threadpool(a.clone(), r.clone());
//     })
// }




fn main() {
    // Example usage:
    let a = vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ];
    let b = vec![8.0, -11.0, -3.0];

    let solution = gaussian_elimination(a.clone(), b.clone());
    let solution2 = guess_thread(a.clone(), b.clone());
    let solution3 = guess_rayon_map(a.clone(), b.clone());
    let solution4 = guess_rayon_map_threadpool(a.clone(), b.clone());
    let solution5 = guess_thread_back(a.clone(), b.clone());
    

    println!("Solution: {:?}", solution);
 
    println!("solution thread: {:?}", solution2);
    println!("solution rayon_map: {:?}", solution3);
    println!("solution rayon_map_threadpool: {:?}", solution4);
    println!("solution thread_back: {:?}", solution5);
    
}