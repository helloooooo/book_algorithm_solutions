use proconio::input;

fn main() {
  input!{
    n: i32,
  }
  func(n);
}

fn func(n :i32 ) -> i32 {
  println!("{}を呼び出しました", n);
  if n == 0 {
    return 0;
  }
  return n + func(n - 1)
}