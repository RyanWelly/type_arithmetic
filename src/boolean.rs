//use std::marker::PhantomData;


//
pub struct True;
pub struct False;

pub trait ToBool {
    fn to_bool() -> bool;
}

impl ToBool for True {
    fn to_bool() -> bool {
        true
    }
}
impl ToBool for False {
    fn to_bool() -> bool {
        false
    }
}



pub trait Not<Bool> {
    type Output;
}

impl Not<True> for True {
    type Output = False;
}

impl Not<False> for False {
    type Output = True;
}

pub trait And<A> {
    type Output;
}


//change to similar to the Add method, should be like And<True> for False etc. 
impl And<True> for True{
    type Output = True;
}

impl And<True> for False {
    type Output = False;
}

impl And<False> for False {
    type Output = False;
}

impl And<False> for True {
    type Output = False;
}


//Maybe try and figure out how I can write something like impl Or<T: ToBool> for True to cover two cases at once.
pub trait Or<P> {
    type Output;
}

impl Or<True> for True {
    type Output = True;
}

impl Or<False> for True {
    type Output = True;
}

impl Or<True> for False {
    type Output = True;
}

impl Or<False> for False {
    type Output = False;
}
