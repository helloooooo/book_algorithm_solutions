use proconio::input;

fn main() {
  input! {
    n: usize,
    an: [i32; n],
    v: i32,
  }

  let mut count = 0;

  for i in 0..n {
    if an[i] == n {
      count += 1;
    }
  }
  println!("count: {}", count);
}