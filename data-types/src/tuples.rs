// Tuples
fn main() {
    let emp = ("Molly", 32, "Marketing");

    // let name = emp.0;
    // let age = emp.1;
    // let dept = emp.2;
    // println!("Name: {}, Age: {}, Department: {}", name, age, dept);

    let (name, age, dept) = emp;
    println!("Name: {}, Age: {}, Department: {}", name, age, dept);
}
