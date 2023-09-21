use wasm_bindgen::prelude::*;

// pub struct OwnerID {
//    id: usize,
// }

#[wasm_bindgen]
pub struct Car {
   pub number: usize,
   pub color:usize, // color in hex code
   // pub boxed_value: Box<usize>,
   // pub owner: OwnerID,
}

#[wasm_bindgen]
pub fn color(a: Car, color: usize ) -> Car {
   Car {
      number: a.number,
      color,
      // boxed_value:Box::new(0),
      // owner: OwnerID { id: 0 }
   }
}

/*
If we take a look at the console, we can see that Car is a class and color is a function. However, we cannot directly instantiate Car as a class using new Car() because inherently, it is a Rust struct and does not possess a constructor per se.

Thus, to instantiate it, we must add the new method to the Car class and expose it as well:
*/

#[wasm_bindgen]
impl Car {
   pub fn new() -> Self {
      Car { 
         number: 0, 
         color: 0
         // owner: OwnerID { id: 0 },
      }
   }
   pub fn duplicate(&self) -> Self {
      Self {
         number: self.number + 1,
         color: self.color,
      }
   }
   pub fn change_number(&mut self,number:usize) {
      self.number = number;      
   }
   // pub fn get_id(&self) -> usize {
   //    self.owner.id
   // }
}

#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> usize {
   a + b
}

// #[wasm_bindgen]
// extern "C" {
//    fn alert(s: &str);
//    #[wasm_bindgen(js_name = alert)]
//    fn alert_usize(a: usize);
// }
// #[wasm_bindgen]
// pub fn greet() {
//    alert("Hello in JS from rust!");
//    alert_usize(5);
// }

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
