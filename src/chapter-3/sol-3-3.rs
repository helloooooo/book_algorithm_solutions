use proconio::input;


fn main() {
  input! {
    n: usize,
    an: [i32; n],
  }

  let min_v = an.iter().min().unwrap();

  let ans = an.iter().filter(|a| *a != min_v).min().unwrap();
  println!("ans: {}", ans);
}