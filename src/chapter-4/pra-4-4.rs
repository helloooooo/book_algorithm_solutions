use proconio::input;


fn main() {
 input!{
   m: i32,
   n: i32,
 }
 println!("{}", gcd(n, m))
}

fn gcd(m: i32, n: i32) -> i32 {
  if n == 0 {
    return m
  }
  return gcd(n, m %n)
}
