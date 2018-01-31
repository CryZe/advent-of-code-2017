#![feature(nll)]

fn int()  {
    let mut _x = &mut 5;
    let _y = &_x;
    _x = &mut 7;
}

pub enum Op {
    Snd,
    Set,
    Add,
    Mul,
    Mod,
    Rcv,
    Jgz,
}

pub struct Bla(Box<i32>);
