use modulo::Mod;
use num_traits::{Pow, pow};
pub mod field_element;

fn main() {
    let test = field_element::FieldElement{
        num: 10,
        prime: 13,
    };
    println!("{:?}",test);
    let comp = field_element::FieldElement{
        num: 9,
        prime: 113,
    };

    println!("{:?}", test==comp);

    let a = field_element::FieldElement{
        num: 7,
        prime: 13,
    };
    let b = field_element::FieldElement{
        num: 12,
        prime: 13,
    };
    let c = field_element::FieldElement{
        num: 6,
        prime: 13,
    };

    let ret = a+b;
    println!("{:?}", &ret);
    println!("{:?}", (ret.unwrap()==c));

    let a = field_element::FieldElement{
        num: 3,
        prime: 13,
    };
    let b = field_element::FieldElement{
        num: 12,
        prime: 13,
    };
    let c = field_element::FieldElement{
        num: 10,
        prime: 13,
    };
    let ret = a*b;
    println!("{:?}", ret.unwrap()==c);
    let a = field_element::FieldElement{
        num: 3,
        prime: 13,
    };
    let b = field_element::FieldElement{
        num: 1,
        prime: 13,
    };
    let ret = Pow::pow(a, 3);
    println!("Power");
    println!("{:?}", ret==b);
}

