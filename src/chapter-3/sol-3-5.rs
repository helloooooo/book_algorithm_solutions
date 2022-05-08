use proconio::input;

fn main() {
  input! {
    n: usize,
    an: [i32; n],
  }

  let mut ans = std::i32::MAX;

  if an.iter().all(|a| *a % 2 == 0) { 
    for a in an {
      ans = std::cmp::min(how_many_times(a), ans);
    }
  }


  println!("{}",ans);
}

fn how_many_times(n :i32) -> i32 {
  let mut cp = n;
  let mut count = 0;
  while cp % 2 == 0 {
    cp = cp / 2;
    count += 1;
  }
  count
}