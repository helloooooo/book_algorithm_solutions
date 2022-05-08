use proconio::input;

fn main() {
  input! {
    s: String,
  }

  let sn = s.chars().map(|c| c as i32 - 48).collect::<Vec<i32>>();

  let n = s.len()- 1;

  let mut sum = 0;

  for bit in 0..(1 << n ) {

    let mut num = 0;

    for i in 0..n {
      num *= 10;
      num += sn[i];
      if bit & (1 << i) != 0 {
        sum += num;
        num = 0;
      }
    }

    num *= 10;
    num += sn.last().unwrap();
    sum += num;
  }

  println!("{}", sum);
}