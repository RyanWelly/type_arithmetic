use std::{marker::PhantomData};

use crate::boolean::*;
mod boolean;
struct Zero {} // 0. Important to make sure we follow axiom that succ(a) != 0 for all nats a.

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

//Trait for runtime testing. 
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


#[test]
fn test_peano() {
    type ZERO = Zero;
    type ONE = Succ<ZERO>;
    type TWO = Succ<ONE>;
    type THREE = Succ<TWO>;
    type FOUR = Succ<THREE>;
    type FIVE = Succ<FOUR>;
    println!("{}", <Succ<TWO> as Add<ZERO>>::Result::to_int());

    assert_eq!(<Succ<TWO> as Add<ZERO>>::Result::to_int(), <Succ<Succ<Succ<ZERO>>> as ToInt>::to_int());
    //type ShouldBeFive = <TWO as Add<THREE>>::Result;
    //type ShouldBeFive = <Zero as Add<THREE>>::Result;
 


    assert_eq!(std::any::TypeId::of::<ONE>(), std::any::TypeId::of::<Succ<ZERO>>()); // 1 == succ(zero)
    assert_eq!(std::any::TypeId::of::<<TWO as Add<THREE>>::Result>(), std::any::TypeId::of::<FIVE>());  //2 + 3 == 5
    assert_eq!(std::any::TypeId::of::<<ZERO as Add<THREE>>::Result>(), std::any::TypeId::of::<THREE>());  //2 + 3 == 5
 


}

#[test]
fn test_boolean() {

    assert_eq!(<True as And<True>>::Output::to_bool(), <True as ToBool>::to_bool()); //(True and True) == True
    assert_eq!(<False as Not<False>>::Output::to_bool(), true);
    assert_eq!(<True as And<<False as Not<False>>::Output>>::Output::to_bool(), true);  //(true and (Not(false))) == true



    assert_eq!(std::any::TypeId::of::<<True as And<False>>::Output>(), std::any::TypeId::of::<False>());  

}