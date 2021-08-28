use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use modulo::Mod;
use num_traits::{Pow, ToPrimitive, pow};

pub struct FieldElement<usize> {
    pub num: usize,
    pub prime: usize
}

impl FieldElement<usize> {
    pub fn new(num: usize, prime: usize) -> Option<FieldElement<usize>>{
        if num >=0 && num < prime{
            Some(FieldElement{num: num, prime: prime})
        }
        else {
            None
        }
    }
}
impl fmt::Debug for FieldElement<usize> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.num, self.prime)
    }
}
impl PartialEq for FieldElement<usize> {
    fn eq(&self, other: &Self) -> bool{
        self.num == other.num && self.prime == other.prime
    }
}
impl Add for FieldElement<usize> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Option<Self> {
       if self.prime != other.prime {
           return None
       }
       let num = (self.num + other.num).modulo(self.prime);  
       Some(Self{num: num, prime: self.prime})
    }
}
impl Sub for FieldElement<usize> {
    type Output = Option<Self>;
    fn sub(self, other: Self) -> Option<Self> {
        if self.prime != other.prime {
           return None 
        }
        let num = (self.num - other.num).modulo(self.prime);
        Some(Self{num: num, prime: self.prime})
    }
}
impl Mul for FieldElement<usize> {
    type Output = Option<Self>;
    fn mul(self, other: Self) -> Option<Self> {
        if self.prime != other.prime {
            return None
        }
        let num = (self.num * other.num).modulo(self.prime);
        Some(Self{num: num, prime:self.prime})
    }
}
impl Pow<isize> for FieldElement<usize> {
    type Output = Self;
    fn pow(self, exp: isize) -> Self {
        let exponent = exp;
        let prime = &self.prime;
        while exponent < 0 {
            let exponent = exponent + (prime - 1).to_isize().unwrap();
        }
        let num = Pow::pow(self.num, exp.to_usize().unwrap()).modulo(self.prime);
        Self{num: num, prime: self.prime}
    }
}
impl Div for FieldElement<usize> {
    type Output = Option<Self>;
    fn div(self, other: Self) -> Option<Self> {
        if self.prime != other.prime{
            return None
        }
        // use Fermat's little theorem:
        let num = self.num * Pow::pow(other.num, self.prime-2) % self.prime;
        Some(Self{num: num, prime:self.prime})
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;
    use std::ops::{Add, Div, Mul, Sub};
    use modulo::Mod;
    use num_traits::{Pow, ToPrimitive, pow};

    #[test]
    fn is_same(){
        assert_eq!(FieldElement{num: 10, prime: 13}, FieldElement{num: 10, prime: 13});
    } 

    #[test]
    fn add() {
        let a = FieldElement{num: 7, prime: 13};
        let b = FieldElement{num: 12, prime: 13};
        let c = FieldElement{num: 6, prime: 13};
        assert_eq!((a+b).unwrap(), c);
    }

    #[test]
    fn multiple() {
        let a = FieldElement{num: 3, prime: 13};
        let b = FieldElement{num: 12, prime: 13};
        let c = FieldElement{num: 10, prime: 13};
        assert_eq!((a*b).unwrap(), c);
    }
    #[test]
    fn power() {
        let a = FieldElement{num: 3, prime: 13};
        let b = FieldElement{num: 1, prime: 13};
        assert_eq!(Pow::pow(a, 3), b);
    }

}
