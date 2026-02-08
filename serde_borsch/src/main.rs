use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    password: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
struct Car {
    name: String,
    model: String,
}

fn main() {
    // --------------------------- using serde -------------------------
    let u: User = User {
        name: String::from("tanmay"),
        password: String::from("2026"),
    };

    let serialize_str = serde_json::to_string(&u);

    let json_string = match serialize_str {
        Ok(s) => {
            println!("the string is: {}", s);
            s
        }
        Err(_) => {
            println!("error while converting string!");
            return;
        }
    };

    let deserialize_str: Result<User, serde_json::Error> = serde_json::from_str(&json_string);
    match deserialize_str {
        Ok(user) => println!("the deserialize string is: {:?}", user),
        Err(_) => println!("error while deserialization"),
    }

    // --------------------------- using borsh --------------------
    let c: Car = Car {
        name: String::from("bmw"),
        model: String::from("i8"),
    };

    let mut buffer: Vec<u8> = Vec::new();
    let result = c.serialize(&mut buffer);

    match result {
        Ok(_) => println!("the serialized data is: {:?}", buffer),
        Err(_) => println!("error while serializing"),
    }

    let deserialize = Car::try_from_slice(&buffer).unwrap();
    assert_eq!(c, deserialize);
    println!("done se and de using borsh: {:?}", deserialize)
}
