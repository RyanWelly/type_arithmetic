use std::marker::PhantomData;


//
pub struct True {}
pub struct False {}

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

