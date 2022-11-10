use std::ops::{Try, FromResidual, ControlFlow, Residual, Yeet};
use std::convert::Infallible;

#[derive(Clone, Hash)]
pub struct Name(pub String);
pub struct Nondet<T>(pub T);

#[macro_export]
macro_rules! yeet {
    ($x:expr) => {
        do yeet $x
    };
}

impl<T, E> Try for Nondet<Result<T, E>> {
    type Output = T;
    type Residual = Nondet<Result<Infallible, E>>;

    fn from_output(output: Self::Output) -> Self {
        Nondet(Ok(output))
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(x) => ControlFlow::Continue(x),
            Err(e) => ControlFlow::Break(Nondet(Err(e))),
        }
    }
}

impl<T, E> FromResidual<Nondet<Result<Infallible, E>>> for Nondet<Result<T, E>> {
    fn from_residual(residual: Nondet<Result<Infallible, E>>) -> Self {
        match residual.0 {
            Ok(x) => match x {},
            Err(e) => Nondet(Err(e))
        }
    }
}

impl<T, E> FromResidual<Result<Infallible, E>> for Nondet<Result<T, E>> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(x) => match x {},
            Err(e) => Nondet(Err(e))
        }
    }
}

impl<T, E> Residual<T> for Nondet<Result<Infallible, E>> {
    type TryType = Nondet<Result<T, E>>;
}

impl<T, E> FromResidual<Yeet<E>> for Nondet<Result<T, E>> {
    fn from_residual(residual: Yeet<E>) -> Self {
        Nondet(Err(residual.0))
    }
}
