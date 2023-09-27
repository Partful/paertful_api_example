// Import necessary serde and serde_json libraries
use serde::{Deserialize, Serialize};
use serde_json::Result;

// Define a struct named FullPart
struct FullPart {
    part: Part,         // A field named 'part' of type 'Part' struct
    shopping: Shopping, // A field named 'shopping' of type 'Shopping' struct
}

// Define a struct named 'Part' with serialization and deserialization support
#[derive(Serialize, Deserialize)]
pub struct Part {
    number: String,                     // A field to store a part number as a String
    name: String,                       // A field to store a part name as a String
    description: String,                // A field to store a part description as a String
    notes: String,                      // A field to store part notes as a String
    customAttributes: CustomAttributes, // A field to store custom attributes of type 'CustomAttributes'
}

// Define a struct named 'CustomAttributes' with serialization and deserialization support
#[derive(Serialize, Deserialize)]
pub struct CustomAttributes {
    material: String, // A field to store material information as a String
    sources: String,  // A field to store sources information as a String
}

// Define a struct named 'Shopping' with serialization and deserialization support
#[derive(Serialize, Deserialize)]
pub struct Shopping {
    price: usize,         // A field to store the price of the part as a usize
    availability: bool,   // A field to store availability information as a bool
    priceOnRequest: bool, // A field to store price-on-request information as a bool
}

impl FullPart {
    // Define a constructor method for the 'FullPart' struct
    pub fn new() {
        // Constructor is currently empty and does not initialize any values
        // You can add logic here to initialize 'FullPart' fields if needed
    }
}
