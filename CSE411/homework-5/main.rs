use std::collections::HashMap;

fn main() {
    let data = vec![
        ("Alice", 50),
        ("Bob", 80),
        ("Charlie", 60),
        ("David", 75),
        ("Eve", 90),
        ("Frank", 55),
        ("Grace", 45),
        ("Hannah", 85),
        ("Ivy", 95),
        ("Jack", 40),
        ("test", 70)
    ];

    // Question 1
    // let greater:Vec<_> = data.iter().filter(|(x, y)|{
    //     y > &70
    // }).collect();
    // println!("{:?}", greater);

    // Question 2
    // data.iter().enumerate().for_each(|x| println!("{:?}", x));

    // Question 3
    // let remarks = vec!["Good", "Excellent", "Average", "Good", "Outstanding", "Fair", "Poor", "Very Good", "Exceptional", "Poor"];
    // let remark_for_stu: Vec<_> = data.iter().zip(remarks).filter_map(|((x, y), z)|{
    //     if y > &75 {
    //         Some((x, y, z))
    //     } else {None}
    // }).collect();
    // println!("{:?}", remark_for_stu);

    // Question 4
    // let data2 = vec![
    //     ("stu1", 90),
    //     ("stu2", 92),
    //     ("stu3", 94),
    //     ("stu4", 96),
    // ];
    // println!("The length of list after chain is {:?}", data.iter().chain(data2.iter()).count());

    // Question 5
    // let nested_data = vec![vec![data[0], data[1]], vec![data[2], data[3]], vec![data[4], data[5]]];
    // let first_five:Vec<_> = data.iter().filter_map(|x| Some(x)).take(5).collect();
    // println!("{:?}", first_five);
    // println!("{:?}", nested_data);

    //Question 6
    // let cycled_data = data.iter().cycle().take(15);
    // println!("{:?}", cycled_data);

    //Question 7
    // let all_above40 = data.iter().all(|(x, y)|
    //     y > &40
    // );
    // println!("{:?}", all_above40);

    // let any_above90 = data.iter().any(|(x, y)| 
    //     y > &90
    // );
    // println!("{:?}", any_above90);


let activities = vec![
    (1, vec!["Read", "Write", "Code"]),
    (2, vec!["Draw", "Paint"]),
    (3, vec!["Code", "Debug"]),
    (4, vec!["Paint", "Sculpt"]),
    (5, vec!["Read", "Research"]),
    (6, vec!["Write", "Blog"]),
    (7, vec!["Code", "Review"]),
    (8, vec!["Draw", "Design"]),
    (9, vec!["Research", "Experiment"]),
    (10, vec!["Sculpt", "Model"]),
    (11, vec!["Code"])
];
    //Question 8
    // let stu_act: Vec<_> = data.iter().zip(activities).filter_map(|((x, y), (a,b))| {
    //     if y > &60 {
    //         Some(((x, y), b))
    //     } else {
    //          None
    //     }
    // }).collect();
    // println!("{:?}", stu_act);

    //Question 9
    // let high_stu_act:Vec<_> = data.iter().zip(activities).filter_map(|((x, y), (a, b))| {
    //     if y > &70 {
    //         Some(b)
    //     } else {
    //         None
    //     }
    // }).collect();
    
    // let mut unique_act:Vec<_> = high_stu_act.iter().flat_map(|act| act.iter()).collect();
    // unique_act.sort();
    // unique_act.dedup();
    // println!("{:?}", unique_act);

    //Question 10
    // let mut flag = true;
    // let pairs:Vec<_> = data.iter().zip(activities).filter_map(|((x, y), (a, b))| {
    //     if flag {
    //         if *y < 50 {
    //             flag = false;
    //         }
    //         Some((x, y, b))
    //     } else {
    //         None
    //     }
    // }).collect();
    // println!("{:?}", pairs)

    //Question 11
    // let mut less_60 = false;
    // let mut over_90 = false;
    // let mut iterator = data.iter().cycle().skip_while(|&(x, y)| {
    //     if y < &60 {
    //         less_60 = true;
    //     }
    //     !less_60
    // }).take_while(|(x, y)| {
    //     if y > &90 {
    //         if over_90 {
    //             false
    //         } else {
    //             over_90 = true;
    //             true
    //         }
    //     } else {
    //         true
    //     }
    // });
    // println!("{:?}", iterator);


    //Question 12
    // let stu_act_set = data.iter().zip(activities.iter().enumerate());
    // let mut stu_map: HashMap<usize, (&str, i32, Vec<&str>)> = HashMap::new();

    // for (((name, score), (num, (id, act)))) in stu_act_set {
    //     stu_map.insert(*id, (name, *score, act.clone()));
    // }
    // println!("{:?}", stu_map);


    //Question 13
    let code_stu:Vec<_> = data.iter().zip(activities.iter()).filter_map(|((name, score), (id, act))| {
        if act.contains(&"Code"){
            if score > &65 {
                Some((name, score))
            } else {
                None
            }
        } else {
            None
        }
    }).collect();

    println!("{:?}", code_stu);

}
