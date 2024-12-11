use cs453getters::CS453Getters;

#[derive(CS453Getters)]
pub struct MyStruct {
    pub pub_num: u64,
    private_num: u64,
}

fn main() {
    let val = MyStruct { pub_num: 42, private_num: 0 };
    assert_eq!(val.pub_num_cs453(), &42);
    println!("Test passed!");
}
