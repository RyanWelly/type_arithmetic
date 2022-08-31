use std::{marker::PhantomData};
use crate::boolean::*;
mod boolean;
use seq_macro::seq;

struct Zero; // 0, the root of all evil (and all natural numbers!)

struct Succ<Nat> {
    _marker: PhantomData<Nat> //just needs some kind of marker so we can retain the generic type; as recommended by the rust compiler, we use PhantomData.
}


trait Add<Nat> {
    type Result;
}

//a + 0 = a for all nats 
impl<A> Add<A> for Zero {
    type Result = A;
}

//a + succ(b) = succ(a+b) for all nats a,b
impl<A,B: Add<A>> Add<A> for Succ<B> {
    type Result = Succ<<B as Add<A>>::Result>;
}

//Multiplication axioms;
// a * 0 = 0
// a * succ(b) = a + a * b


trait Mul<Nat> {
    type Result;
}

// a * 0 = 0
impl<A> Mul<A> for Zero {
    type Result = Zero;
}


// a * succ(b) = a + a * b
impl<A: Add<B>, B: Mul<A>> Mul<A> for Succ<B> where <B as Mul<A>>::Result: Add<A>{
    //The "where <B as Mul<A>>::Result: Add<A>" ensures that Rust knows we can add a * b to a; this is valid for all naturals a and b, 
    //so this is all we need to do to implemenent multiplication for all non-zero naturals.
    type Result = <<B as Mul<A>>::Result as Add<A>>::Result;
}





//Trait for runtime testing. Given we can directly compare the types, this is no longer neccessary. 
trait ToInt {
    fn to_int() -> u32;
}
impl ToInt for Zero {
    fn to_int() -> u32 { 
        0 
    }
}
impl<T: ToInt> ToInt for Succ<T> {
    fn to_int() -> u32 { 
        1 + T::to_int() 
    }
}





//Converts an positive integer into the type equivalent. For example, int_to_type!(3) expands to Succ<Succ<Succ<Zero>>>, otherwise known as THREE or 3.
macro_rules! int_to_type {
    ($e:expr) => { 
        seq!(N in 0..$e { #(Succ::<)* Zero #(>)*} ) };
}


//Things to note about the tests; Most of them work by comparing the TypeId of each type, which gives useless 
//error messages when they fail.

#[test]
fn test_peano() {
    type ZERO = Zero;
    type ONE = Succ<ZERO>;
    type TWO = Succ<ONE>;
    type THREE = Succ<TWO>;
    type FOUR = Succ<THREE>;
    type FIVE = Succ<FOUR>;
    type SIX = Succ<FIVE>;
    println!("{}", <Succ<TWO> as Add<ZERO>>::Result::to_int());

    assert_eq!(<Succ<TWO> as Add<ZERO>>::Result::to_int(), <Succ<Succ<Succ<ZERO>>> as ToInt>::to_int());
 

    //Tests for Addition
    assert_eq!(std::any::TypeId::of::<ONE>(), std::any::TypeId::of::<Succ<ZERO>>()); // 1 == succ(zero)
    assert_eq!(std::any::TypeId::of::<<TWO as Add<THREE>>::Result>(), std::any::TypeId::of::<FIVE>());  //2 + 3 == 5
    assert_eq!(std::any::TypeId::of::<<ZERO as Add<THREE>>::Result>(), std::any::TypeId::of::<THREE>());  //2 + 3 == 5
 



    //Tests for Multiplication
    type TwoTimesThree = <TWO as Mul<THREE>>::Result;


    assert_eq!(std::any::TypeId::of::<TwoTimesThree>(), std::any::TypeId::of::<SIX>());
    assert_eq!(std::any::TypeId::of::<<ZERO as Mul<TWO>>::Result>(), std::any::TypeId::of::<ZERO>());
    assert_eq!(std::any::TypeId::of::<<TWO as Mul<ZERO>>::Result>(), std::any::TypeId::of::<ZERO>());
    assert_eq!(std::any::TypeId::of::<<TWO as Mul<ONE>>::Result>(), std::any::TypeId::of::<TWO>());
    assert_eq!(std::any::TypeId::of::<<TWO as Mul<Succ<ONE>>>::Result>(), std::any::TypeId::of::<FOUR>());
    type macro_test = int_to_type!(5);
    assert_eq!(std::any::TypeId::of::<macro_test>(), std::any::TypeId::of::<FIVE>());

    type TWELVE = int_to_type!(12);
    type ONEHUNDRED = int_to_type!(1660);

    assert_eq!(std::any::TypeId::of::<TWELVE>(),std::any::TypeId::of::<<THREE as Mul<FOUR>>::Result>());
 

 
}

#[test]
fn test_boolean() {

    assert_eq!(<True as And<True>>::Output::to_bool(), <True as ToBool>::to_bool()); //(True and True) == True
    assert_eq!(<False as Not<False>>::Output::to_bool(), true);
    assert_eq!(<True as And<<False as Not<False>>::Output>>::Output::to_bool(), true);  //(true and (Not(false))) == true



    assert_eq!(std::any::TypeId::of::<<True as And<False>>::Output>(), std::any::TypeId::of::<False>());
    
    assert_eq!(std::any::TypeId::of::<True>(), std::any::TypeId::of::<<False as Or<True>>::Output>()); //True == (False or True)


}