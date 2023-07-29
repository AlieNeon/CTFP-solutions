use std::collections::HashMap;
use rand::prelude::*;

fn cache_f<A: std::hash::Hash + Eq + Clone, B: Clone>(f: impl Fn(A) -> B) -> impl FnMut(A) -> B {
  let mut cache: HashMap<A, B> = HashMap::new();
  move |x| -> B {
    if let Some(ret) = cache.get(&x) {
      ret.clone()
    } else {
      let ret = f(x.clone());
      cache.insert(x.clone(), ret.clone());
      ret
    }
  }
}

fn factorial(n: u128) -> u128 {
	(2..n).fold(2, |a, b| a * b)
}

pub fn run_challenge() {
  println!("This is the code to run all the chalenges of section 2.7:");
	let mut cached_factorial = cache_f(factorial);
  println!("1. cache_f(factorial)(20): {}, cached: {}", cached_factorial(20), cached_factorial(20));
  println!("2. Caching a random with no input with my cache_f function is not posible due to type strictness");
	let mut cached_rand = cache_f(|x| rand::thread_rng().gen_range(1..x));
	println!("3. cache_f(random_range)(20): {}, cached loop: {:?}", cached_rand(20), (0..10).map(|_| cached_rand(20)).collect::<Vec<_>>());
	println!("4. Which of these C++ functions are pure?
	int fact(int n) {{int i;int result = 1;for (i = 2; i <= n; ++i)result *= i;return result;}}
		This one is pure, int -> int function
	std::getchar()
		This is a state read, dirty impure IO
	bool f() {{std::cout << 'Hello!' << std::endl;return true;}}
		Side effects moment, like yeah the bool can be cached but that print not
	int f(int x) {{static int y = 0;y += x;return y;}}
		So static is just a way to say there is some internal state in the function, just like out cache_f, but not for good but for evil");
	println!("5. There are a bunch of boolean functions: not, and, or, xor, and their negatives, including (notnot := id) that we all know a coconut is a nut. I may be forgeting some but you get the gist being a finite set there is a finite ammount of (pure (terminating)) functions
	Just realized that its bool -> bool not like i said so there are only 4: not, id, true and false");
	println!("6. A picture of all () -> Bool mappings
  true <- () -> false
    true()  false()")
}
