use std::str::FromStr;

pub struct Parsed<I, T>
where
    I: Iterator,
    I::Item: ToString,
    T: FromStr,
{
    inner: I,
    _marker: std::marker::PhantomData<T>,
}

impl<I, T> Iterator for Parsed<I, T>
where
    I: Iterator,
    I::Item: ToString,
    T: FromStr,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|item| item.to_string().trim().parse().ok())
    }
}

pub trait ParsedExt: Iterator {
    fn parsed<T>(self) -> Parsed<Self, T>
    where
        Self::Item: ToString,
        Self: Sized,
        T: FromStr,
    {
        Parsed {
            inner: self,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<I: Iterator> ParsedExt for I {}
