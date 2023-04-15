fn main() {
  let mut msg_number = 1;
  let msg = "hi!";
  println!("message number {}: {}", msg_number, msg);

  msg_number = 2;
  let msg = "hello!";
  println!("message number {}: {}", msg_number, msg);

  msg_number = 3;
  let msg = "hey!";
  println!("message number {1}: {0}", msg_number, msg);
}
