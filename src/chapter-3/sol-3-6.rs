use proconio::input;

fn main() {
  input! {
    k: usize,
    n: usize,
  }

  let mut ans = 0;

  for x in 0..k {
    for y in 0..k {
      let z = n - x - y;
      if z < k {
        ans += 1;
      }
    }
  }
  println!("{}", ans);
}
