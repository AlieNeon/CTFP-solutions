// 1.4.1: Implement, as best as you can, the identity function in your favorite
//        language (or the second favorite, if your favorite language happens
//        to be Haskell).
function id<T>(t: T): T {
  return t;
}

// 1.4.2: Implement the composition function in your favorite language. It
//        takes two functions as arguments and returns a function that is
//        their composition.
function compose<C, B, A>(ba: (b: B) => A, cb: (c: C) => B): (c: C) => A {
  return (c) => ba(cb(c));
}
let isEven = (n: number) => n % 2 == 0;
let hasEvenLength = compose(isEven, (s: string) => s.length);
console.log(hasEvenLength("true"));  // true
console.log(hasEvenLength("false")); // false

// 1.4.3: Write a program that tries to test that your composition function
//        respects identity.

// This can be proven directly using function extensionality
//
// Having f: A => B and a: A:
//
// 1. (compose(f, id<A>))(a) == (applying the definition of compose)
//    f(id<A>(a))            == (applying the definition of id)
//    f(a)
// Then since (compose(f, id<A>))(a) == f(a), we prove that the result of
// calling compose(f, id<A>) with the parameter a is the same as calling f
// with the parameter a.
//
// 2. The couterpart (compose(id<B>, f))(a) can be proven in a similar way

// We can check it partially with these function:

function doesCompositionRespectsId<A, B>(f: (a: A) => B, a: A): boolean {
  return compose(f, id<A>)(a) === f(a) && compose(id<B>, f)(a) === f(a)
}
console.log(doesCompositionRespectsId(isEven, 0)) // true
console.log(doesCompositionRespectsId((s: string) => s.length, "hi")) // true
console.log(doesCompositionRespectsId(id<undefined>, undefined)) // true