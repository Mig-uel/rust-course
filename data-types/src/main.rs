fn main() {
      let i32_var: i32 = 1_337; // 32-bit signed integer

      let f32_var = i32_var as i16; // Casting i32 to i16

      let float_var: f32 = 3.143; // f64 by default
      println!("{:.3}", float_var); // Print float with 3 decimal places

      let with_milk = false; // boolean type
      let with_sugar: bool = true; // explicit boolean type

      let is_my_type_of_coffee = with_milk && with_sugar; // logical AND operation

      let is_acceptable_offer = with_milk || with_sugar; // logical OR operation

      let arr: [i8; 4] = [1, 2, 3, 4]; // array of 4 i8 elements
      println!("{:?}", arr);

      let tuple: (i8, f32, bool, [i8; 4]) = (42, 3.14, true, arr); // tuple with different types
      println!("{:?}", tuple);
}