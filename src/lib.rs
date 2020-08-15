pub trait Function: Clone {
    type Args;
    type Output;

    fn call(&self, args: Self::Args) -> Self::Output;
}

impl<Out> Function for fn() -> Out {
    type Args = ();
    type Output = Out;
    fn call(&self, _: Self::Args) -> Self::Output {
        self()
    }
}

impl<A, Out> Function for fn(A) -> Out {
    type Args = (A,);
    type Output = Out;
    fn call(&self, args: Self::Args) -> Self::Output {
        self(args.0)
    }
}
