# Introducing *f-infinity*
f-infinity is a new way of representing floating-point numbers. With the power of Rust, f-infinity is a powerful tool capable of representing numbers exactly.

## How to use it
For the moment, there is no specific library, but it can be used via **main.rs**There are 3 possible number choices at the moment: 
  - BigNumber : A way to store very big number
  - SmallNumber : A way to store very small number
  - RationalNumber : A way to store fraction

Each of these numbers can be created with, for example,

    let number = SmallNumber::new("2.3456");

Operations can be performed on these numbers:
  - BigNumber: Addition, Subtraction and Multiplication
  - SmallNumber: Addition, Subtraction and Multiplication
  - RationalNumber: Addition only for the moment


 ## Contributing
You can help the project by improving performance and adding new traits and functions to the numbers.
