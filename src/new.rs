pub fn print_type<T>() {
    println!("{}", core::any::type_name::<T>());
}

pub struct Nil;

pub struct Cons<H, T>(H, T);

trait Prepend<D>: Sized {
    type Out;
}

impl<D> Prepend<D> for Nil {
    type Out = Cons<D, Self>;
}

impl<D, H, T> Prepend<D> for Cons<H, T> {
    type Out = Cons<D, Self>;
}


trait Append<D>: Sized {
    type Out;
}

impl<D> Append<D> for Nil {
    type Out = Cons<D, Nil>;
}

impl<D, H, T> Append<D> for Cons<H, T>
    where T: Append<D>
{
    type Out = Cons<H, <T as Append<D>>::Out>;
}

impl<H, A, B> Cons<H, Cons<A, B>> {
    fn advance(self) -> Option<(H, Cons<A, B>)> {
        Some((self.0, self.1))
    }
}

impl Nil {
    fn advance<A, B>(self) -> Option<(A, B)> {
        None
    }
}

fn make<A, B>(a: A, b: B) -> Cons<A, B> {
    Cons(a, b)
}

#[test]
fn test() {
    type X = <<<Nil as Prepend<bool>>::Out as Prepend<i32>>::Out as Prepend<&'static str>>::Out;
    let x: X = make("asd", make(-12, make(true, Nil)));

    type Y = <<<Nil as Append<bool>>::Out as Append<i32>>::Out as Append<&'static str>>::Out;
    print_type::<Y>();
}