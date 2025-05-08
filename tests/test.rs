use reflect_rs::Reflection;

#[derive(Reflection)]
pub struct Person {
    name: String,
    age: i32,
}

// ===== Get Test ===== //

fn print_person_reflection(person: &impl Reflection) {
    let name = person.get_field("name");
    let age = person.get_field("age");

    println!("name: {:?}; age: {:?}", name, age);
}

#[test]
fn get_test() {
    let person = Person {
        name: "WiszeL".into(),
        age: 21,
    };
    print_person_reflection(&person);
}
