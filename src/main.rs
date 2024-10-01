mod uint256;

use uint256::Uint256;

fn main() {
    let a = Uint256::ONE;
    let two = Uint256::TWO;
    let new = two + two + a;
    let max = Uint256::MAX;
    let sth = max + a;

    let addme = Uint256([u64::MAX, 0, 0, 0]);
    let shouldbe = Uint256([0, 1, 0, 0]);
    let is = addme + Uint256::ONE;

    println!("Hello, world!");
    println!("  a: {:?}", a);
    println!("two: {:?}", two);
    println!("new: {:?}", new);

    println!("max: {:?}", max);
    println!("sth: {:?}", sth);

    println!("   addme: {:?}", addme);
    println!("shouldbe: {:?}", shouldbe);
    println!("      is: {:?}", is);
}
