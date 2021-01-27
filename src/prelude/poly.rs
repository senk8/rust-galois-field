use super::ring_context::RingContext;
use super::ring_elm::RingElm;
use std::fmt;
use std::ops::*;
use num::Complex;
use num::traits::Num;

#[derive(Debug,Clone)]
pub struct Polynomial(Vec<f64>);

impl Polynomial {
    pub fn new(vec: Vec<f64>) -> Polynomial {
        Polynomial(vec)
    }
    pub fn eval(&self, root:Complex<f64>)->Complex<f64> 
    {
        let mut sum = Complex::new(0f64,0f64);
        for i in 0..self.0.len() {
            sum = sum + root.powi(i as i32) * self.0[i];
        }
        sum
    }

    /*
    pub fn size(self)->{
        // sum の型は？
        self.0.fold(0,|sum,a|sum+a.pow(2))
    }
    */
}

impl Add for Polynomial {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Polynomial(self.0.iter().zip(rhs.0).map(|(x, y)| *x + y).collect())
    }
}

impl Mul for Polynomial {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        use std::cmp::{max,min};
        let d= self.0.len()+rhs.0.len()-1;
        let mut poly = vec![0f64;d];

        for k in 0..=d{
            for i in 0..=k{
                if min(self.0.len(),rhs.0.len()) <= max(i,k-i) { continue; }
                poly[k]=poly[k]+self.0[i]*rhs.0[k-i];
            }
        }

        Polynomial(poly)
    }
}

/*
impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().map(|num|std::char::from_digit(num as u32, 10));
        write!(f,format!(self.0.map.) )
    }
}
*/

/*
pub fn is_irreducible(poly:Polynomial) -> bool
{
    let p =3;
    let mut irreducible = true;
    if poly[0]%p==0 && poly[0]*poly[0]%p {
    }

    for i in 1..k{
        irreducible &= poly[i]%3==0;
    }

    if poly[k]%p != 0
    irreducible
}
*/

//pub fn crt()
