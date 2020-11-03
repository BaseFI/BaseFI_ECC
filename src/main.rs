
pub mod BigIntegerUtils;
use BigIntegerUtils::BigInteger;

pub mod ECC_Point;
use ECC_Point::ECC_POINT;

//#[global_allocator]
//static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
fn main() {
    println!("Hello, world!");
    let mut bi=BigInteger::new();
    bi.from_base_16(String::from("0102030405"));
    println!("bi{}", bi.to_base_16());
    let mut bi2=BigInteger::new();
    bi2.from_base_16(String::from("2"));

    let mut bi3=bi%bi2;
    println!("bi3{}", bi3.to_base_16());
    //let mut bi5=&bi%&bi2;
    //let mut bi5=bi%bi2;
    //let mut bi4=bi.clone();
    //bi4.shift(-9);
    println!("i<<2 {}", 0xff<<2);
    //println!("bi4{}", bi4.to_base_16());

    let g_ecc_point=ECC_POINT::Get_G_Point();
    let mut kg=ECC_POINT::Mul(&g_ecc_point,&BigInteger::static_from_base_16(String::from("12345678")));
    println!("kgx{}", kg.X.to_base_16());
    println!("kgy{}", kg.Y.to_base_16());
}
