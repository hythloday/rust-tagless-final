#![feature(generic_associated_types)]

trait Lang {
    type Expr<T>;
    fn int(i: u64) -> Self::Expr<u64>;
    fn add(l: Self::Expr<u64>, r: Self::Expr<u64>) -> Self::Expr<u64>;
}

use core::marker::PhantomData;
struct Evaluation<Env>(PhantomData<Env>);

type Func<A, B> = Box<dyn Fn(A) -> B>;

impl<Env: Copy + 'static> Lang for Evaluation<Env> {
    type Expr<T> = Func<Env, T>;
    fn int(i: u64) -> Self::Expr<u64> {
        Box::new(move |_env| i)
    }
    fn add(l: Func<Env, u64>, r: Func<Env, u64>) -> Func<Env, u64> {
        Box::new(move |env| l(env) + r(env))
    }
}

struct PrettyPrinting;
impl Lang for PrettyPrinting {
    type Expr<T> = String;
    fn int(i: u64) -> Self::Expr<u64> {
        i.to_string()
    }
    fn add(l: String, r: String) -> Self::Expr<u64> {
        format!("({} + {})", l, r)
    }
}

fn program<L>() -> L::Expr<u64>
where L: Lang {
    L::add(
        L::add(L::int(1), L::int(2)),
        L::add(L::int(3), L::int(4))
    )
}

fn main() {
    let pretty = program::<PrettyPrinting>();
    println!("pretty: {:?}", pretty);
    let z = ();
    let eval = program::<Evaluation<()>>()(z);
    println!("eval: {:?}", eval);
}
