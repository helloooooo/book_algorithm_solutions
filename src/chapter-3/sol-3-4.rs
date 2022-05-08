use proconio::input;


fn main() {
  input! {
    n: usize,
    an: [i32; n],
  }

  let min_v = an.iter().min().unwrap();
  let max_v = an.iter().max().unwrap();

  println!("ans: {}", max_v - min_v);
}