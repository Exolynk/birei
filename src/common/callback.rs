use std::ops::Deref;

use leptos::prelude::{Callable, Callback};

/// Boxed zero-argument callback for one-off component APIs.
pub struct BoxCallback(Box<dyn Fn() + Send + Sync + 'static>);

impl BoxCallback {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self(Box::new(f))
    }

    pub fn run(&self) {
        (self.0)();
    }
}

impl Deref for BoxCallback {
    type Target = dyn Fn() + Send + Sync + 'static;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<F> From<F> for BoxCallback
where
    F: Fn() + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

/// Boxed callback with one argument.
pub struct BoxOneCallback<A, Return = ()>(Box<dyn Fn(A) -> Return + Send + Sync + 'static>);

impl<A, Return> BoxOneCallback<A, Return> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> Return + Send + Sync + 'static,
    {
        Self(Box::new(f))
    }

    pub fn run(&self, arg: A) -> Return {
        (self.0)(arg)
    }
}

impl<A, Return> Deref for BoxOneCallback<A, Return> {
    type Target = dyn Fn(A) -> Return + Send + Sync + 'static;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<F, A, Return> From<F> for BoxOneCallback<A, Return>
where
    F: Fn(A) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

impl<A, Return> From<Callback<A, Return>> for BoxOneCallback<A, Return>
where
    A: Send + Sync + 'static,
    Return: 'static,
{
    fn from(value: Callback<A, Return>) -> Self {
        Self::new(move |arg| value.run(arg))
    }
}

/// Cloneable zero-argument callback for APIs that need to fan out handlers.
pub struct ArcCallback(Callback<(), ()>);

impl Clone for ArcCallback {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for ArcCallback {}

impl ArcCallback {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self(Callback::new(move |_| f()))
    }

    pub fn run(&self) {
        self.0.run(());
    }
}

impl<F> From<F> for ArcCallback
where
    F: Fn() + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

/// Cloneable callback with one argument.
pub struct ArcOneCallback<A: 'static, Return: 'static = ()>(Callback<A, Return>);

impl<A: 'static, Return: 'static> Clone for ArcOneCallback<A, Return> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A: 'static, Return: 'static> Copy for ArcOneCallback<A, Return> {}

impl<A: 'static, Return: 'static> ArcOneCallback<A, Return> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> Return + Send + Sync + 'static,
    {
        Self(Callback::new(f))
    }

    pub fn run(&self, arg: A) -> Return {
        self.0.run(arg)
    }
}

impl<F, A, Return> From<F> for ArcOneCallback<A, Return>
where
    A: 'static,
    Return: 'static,
    F: Fn(A) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

impl<A, Return> From<Callback<A, Return>> for ArcOneCallback<A, Return>
where
    A: Send + Sync + 'static,
    Return: 'static,
{
    fn from(value: Callback<A, Return>) -> Self {
        Self(value)
    }
}

/// Cloneable callback with two arguments.
pub struct ArcTwoCallback<A: 'static, B: 'static, Return: 'static = ()>(Callback<(A, B), Return>);

impl<A: 'static, B: 'static, Return: 'static> Clone for ArcTwoCallback<A, B, Return> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A: 'static, B: 'static, Return: 'static> Copy for ArcTwoCallback<A, B, Return> {}

impl<A: 'static, B: 'static, Return: 'static> ArcTwoCallback<A, B, Return> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A, B) -> Return + Send + Sync + 'static,
    {
        Self(Callback::new(move |(a, b)| f(a, b)))
    }

    pub fn run(&self, a: A, b: B) -> Return {
        self.0.run((a, b))
    }
}

impl<F, A, B, Return> From<F> for ArcTwoCallback<A, B, Return>
where
    A: 'static,
    B: 'static,
    Return: 'static,
    F: Fn(A, B) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}
