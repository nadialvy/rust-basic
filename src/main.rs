fn main() {
  let (var1, mut var2, var3) = (1, 2, 3);
  println!("var1: {}, var2: {}, var3: {}", var1, var2, var3);
  var2 = 4;
  println!("var1: {}, var2: {}, var3: {}", var1, var2, var3);
}
