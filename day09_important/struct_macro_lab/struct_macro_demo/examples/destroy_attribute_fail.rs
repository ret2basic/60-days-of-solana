use struct_macro_macros::destroy_attribute;

#[destroy_attribute]
struct MyStruct {
    baz: i32,
    qux: i32,
}

fn main() {
    let demo = MyStruct { baz: 3, qux: 4 };
    println!("struct is {:?}", demo);
}