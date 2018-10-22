use std::io;
//use std::io::prelude::*;
fn add(A: i32, B: i32)
{
    return (A + B);
}

fn sub(A: i32, B: i32)
{
    return (A - B);
}

fn mult(A: i32, B: i32)
{
    return (A*B);
}

fn mult(A: i32, B: i32)
{
    return (A/B);
}

fn input()
{
    let mut A = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");
    let mut vec = A.split_whitespace()
    .map(|x| x.parse::<i32>().expect("parse error"))
    .collect::<Vec<i32>>();
    return A;
}

fn main() {
    let mut A = input();

    let mut B = input();

    let c = sub(A,B);
    println!("{}",c);
}
