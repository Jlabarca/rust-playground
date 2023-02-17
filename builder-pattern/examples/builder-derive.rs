#[macro_use]
extern crate derive_builder;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct Person {
    name: String,
    age: u32,
    list: Vec<String>,
}

fn main() -> Result<(), ()> {
    let  person = PersonBuilder::default()
        .name( "John Doe" .to_string())
        .age( 42 )
        .add("test")
        .add_to_list("test2")
        .build();

    println!("{:?}",  person);
    Ok(())
}
