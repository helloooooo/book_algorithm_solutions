use proconio::input;


fn main() {
  input!{
    n: i32
  }
  println!("{}", fibo(n))
}

fn fibo(n : i32) -> i32 {
  if n == 0 { return 0 }
  if n == 1 { return 1 }

  return fibo(n - 1) + fibo(n - 2);
}