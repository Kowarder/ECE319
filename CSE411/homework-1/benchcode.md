# 100-1000-10000
#[bench]
fn vec_0_100(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        v.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _j in 0..50 {
            v.insert(random.gen_range(0, 100));
            v.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn vec_20_100(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        v.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..20 {
            v.find(random.gen_range(0, 100));
        }
        for _j in 0..40 {
            v.insert(random.gen_range(0, 100));
            v.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn vec_50_100(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        v.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..50 {
            v.find(random.gen_range(0, 100));
        }
        for _j in 0..25 {
            v.insert(random.gen_range(0, 100));
            v.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn vec_80_100(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        v.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..80 {
            v.find(random.gen_range(0, 100));
        }
        for _j in 0..10 {
            v.insert(random.gen_range(0, 100));
            v.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn vec_100_100(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        v.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..100 {
            v.find(random.gen_range(0, 100));
        }
    })
}
#[bench]
fn list_0_100(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        l.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _j in 0..50 {
            l.insert(random.gen_range(0, 100));
            l.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn list_20_100(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        l.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..20 {
            l.find(random.gen_range(0, 100));
        }
        for _j in 0..40 {
            l.insert(random.gen_range(0, 100));
            l.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn list_50_100(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        l.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..50 {
            l.find(random.gen_range(0, 100));
        }
        for _j in 0..25 {
            l.insert(random.gen_range(0, 100));
            l.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn list_80_100(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        l.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..80 {
            l.find(random.gen_range(0, 100));
        }
        for _j in 0..10 {
            l.insert(random.gen_range(0, 100));
            l.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn list_100_100(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        l.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..100 {
            l.find(random.gen_range(0, 100));
        }
    })
}
#[bench]
fn tree_0_100(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        t.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _j in 0..50 {
            t.insert(random.gen_range(0, 100));
            t.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn tree_20_100(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        t.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..20 {
            t.find(random.gen_range(0, 100));
        }
        for _j in 0..40 {
            t.insert(random.gen_range(0, 100));
            t.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn tree_50_100(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        t.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..50 {
            t.find(random.gen_range(0, 100));
        }
        for _j in 0..25 {
            t.insert(random.gen_range(0, 100));
            t.remove(random.gen_range(0, 100));
        }
        
    })
}#[bench]
fn tree_80_100(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        t.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..80 {
            t.find(random.gen_range(0, 100));
        }
        for _j in 0..10 {
            t.insert(random.gen_range(0, 100));
            t.remove(random.gen_range(0, 100));
        }
        
    })
}#[bench]
fn tree_100_100(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        t.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..100 {
            t.find(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn hash_0_100(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        h.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _j in 0..50 {
            h.insert(random.gen_range(0, 100));
            h.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn hash_20_100(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        h.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..20 {
            h.find(random.gen_range(0, 100));
        }
        for _j in 0..40 {
            h.insert(random.gen_range(0, 100));
            h.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn hash_50_100(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        h.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..50 {
            h.find(random.gen_range(0, 100));
        }
        for _j in 0..25 {
            h.insert(random.gen_range(0, 100));
            h.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn hash_80_100(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        h.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..80 {
            h.find(random.gen_range(0, 100));
        }
        for _j in 0..10 {
            h.insert(random.gen_range(0, 100));
            h.remove(random.gen_range(0, 100));
        }
        
    })
}
#[bench]
fn hash_100_100(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100 {
        h.insert(random.gen_range(0, 100));
    }
    b.iter(|| {
        for _k in 0..100 {
            h.find(random.gen_range(0, 100));
        }
    })
}
test hash_0_100   ... bench:       3,809 ns/iter (+/- 38)
test hash_100_100 ... bench:       2,603 ns/iter (+/- 14)
test hash_20_100  ... bench:       3,679 ns/iter (+/- 36)
test hash_50_100  ... bench:       3,607 ns/iter (+/- 73)
test hash_80_100  ... bench:       3,454 ns/iter (+/- 42)
test list_0_100   ... bench:       5,487 ns/iter (+/- 94)
test list_100_100 ... bench:       1,278 ns/iter (+/- 8)
test list_20_100  ... bench:       4,681 ns/iter (+/- 43)
test list_50_100  ... bench:       3,394 ns/iter (+/- 48)
test list_80_100  ... bench:       2,134 ns/iter (+/- 37)
test tree_0_100   ... bench:       4,273 ns/iter (+/- 42)
test tree_100_100 ... bench:       2,989 ns/iter (+/- 24)
test tree_20_100  ... bench:       4,160 ns/iter (+/- 44)
test tree_50_100  ... bench:       3,780 ns/iter (+/- 14)
test tree_80_100  ... bench:       3,518 ns/iter (+/- 373)
test vec_0_100    ... bench:       3,267 ns/iter (+/- 47)
test vec_100_100  ... bench:       1,276 ns/iter (+/- 10)
test vec_20_100   ... bench:       2,869 ns/iter (+/- 138)
test vec_50_100   ... bench:       2,274 ns/iter (+/- 15)
test vec_80_100   ... bench:       1,695 ns/iter (+/- 18)

#[bench]
fn vec_0_1000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        v.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _j in 0..500 {
            v.insert(random.gen_range(0, 1000));
            v.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn vec_20_1000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        v.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..200 {
            v.find(random.gen_range(0, 1000));
        }
        for _j in 0..400 {
            v.insert(random.gen_range(0, 1000));
            v.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn vec_50_1000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        v.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..500 {
            v.find(random.gen_range(0, 1000));
        }
        for _j in 0..250 {
            v.insert(random.gen_range(0, 1000));
            v.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn vec_80_1000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        v.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..800 {
            v.find(random.gen_range(0, 1000));
        }
        for _j in 0..100 {
            v.insert(random.gen_range(0, 1000));
            v.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn vec_100_1000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        v.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..1000 {
            v.find(random.gen_range(0, 1000));
        }
    })
}
#[bench]
fn list_0_1000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        l.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _j in 0..500 {
            l.insert(random.gen_range(0, 1000));
            l.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn list_20_1000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        l.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..200 {
            l.find(random.gen_range(0, 1000));
        }
        for _j in 0..400 {
            l.insert(random.gen_range(0, 1000));
            l.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn list_50_1000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        l.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..500 {
            l.find(random.gen_range(0, 1000));
        }
        for _j in 0..250 {
            l.insert(random.gen_range(0, 1000));
            l.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn list_80_1000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        l.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..800 {
            l.find(random.gen_range(0, 1000));
        }
        for _j in 0..100 {
            l.insert(random.gen_range(0, 1000));
            l.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn list_100_1000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        l.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..1000 {
            l.find(random.gen_range(0, 1000));
        }
    })
}
#[bench]
fn tree_0_1000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        t.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _j in 0..500 {
            t.insert(random.gen_range(0, 1000));
            t.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn tree_20_1000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        t.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..200 {
            t.find(random.gen_range(0, 1000));
        }
        for _j in 0..400 {
            t.insert(random.gen_range(0, 1000));
            t.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn tree_50_1000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        t.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..500 {
            t.find(random.gen_range(0, 1000));
        }
        for _j in 0..250 {
            t.insert(random.gen_range(0, 1000));
            t.remove(random.gen_range(0, 1000));
        }
        
    })
}#[bench]
fn tree_80_1000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        t.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..800 {
            t.find(random.gen_range(0, 1000));
        }
        for _j in 0..100 {
            t.insert(random.gen_range(0, 1000));
            t.remove(random.gen_range(0, 1000));
        }
        
    })
}#[bench]
fn tree_100_1000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        t.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..1000 {
            t.find(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn hash_0_1000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        h.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _j in 0..500 {
            h.insert(random.gen_range(0, 1000));
            h.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn hash_20_1000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        h.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..200 {
            h.find(random.gen_range(0, 1000));
        }
        for _j in 0..400 {
            h.insert(random.gen_range(0, 1000));
            h.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn hash_50_1000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        h.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..500 {
            h.find(random.gen_range(0, 1000));
        }
        for _j in 0..250 {
            h.insert(random.gen_range(0, 1000));
            h.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn hash_80_1000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        h.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..800 {
            h.find(random.gen_range(0, 1000));
        }
        for _j in 0..100 {
            h.insert(random.gen_range(0, 1000));
            h.remove(random.gen_range(0, 1000));
        }
        
    })
}
#[bench]
fn hash_100_1000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..1000 {
        h.insert(random.gen_range(0, 1000));
    }
    b.iter(|| {
        for _k in 0..1000 {
            h.find(random.gen_range(0, 1000));
        }
    })
}
test hash_0_1000   ... bench:      29,517 ns/iter (+/- 1,055)
test hash_100_1000 ... bench:      22,545 ns/iter (+/- 491)
test hash_20_1000  ... bench:      29,508 ns/iter (+/- 842)
test hash_50_1000  ... bench:      27,189 ns/iter (+/- 232)
test hash_80_1000  ... bench:      26,165 ns/iter (+/- 138)
test list_0_1000   ... bench:     398,519 ns/iter (+/- 17,866)
test list_100_1000 ... bench:       9,162 ns/iter (+/- 316)
test list_20_1000  ... bench:     330,929 ns/iter (+/- 22,742)
test list_50_1000  ... bench:     206,705 ns/iter (+/- 11,959)
test list_80_1000  ... bench:      87,884 ns/iter (+/- 4,900)
test tree_0_1000   ... bench:      57,134 ns/iter (+/- 2,300)
test tree_100_1000 ... bench:      35,913 ns/iter (+/- 987)
test tree_20_1000  ... bench:      53,290 ns/iter (+/- 1,067)
test tree_50_1000  ... bench:      47,701 ns/iter (+/- 807)
test tree_80_1000  ... bench:      41,772 ns/iter (+/- 929)
test vec_0_1000    ... bench:     141,201 ns/iter (+/- 2,638)
test vec_100_1000  ... bench:       9,119 ns/iter (+/- 132)
test vec_20_1000   ... bench:     116,860 ns/iter (+/- 3,618)
test vec_50_1000   ... bench:      76,228 ns/iter (+/- 1,334)
test vec_80_1000   ... bench:      35,729 ns/iter (+/- 1,866)

-----------------------------------------------------------
test hash_0_10000   ... bench:     406,672 ns/iter (+/- 7,683)
test hash_100_10000 ... bench:     335,910 ns/iter (+/- 4,134)
test hash_20_10000  ... bench:     391,880 ns/iter (+/- 9,769)
test hash_50_10000  ... bench:     373,991 ns/iter (+/- 8,896)
test hash_80_10000  ... bench:     354,903 ns/iter (+/- 8,879)
test list_0_10000   ... bench:  92,096,787 ns/iter (+/- 9,139,367)
test list_100_10000 ... bench:     181,481 ns/iter (+/- 4,421)
test list_20_10000  ... bench:  73,942,629 ns/iter (+/- 8,606,293)
test list_50_10000  ... bench:  44,812,750 ns/iter (+/- 5,672,614)
test list_80_10000  ... bench:  18,461,420 ns/iter (+/- 1,584,605)
test tree_0_10000   ... bench:     847,475 ns/iter (+/- 9,715)
test tree_100_10000 ... bench:     587,966 ns/iter (+/- 30,753)
test tree_20_10000  ... bench:     795,900 ns/iter (+/- 7,146)
test tree_50_10000  ... bench:     716,847 ns/iter (+/- 7,960)
test tree_80_10000  ... bench:     629,410 ns/iter (+/- 5,226)
test vec_0_10000    ... bench:  12,379,845 ns/iter (+/- 160,569)
test vec_100_10000  ... bench:     181,292 ns/iter (+/- 1,496)
test vec_20_10000   ... bench:   9,942,004 ns/iter (+/- 176,591)
test vec_50_10000   ... bench:   6,297,033 ns/iter (+/- 88,469)
test vec_80_10000   ... bench:   2,650,197 ns/iter (+/- 38,215)