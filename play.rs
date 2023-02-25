
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod contract{
   
    #[ink(storage)]
   pub struct Calculator{
       value: u8
   }
    
  impl Calculator{

    #[ink(message)]
     pub fn add_n(&mut self, dand: u8) {
        self.value = self.value + dand;
     }

     #[ink(message)]
     pub fn sub_n(&mut self, dsub: u8) {
       self.value = if self.value > dsub {
            self.value - dsub
        }else{
            dsub - self.value
        }
     }
    
     #[ink(message)]
     pub fn get_value(&self) -> u8 {
        return self.value;
     }

     #[ink(constructor)]
     pub fn new(initial_value: u8) -> Self{
        return Self{value: initial_value};
     }
    
     #[ink(constructor)]
     pub fn default() -> Self{
        return Self::new(Default::default());
     }
  }

  #[cfg(test)]
   mod tests {
      use super::*;
      
      #[ink::test]
      fn default_works(){
        let calculator: Calculator = Calculator::default();
        assert_eq!(calculator.get_value(), 0);
      }
      
      #[ink::test]
      fn it_works(){
        let mut calculator: Calculator = Calculator::new(6);
        calculator.add_n(4);
        calculator.sub_n(2);
        assert_eq!(calculator.get_value(), 8);
      }
   }
}