use linked_list::ComputeNorm;
use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    let mut dup = list.clone();
    println!("\n====== Clone test ======");
    println!("{}", dup);
    println!("size: {}", dup.get_size());
    println!("{}", dup.to_string()); // ToString impl for anything impl Display

    println!("\n====== PartialEq test ======");
    assert!(dup == list);
    list.push_front(232);
    dup.push_front(233);
    assert!(dup != list);
    println!("passed");

    println!("\n====== Iterator & ComputeNorm test ======");
    list.pop_front();
    for val in &list {
        print!("{} ", val);
    }
    println!("");
    for val in dup {
        print!("{} ", val);
    }
    println!("");
    assert_eq!(
        list.compute_norm(),
        ((10 * (10 + 1) * (2 * 10 + 1) / 6) as f64).sqrt()
    );
    println!("passed");

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
