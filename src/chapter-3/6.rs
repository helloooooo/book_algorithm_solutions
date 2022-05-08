use proconio::input;

fn main() {
  input! {
    n: usize,
    an: [usize; n],
    w: usize,
  }

  let mut exist = false;

  for bit in 0..(1 << n) {
    let mut sum = 0;
    for i in 0..n {
      if bit & (1 << i) != 0 {
        sum += an[i];
      }
    }
    if sum == w {
      exist = true;
    }
  }

  let ans = if exist { 
    "Yes"
  } else {
    "No"
  };

  println!("{}", ans);
}