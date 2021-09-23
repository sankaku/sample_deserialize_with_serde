use sample_deserialize_with_serde::jsons::{JSON1, JSON2, JSON3};

use sample_deserialize_with_serde::person1;
use sample_deserialize_with_serde::person2;
use sample_deserialize_with_serde::person3;
use sample_deserialize_with_serde::person4;
use sample_deserialize_with_serde::person5;
use sample_deserialize_with_serde::person6;

fn main() {
    // person1
    println!("[person1]");
    let p1_1: person1::Person = person1::json_to_person(JSON1).unwrap();
    let p1_2: person1::Person = person1::json_to_person(JSON2).unwrap();
    let p1_3: person1::Person = person1::json_to_person(JSON3).unwrap();

    println!("p1_1 = {:?}", p1_1);
    println!("p1_2 = {:?}", p1_2);
    println!("p1_3 = {:?}", p1_3);

    // person2
    println!("\n[person2]");
    let p2_1: person2::Person = person2::json_to_person(JSON1).unwrap();
    let p2_2: person2::Person = person2::json_to_person(JSON2).unwrap();
    let p2_3: person2::Person = person2::json_to_person(JSON3).unwrap();

    println!("p2_1 = {:?}", p2_1);
    println!("p2_2 = {:?}", p2_2);
    println!("p2_3 = {:?}", p2_3);

    // person3
    println!("\n[person3]");
    let p3_1: person3::Person = person3::json_to_person(JSON1).unwrap();
    // How to deserialize `null` as `MaybeCompany {value: None}` ?
    // let p3_2: person3::Person = person3::json_to_person(JSON2).unwrap();
    let p3_3: person3::Person = person3::json_to_person(JSON3).unwrap();

    println!("p3_1 = {:?}", p3_1);
    // println!("p3_2 = {:?}", p3_2);
    println!("p3_3 = {:?}", p3_3);

    // person4
    println!("\n[person4]");
    let p4_1: person4::Person = person4::json_to_person(JSON1).unwrap();
    let p4_2: person4::Person = person4::json_to_person(JSON2).unwrap();
    let p4_3: person4::Person = person4::json_to_person(JSON3).unwrap();

    println!("p4_1 = {:?}", p4_1);
    println!("p4_2 = {:?}", p4_2);
    println!("p4_3 = {:?}", p4_3);

    // person5
    println!("\n[person5]");
    let p5_1: person5::Person = person5::json_to_person(JSON1).unwrap();
    let p5_2: person5::Person = person5::json_to_person(JSON2).unwrap();
    let p5_3: person5::Person = person5::json_to_person(JSON3).unwrap();

    println!("p5_1 = {:?}", p5_1);
    println!("p5_2 = {:?}", p5_2);
    println!("p5_3 = {:?}", p5_3);

    // person6
    println!("\n[person6]");
    let p6_1: person6::Person = person6::json_to_person(JSON1).unwrap();
    let p6_2: person6::Person = person6::json_to_person(JSON2).unwrap();
    let p6_3: person6::Person = person6::json_to_person(JSON3).unwrap();

    println!("p6_1 = {:?}", p6_1);
    println!("p6_2 = {:?}", p6_2);
    println!("p6_3 = {:?}", p6_3);
}
