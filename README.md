Playing around with Rust's type system, implemented basic Peano arithmetic (only up to addition, so technically only Presburger arithmetic), and some simple boolean functions solely in rust's type system. Todo: Add multiplication to peano, and implement a type-level modular arithmetic.

For example: 


```
type One = Succ<Zero>;
type Two = Succ<One>;
type Three = Succ<Two>;

<One as Add<Two>> == Three; //These types are equal, but this line of code is not valid rust; see next line 

assert_eq!(std::any::TypeId::of::<<One as Add<Two>>::Result>(), std::any::TypeId::of::<Three>());  //How you can directly compare the types to check the arithmetic.

```

There is also some traits to compare the value of the types at runtime (ToInt and ToBool), but these aren't really needed since you can directly compare the types.