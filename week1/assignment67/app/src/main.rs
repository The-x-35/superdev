use serialize_macro::{SerializeNumberStruct, DeserializeNumberStruct};
use serialize_macro_traits::{Serialize, Deserialize};

#[derive(SerializeNumberStruct, DeserializeNumberStruct)]
struct Swap {
    name: String,
    qty_2: usize,
    qty_3: i8
}

fn main() {
    println!("Testing custom derive macro with String support:");
    
    let s1 = Swap {
        name: "Hello".to_string(),
        qty_2: 2,
        qty_3: 100
    };
    let bytes1 = s1.serialize();
    println!("Test 1 - Simple string:");
    println!("  Original: name='{}', qty_2={}, qty_3={}", s1.name, s1.qty_2, s1.qty_3);
    println!("  Serialized bytes: {:?}", bytes1);
    
    match Swap::deserialize(&bytes1) {
        Ok(deserialized) => {
            println!("  Deserialized: name='{}', qty_2={}, qty_3={}", 
                deserialized.name, deserialized.qty_2, deserialized.qty_3);
        }
        Err(e) => println!("  Deserialization failed: {:?}", e),
    }
    
    let s2 = Swap {
        name: "".to_string(),
        qty_2: 0,
        qty_3: 0
    };
    let bytes2 = s2.serialize();
    println!("\nTest 2 - Empty string:");
    println!("  Original: name='{}', qty_2={}, qty_3={}", s2.name, s2.qty_2, s2.qty_3);
    println!("  Serialized bytes: {:?}", bytes2);
    
    match Swap::deserialize(&bytes2) {
        Ok(deserialized) => {
            println!("  Deserialized: name='{}', qty_2={}, qty_3={}", 
                deserialized.name, deserialized.qty_2, deserialized.qty_3);
        }
        Err(e) => println!("  Deserialization failed: {:?}", e),
    }
    
    let s3 = Swap {
        name: "Hello, World! 🌍 你好 こんにちは".to_string(),
        qty_2: 42,
        qty_3: -50
    };
    let bytes3 = s3.serialize();
    println!("\nTest 3 - Long string with Unicode:");
    println!("  Original: name='{}', qty_2={}, qty_3={}", s3.name, s3.qty_2, s3.qty_3);
    println!("  Serialized bytes: {:?}", bytes3);
    
    match Swap::deserialize(&bytes3) {
        Ok(deserialized) => {
            println!("  Deserialized: name='{}', qty_2={}, qty_3={}", 
                deserialized.name, deserialized.qty_2, deserialized.qty_3);
        }
        Err(e) => println!("  Deserialization failed: {:?}", e),
    }
}
