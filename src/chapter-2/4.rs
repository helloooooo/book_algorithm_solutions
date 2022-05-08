use proconio::input;

fn main() {
  input! {
    n: usize,
    xyn: [(i64, i64); n],
  }
  let mut min_dist = std::f64::MAX;

  for i in 0..n {
    for j in i..n {
      let dist = calc_dist(xyn[i].0, xyn[i].1, xyn[j].0, xyn[j].1);
      if dist < min_dist {
        min_dist = dist;
      }
    }
  }
  println!("{}", min_dist);
}

fn calc_dist(x1: i64, y1: i64, x2: i64, y2: i64) -> f64 {
  let dist: f64 =((x1-x2)*(x1-x2) + (y1-y2)*(y1-y2)) as f64;
  dist.sqrt()
 }
