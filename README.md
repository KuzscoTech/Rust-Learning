# Learning Rust <strr>

### First Learning Day: 
-----------------------------------------------------------------------------
Learning Primitive Types, print function, and creating custom print fn for each Struct needed. 
<br>
Learning for loops, and impl and if statements.

### Second Learning Day:
Learned Paramaterization of functions, secondary functions within functions, creating hex values that are converted to strings, and also utilized initialization of arrays and slices and how they work.

### Third Learning Day:
Learned Custom Types such as enum, structs, linkedlist implementation and use. How this works to create unique data structures. 

### Fourth Learning Day:
Variable bindings the variables are always unmuttable by default, and using the mut keyword allows us to make them mutable. Shadow binding has to do with scope where you can have multiple different scopes depending on what block your in. To create different scopes all you have to do is put them inside a {} statement.

Types: there are implicit and explicit types, mainly we use explicit because there is no implicit conversion for them. Also when you do conversions of different sizes we always reduce from the highest memory address. 
For Example: conversion u16 -> u8 means we lose the upper 8 bits.

Conversion: From and Into, From is used to change the defined object to the new primitive data type, while Into is the opposite where you take that specific data type and turn it into the one you are using. 

TryFrom and TryInto: Same things but instead of returning nothing if they fail, they can return a custom error that you specify.
FromStr and IntoStr: Used to convert strings to other data types, vice versa

parse().unwrap() -> take the string and translate it into another type such as integer
Example: let turbo_parsed = "10".parse::<i32>().unwrap();