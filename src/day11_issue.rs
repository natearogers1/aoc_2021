// minimum reproducible example for issue during day11.
// Do some operation on a whole vector after having filtered down the vector to a smaller subset

pub fn run() {
    let mut og_vec = vec![
        MyData { v: 1 },
        MyData { v: 2 },
        MyData { v: 3 },
        MyData { v: 4 },
        MyData { v: 5 },
        MyData { v: 6 },
    ];
    issue1(&mut og_vec)
}

fn issue1(og_vec: &mut Vec<MyData>) {
    // get a vec of data with a certain criteria
    let subset_vec: Vec<&mut MyData> = og_vec.iter_mut().filter(|v| v.v % 2 == 0).collect();

    // try to do something on the original vec using the subset vec

    // nested for loop doesn't work,
    // line 21 for loop is a mutable borrow, but og_vec is borrowed immutably on line 14
    //for subset_item in subset_vec {
    //    for og_item in &mut og_vec {
    //       og_item.v = og_item.v * subset_item.v
    //    }
    //}

    for i in 0..subset_vec.len() {
        for j in 0..og_vec.len() {
            og_vec[j].v = og_vec[j].v * subset_vec[i].v
        }
    }
}
#[derive(Debug)]
struct MyData {
    v: u8,
}
