#[bench]
fn thread_4_256(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..256 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..256 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..256 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_thread(a.clone(), r.clone());
    })
}
42202366ns
#[bench]
fn thread_4_512(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..512 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..512 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..512 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_thread(a.clone(), r.clone());
    })
}
247287545ns

#[bench]
fn rayon_map_threadpool_4_256(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..256 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..256 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..256 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_rayon_map_threadpool(a.clone(), r.clone());
    })
    
    
}
18564554ns

#[bench]
fn rayon_map_threadpool_4_512(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..512 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..512 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..512 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_rayon_map_threadpool(a.clone(), r.clone());
    })
}
92,587,337 ns/iter
#[bench]
fn rayon_map_threadpool_4_1024(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..512 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..512 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..512 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_rayon_map_threadpool(a.clone(), r.clone());
    })
}
105,169,879 ns/iter
#[bench]
fn rayon_map_threadpool_4_2048(b:&mut Bencher) {
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
        guess_rayon_map_threadpool(a.clone(), r.clone());
    })
}
3,844,208,587
#[bench]
fn rayon_map_threadpool_4_4096(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..4096 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..4096 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..4096 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_rayon_map_threadpool(a.clone(), r.clone());
    })
}

#[bench]
fn thread_8_512(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..512 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..512 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..512 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_thread(a.clone(), r.clone());
    })
}

#[bench]
fn rayon_map_threadpool_8_512(b:&mut Bencher) {
    let mut a:Vec<Vec<f64>> = Vec::new();
    let mut r:Vec<f64> = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..512 {
        let mut v:Vec<f64> = Vec::new();
        for _ in 0..512 {
            v.push(random.gen_range(-50.0, 50.0));
        }
        a.push(v);
    }
    for _ in 0..512 {
        r.push(random.gen_range(-50.0, 50.0));
    }

    b.iter(||{
        guess_rayon_map_threadpool(a.clone(), r.clone());
    })
}