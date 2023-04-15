fn main() {
  // type inference
  let var1 = "hello";
  let var2 = 12;

  // manifest typing
  let var3: i32 = 12;
  let var4: &str = "hello";

  println!("{} {} {} {}", var1, var2, var3, var4);
}
