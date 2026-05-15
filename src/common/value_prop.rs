use leptos::prelude::*;

/// Flexible component prop for values that may be static, optional, signal-backed,
/// or derived by a closure.
#[derive(Clone, Copy)]
pub struct ValueProp<T: Send + Sync + 'static>(Option<Signal<Option<T>>>);

impl<T> Default for ValueProp<T>
where
    T: Send + Sync + 'static,
{
    fn default() -> Self {
        Self(None)
    }
}

impl<T> ValueProp<T>
where
    T: Send + Sync + 'static,
{
    pub fn stored(value: T) -> Self {
        Self(Some(Signal::stored(Some(value))))
    }

    pub fn optional(value: Option<T>) -> Self {
        Self(Some(Signal::stored(value)))
    }

    pub fn derive(value: impl Fn() -> T + Send + Sync + 'static) -> Self {
        Self(Some(Signal::derive(move || Some(value()))))
    }

    pub fn derive_optional(value: impl Fn() -> Option<T> + Send + Sync + 'static) -> Self {
        Self(Some(Signal::derive(value)))
    }
}

impl<T> ValueProp<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn get(&self) -> Option<T> {
        self.0.and_then(|value| value.get())
    }

    pub fn get_untracked(&self) -> Option<T> {
        self.0.and_then(|value| value.get_untracked())
    }
}

impl<T> From<Option<T>> for ValueProp<T>
where
    T: Send + Sync + 'static,
{
    fn from(value: Option<T>) -> Self {
        Self::optional(value)
    }
}

impl<T> From<ReadSignal<T>> for ValueProp<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn from(value: ReadSignal<T>) -> Self {
        Self(Some(Signal::derive(move || Some(value.get()))))
    }
}

impl<T> From<RwSignal<T>> for ValueProp<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn from(value: RwSignal<T>) -> Self {
        Self(Some(Signal::derive(move || Some(value.get()))))
    }
}

impl<T> From<Memo<T>> for ValueProp<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn from(value: Memo<T>) -> Self {
        Self(Some(Signal::derive(move || Some(value.get()))))
    }
}

impl<T> From<Signal<T>> for ValueProp<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn from(value: Signal<T>) -> Self {
        Self(Some(Signal::derive(move || Some(value.get()))))
    }
}

impl<T> From<ReadSignal<Option<T>>> for ValueProp<T>
where
    T: Send + Sync + 'static,
{
    fn from(value: ReadSignal<Option<T>>) -> Self {
        Self(Some(value.into()))
    }
}

impl<T> From<RwSignal<Option<T>>> for ValueProp<T>
where
    T: Send + Sync + 'static,
{
    fn from(value: RwSignal<Option<T>>) -> Self {
        Self(Some(value.into()))
    }
}

impl<T> From<Memo<Option<T>>> for ValueProp<T>
where
    T: Send + Sync + 'static,
{
    fn from(value: Memo<Option<T>>) -> Self {
        Self(Some(value.into()))
    }
}

impl<T> From<Signal<Option<T>>> for ValueProp<T>
where
    T: Send + Sync + 'static,
{
    fn from(value: Signal<Option<T>>) -> Self {
        Self(Some(value))
    }
}

impl<T> From<MaybeProp<T>> for ValueProp<T>
where
    T: Send + Sync + 'static,
{
    fn from(value: MaybeProp<T>) -> Self {
        Self(value.into())
    }
}

impl<T> From<MaybeProp<Option<T>>> for ValueProp<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn from(value: MaybeProp<Option<T>>) -> Self {
        Self(Some(Signal::derive(move || value.get().flatten())))
    }
}

impl<T, F> From<F> for ValueProp<T>
where
    T: Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::derive(value)
    }
}

impl From<&str> for ValueProp<String> {
    fn from(value: &str) -> Self {
        Self::stored(value.to_string())
    }
}

impl From<Option<&str>> for ValueProp<String> {
    fn from(value: Option<&str>) -> Self {
        Self::optional(value.map(str::to_string))
    }
}

macro_rules! impl_static_value_prop {
    ($($value_type:ty),* $(,)?) => {
        $(
            impl From<$value_type> for ValueProp<$value_type> {
                fn from(value: $value_type) -> Self {
                    Self::stored(value)
                }
            }
        )*
    };
}

impl_static_value_prop!(
    String, bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);
