/** Challenge 1 */
fn id<T>(item: T) -> T {
  item
}

/** Challenge 2 */
fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
  move |x| g(f(x))
}

/** Challenge 3 */
fn _compose_respects_id_l<A, B>(f: impl Fn(A) -> B) -> impl Fn(A) -> B {
  compose(f, id)
}
/** We need to prove it works both ways
 * both due to the fat that composition is not commutative
 * and that if it was i whould need to prove that
 */
fn _compose_respects_id_r<A, B>(f: impl Fn(A) -> B) -> impl Fn(A) -> B {
  compose(id, f)
}

pub fn run_challenge() {
  println!("This is the code to run all the chalenges of section 1.4:");
  println!("1. Identity function: 'a' = {} and 3 = {}", id('a'), id(3));
  println!(
    "2. Function composition: ispow2 . len 'owol': {}",
    compose(str::len, usize::is_power_of_two)("owol")
  );
  println!("3. It type checks \u{25fb}.\nRead the code to understand what i mean.");
  println!("4. Is the world-wide web a category in any sense? Are links morphisms? \nYes, with links as morphisms and webpages as objects");
  println!("5. Is Facebook a category, with people as objects and friendships as morphisms? \nYes, since graphs are a category.");
  println!("6. Directed graphs are a subset of graphs");
}
