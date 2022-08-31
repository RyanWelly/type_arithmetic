Basic implementation of Peano arithmetic solely in rust's type system. Peano arithmetic means it defines all natural numbers inductively as either Zero or Successor of a natural number.
For more information: [Peano Axioms](https://en.wikipedia.org/wiki/Peano_axioms)

For example: 


```
type One = Succ<Zero>;
type Two = Succ<One>;
type Three = Succ<Two>;

<One as Add<Two>> == Three; //These types are equal, but this line of code is not valid rust; see next line 

assert_eq!(std::any::TypeId::of::<<One as Add<Two>>::Result>(), std::any::TypeId::of::<Three>());  //How you can directly compare the types to check the arithmetic.

<One as Mul<Two>> == Two //We can also use multiplication!

```

There is also some traits to compare the value of the types at runtime (ToInt and ToBool), but these aren't really needed since you can directly compare the types.


Q: Why is this useful? Aren't we just making the compiler do slow math while compiling?

A: Yes, this implemention is incredibly slow for any calculation that isn't with tiny integers, and was written to play around with Rust's type system. For an efficient implementation, you can look at the [typenum](https://github.com/paholg/typenum) crate. The crate that first got me interesting in this was the [dimensioned](https://github.com/paholg/dimensioned) crate, which uses type level arithmetic to give you dimensional safety and tell you off at compile time when you try and add 5 metres to 20 seconds.