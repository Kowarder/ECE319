#![feature(linked_list_remove)]
#![feature(test)]
extern crate test;

use rand::seq::SliceRandom;
use rand::thread_rng;
use test::Bencher;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::vec::Vec;
use rand::Rng;
use std::io;




trait MySet {
    fn insert(&mut self, key: i32) -> bool;
    fn remove(&mut self, key: i32) -> bool;
    fn find(&self, key: i32) -> bool;
}

struct ArraySet {
    cache: Vec<i32>,
}

struct ListSet {
    cache: LinkedList<i32>,
}

struct TreeSet {
    cache: BTreeSet<i32>,
}
struct My_HashSet {
    cache: HashSet<i32>,
}




impl MySet for ArraySet {
    fn insert(&mut self, key: i32) -> bool {
        if self.find(key) {
            false
        } else {
            self.cache.push(key);
            true
        }
    }

    fn remove(&mut self, key: i32) -> bool {
        let mut i = 0;
        while i < self.cache.len() {
            if self.cache[i] == key {
                self.cache.remove(i);
                return true;
            } else {
                i += 1;
            }
            
        }
        false
    }

    fn find(&self, key: i32) -> bool {
        let mut i = 0;
        while i < self.cache.len() {
            if self.cache[i] == key {
                return true
            } else {
                i += 1;
            }
        }
        false
    }
}

impl MySet for ListSet {
    fn insert(&mut self, key: i32) -> bool {
        if self.find(key) {
            false
        } else {
            self.cache.push_back(key);
            true
        }
    }

    fn remove(&mut self, key: i32) -> bool {
        // let mut l = 0;
        // let mut iter = self.cache.iter();
        for (index, &k) in self.cache.iter().enumerate() {
            if k == key {
                self.cache.remove(index);
                return true
            }
        }
        // while l < self.cache.len() {
        //     if iter.next() == Some(key) {
        //         #[feature(linked_list_remove)]
        //         self.cache.remove(key.try_into().unwrap());
        //         return true

        //     } else {
        //         l += 1;
        //     }
        // }
        false
        
    }

    fn find(&self, key: i32) -> bool {
        if self.cache.contains(&key) {
            true
        } else {
            false
        }
    }
}

impl MySet for TreeSet {
    fn insert(&mut self, key: i32) -> bool {
        if self.cache.contains(&key) {
            false
        } else {
            self.cache.insert(key);
            true
        }
    }

    fn remove(&mut self, key: i32) -> bool {
        if self.cache.contains(&key) {
            self.cache.remove(&key);
            true
        } else {
            false
        }
    }

    fn find(&self, key: i32) -> bool {
        if self.cache.contains(&key) {
            true
        } else {
            false
        }
    }
}

impl MySet for My_HashSet {
    fn insert(&mut self, key: i32) -> bool {
        if self.cache.contains(&key) {
            false
        } else {
            self.cache.insert(key);
            true
        }
    }

    fn remove(&mut self, key: i32) -> bool {
        if self.cache.contains(&key) {
            self.cache.remove(&key);
            true
        } else {
            false
        }
    }

    fn find(&self, key: i32) -> bool {
        if self.cache.contains(&key) {
            true
        } else {
            false
        }
    }
}

// #[bench]
// fn vec_find(b:&mut Bencher) {

//     let mut v = ArraySet {
//         cache:Vec::new()
//     };
//     let mut random = rand::thread_rng();

//     b.iter(|| {
//         for i in 0..1_000 {
//             v.find(random.gen_range(0, 100));
//         }
//     })  
// }
#[bench]
fn vec_0_100000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        v.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _j in 0..5000 {
            v.insert(random.gen_range(0, 100000));
            v.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn vec_20_100000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        v.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..2000 {
            v.find(random.gen_range(0, 100000));
        }
        for _j in 0..4000 {
            v.insert(random.gen_range(0, 100000));
            v.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn vec_50_100000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        v.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..5000 {
            v.find(random.gen_range(0, 100000));
        }
        for _j in 0..2500 {
            v.insert(random.gen_range(0, 100000));
            v.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn vec_80_100000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        v.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..8000 {
            v.find(random.gen_range(0, 100000));
        }
        for _j in 0..1000 {
            v.insert(random.gen_range(0, 100000));
            v.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn vec_100_100000(b:&mut Bencher) {
    let mut v = ArraySet {
        cache:Vec::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        v.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..100000 {
            v.find(random.gen_range(0, 100000));
        }
    })
}
#[bench]
fn list_0_100000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        l.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _j in 0..5000 {
            l.insert(random.gen_range(0, 100000));
            l.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn list_20_100000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        l.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..2000 {
            l.find(random.gen_range(0, 100000));
        }
        for _j in 0..4000 {
            l.insert(random.gen_range(0, 100000));
            l.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn list_50_100000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        l.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..5000 {
            l.find(random.gen_range(0, 100000));
        }
        for _j in 0..2500 {
            l.insert(random.gen_range(0, 100000));
            l.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn list_80_100000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        l.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..8000 {
            l.find(random.gen_range(0, 100000));
        }
        for _j in 0..1000 {
            l.insert(random.gen_range(0, 100000));
            l.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn list_100_100000(b:&mut Bencher) {
    let mut l = ListSet {
        cache:LinkedList::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        l.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..100000 {
            l.find(random.gen_range(0, 100000));
        }
    })
}
#[bench]
fn tree_0_100000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        t.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _j in 0..5000 {
            t.insert(random.gen_range(0, 100000));
            t.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn tree_20_100000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        t.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..2000 {
            t.find(random.gen_range(0, 100000));
        }
        for _j in 0..4000 {
            t.insert(random.gen_range(0, 100000));
            t.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn tree_50_100000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        t.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..5000 {
            t.find(random.gen_range(0, 100000));
        }
        for _j in 0..2500 {
            t.insert(random.gen_range(0, 100000));
            t.remove(random.gen_range(0, 100000));
        }
        
    })
}#[bench]
fn tree_80_100000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        t.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..8000 {
            t.find(random.gen_range(0, 100000));
        }
        for _j in 0..1000 {
            t.insert(random.gen_range(0, 100000));
            t.remove(random.gen_range(0, 100000));
        }
        
    })
}#[bench]
fn tree_100_100000(b:&mut Bencher) {
    let mut t = TreeSet {
        cache:BTreeSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        t.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..100000 {
            t.find(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn hash_0_100000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        h.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _j in 0..5000 {
            h.insert(random.gen_range(0, 100000));
            h.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn hash_20_100000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        h.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..2000 {
            h.find(random.gen_range(0, 100000));
        }
        for _j in 0..4000 {
            h.insert(random.gen_range(0, 100000));
            h.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn hash_50_100000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        h.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..5000 {
            h.find(random.gen_range(0, 100000));
        }
        for _j in 0..2500 {
            h.insert(random.gen_range(0, 100000));
            h.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn hash_80_100000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        h.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..8000 {
            h.find(random.gen_range(0, 100000));
        }
        for _j in 0..1000 {
            h.insert(random.gen_range(0, 100000));
            h.remove(random.gen_range(0, 100000));
        }
        
    })
}
#[bench]
fn hash_100_100000(b:&mut Bencher) {
    let mut h = My_HashSet {
        cache:HashSet::new()
    };
    let mut random = rand::thread_rng();
    for _i in 0..100000 {
        h.insert(random.gen_range(0, 100000));
    }
    b.iter(|| {
        for _k in 0..100000 {
            h.find(random.gen_range(0, 100000));
        }
    })
}
// #[bench]
// fn vec_insert(b:&mut Bencher) {
//     let mut v = ArraySet {
//         cache:Vec::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             v.insert(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn vec_remove(b:&mut Bencher) {
//     let mut v = ArraySet {
//         cache:Vec::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             v.remove(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn link_find(b:&mut Bencher) {
//     let mut l = ListSet {
//         cache:LinkedList::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             l.find(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn link_insert(b:&mut Bencher) {
//     let mut l = ListSet {
//         cache:LinkedList::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             l.insert(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn link_remove(b:&mut Bencher) {
//     let mut l = ListSet {
//         cache:LinkedList::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             l.remove(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn tree_find(b:&mut Bencher) {
//     let mut t = TreeSet {
//         cache:BTreeSet::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             t.find(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn tree_insert(b:&mut Bencher) {
//     let mut t = TreeSet {
//         cache:BTreeSet::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             t.insert(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn tree_remove(b:&mut Bencher) {
//     let mut t = TreeSet {
//         cache:BTreeSet::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             t.remove(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn hash_find(b:&mut Bencher) {
//     let mut h = My_HashSet {
//         cache:HashSet::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             h.find(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn hash_insert(b:&mut Bencher) {
//     let mut h = My_HashSet {
//         cache:HashSet::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             h.insert(random.gen_range(0, 100));
//         }
//     })
// }
// #[bench]
// fn counter_hash_remove(b:&mut Bencher) {
//     let mut h = My_HashSet {
//         cache:HashSet::new()
//     };
//     let mut random = rand::thread_rng();
//     b.iter(|| {
//         for _i in 0..1_000 {
//             h.remove(random.gen_range(0, 100));
//         }
//     })
// }

#[test]
fn test_vec_remove() {
    let mut v = ArraySet {
        cache: Vec::new()
    };
    v.insert(1);
    v.insert(2);
    v.insert(3);
    v.remove(1);
    assert_eq!(v.cache[0], 2);
}
#[test]
fn test_vec_insert() {
    let mut v = ArraySet {
        cache: Vec::new()
    };
    v.insert(1);
    
    assert_eq!(v.cache[0], 1);
}
#[test]
fn test_list_insert() {
    let mut l = ListSet {
        cache: LinkedList::new()
    };
    l.insert(1);
    
    assert_eq!(l.cache.pop_back(), Some(1));
}
#[test]
fn test_list_remove() {
    let mut l = ListSet {
        cache: LinkedList::new()
    };
    l.insert(1);
    l.insert(2);
    l.insert(3);
    l.remove(3);
    
    assert_eq!(l.cache.pop_back(), Some(2));
}
#[test]
fn test_tree_insert() {
    let mut t = TreeSet {
        cache: BTreeSet::new()
    };
    t.insert(1);
    assert_eq!(t.cache.get(&1), Some(1).as_ref());
}
#[test]
fn test_tree_remove() {
    let mut t = TreeSet {
        cache: BTreeSet::new()
    };
    t.insert(1);
    t.insert(2);
    t.insert(3);
    t.remove(2);
    assert_eq!(t.cache.get(&2), None);
}
#[test]
fn test_hash_insert() {
    let mut h = My_HashSet {
        cache: HashSet::new()
    };
    h.insert(1);
    assert_eq!(h.cache.get(&1), Some(1).as_ref());
}
#[test]
fn test_hash_remove() {
    let mut h = My_HashSet {
        cache: HashSet::new()
    };
    h.insert(1);
    h.insert(2);
    h.insert(3);
    h.remove(1);
    assert_eq!(h.cache.get(&1), None);
}



fn main() {
    // let mut vset = Vec::<i64>::new();
    let mut vset = ArraySet {
        cache : Vec::new()
    };
    // vset.cache.push(1);
    // vset.cache.push(2);
    // vset.cache.push(3);
    // let mut rng = rand::thread_rng()
    let mut lset = ListSet {
        cache : LinkedList::new()
    };
    // lset.cache.push_back(1);
    // lset.cache.push_back(2);
    // lset.cache.push_back(3);
    let mut tset = TreeSet {
        cache : BTreeSet::new()
    };
    // tset.cache.insert(1, 1);
    // tset.cache.insert(2, 2);
    // tset.cache.insert(3, 3);
    let mut hset = My_HashSet {
        cache : HashSet::new()
    };
    // hset.cache.insert(1, 1);
    // hset.cache.insert(2, 2);
    // hset.cache.insert(3, 3);

    


    let mut i = String::new();
    println!("Please enter operations：");
    io::stdin().read_line(&mut i).expect("failed");
    let i:i32 = i.trim().parse().expect("Please enter the integer");

    let mut k = String::new();
    println!("Please enter the max_key_value:");
    io::stdin().read_line(&mut k).expect("failed");
    let k:i32 = k.trim().parse().expect("Please enter integer");

    let mut d = String::new();
    println!("Please enter data_structure(array, list, tree, or hashtable):");
    io::stdin().read_line(&mut d).expect("failed");

    let mut r = String::new();
    println!("please enter read-only ratio:");
    io::stdin().read_line(&mut r).expect("failed");
    let r:i32 = r.trim().parse().expect("Please enter integer");

    let mut random = rand::thread_rng();
    for _num in 0..(i/2) {
        let r = random.gen_range(0, 1000);
        vset.cache.push(r);
        lset.cache.push_back(r);
        tset.cache.insert(r);
        hset.cache.insert(r);
    }
    // match d {
    //     &"array" =>for a in 0..i {
    //                 let mut r = random.gen_range(0, 3);
    //                 // match r {
    //                 //     &0 => {vset.find(random.gen_range(0, k));}
    //                 // }
    //                 if r == 0 {
    //                     vset.find(random.gen_range(0, k));
    //                 }
    //         },
    // }
    let rest = (100 - r) / 2;

    let mut operation= Vec::<i32>::new();
    for a in 0..(i * r / 100) {
        operation.push(0);
    } 
    for b in 0..(i * rest / 100) {
        operation.push(1);
        operation.push(2);
    }
    let mut rng = thread_rng();
    operation.shuffle(&mut rng);


    if d == "array" {
        for a in 0..i {
            let r = random.gen_range(0, 3);
            match r {
                0 => {vset.find(random.gen_range(0, k));}
                1 => {vset.insert(random.gen_range(0, k));}
                2 => {vset.remove(random.gen_range(0, k));}
                _ => (),
            }
        }
    } else if d == "list" {
        for a in 0..i {
            let r = operation.pop().unwrap();
            match r {
                0 => {lset.find(random.gen_range(0, k));}
                1 => {lset.insert(random.gen_range(0, k));}
                2 => {lset.remove(random.gen_range(0, k));}
                _ => (),
            }
        }
    } else if d == "tree" {
        for a in 0..i {
            let r = operation.pop().unwrap();
            match r {
                0 => {hset.find(random.gen_range(0, k));}
                1 => {hset.insert(random.gen_range(0, k));}
                2 => {hset.remove(random.gen_range(0, k));}
                _ => (),
            }
        }
    } else if d == "hashtable"{
        for a in 0..i {
            let r = operation.pop().unwrap();
            match r {
                0 => {hset.find(random.gen_range(0, k));}
                1 => {hset.insert(random.gen_range(0, k));}
                2 => {hset.remove(random.gen_range(0, k));}
                _ => (),
            }
        }
    }
    
    
    // // vset.find(check);
    // println!("查询的结果是{}", vset.find(check));
    // println!("{:?}", vset.cache);
    // vset.insert(4);
    // println!("{:?}", vset.cache);
    // vset.remove(1);
    // println!("{:?}", vset.cache);


}
