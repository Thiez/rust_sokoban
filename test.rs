

fn main() {
  println!("{}",{
  let foo = Some(~"hello");
  println!("ding");
  let foo: ~[&str] = foo.unwrap().words().collect();
  println!("dong");
  let invalid_str = foo[0];
  println!("{}",invalid_str);
  invalid_str
  });
}
