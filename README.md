Naive implementation of Peano arithmetic solely in rust's type system. Peano arithmetic is essentially a simple but powerful way of defining the natural numbers (0, 1, 2, 3 etc), where we define natural numbers inductively as either Zero or Successor of a natural number. 
For more information: [Peano Axioms](https://en.wikipedia.org/wiki/Peano_axioms)

For example: 


```
type One = Succ<Zero>;
type Two = Succ<One>;
type Three = Succ<Two>; 

<One as Add<Two>> == Three; //These types are equal, but this line of code is not valid rust; see next line for how to directly compare the types.

assert_eq!(std::any::TypeId::of::<<One as Add<Two>>::Result>(), std::any::TypeId::of::<Three>());  //Checking the types are equal.

<One as Mul<Two>> == Two //We can also use multiplication!



```

There is also a macro to easily create the desired integer types:

```
type One = Succ<Zero>;
type OneHundred = int_to_type!(100); //expands to Succ<Succ...<Zero>>>... with 100 Succs, which represents the number 100.

```

Q: Why is this useful? Aren't we just making the compiler do slow math while compiling?

A: Yes, this implemention is incredibly slow for any calculation that isn't with tiny integers, and was written to play around with Rust's type system. For an efficient implementation, you can look at the [typenum](https://github.com/paholg/typenum) crate. The crate that first got me interested in this was the [dimensioned](https://github.com/paholg/dimensioned) crate, which uses type level arithmetic to give you dimensional safety and tell you off at compile time when you try and add 5 metres to 20 seconds.