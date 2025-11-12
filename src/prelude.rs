#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: &str, age: u32) -> Self {
        User {
            name: name.to_string(),
            age,
        }
    }
}

// Scala-like extension traits
pub trait MapFilter<T> {
    fn map<F, R>(self, f: F) -> Vec<R>
    where
        F: FnMut(T) -> R;
    fn filter<F>(self, f: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
        T: Sized;
}

impl<T> MapFilter<T> for Vec<T> {
    fn map<F, R>(self, mut f: F) -> Vec<R>
    where
        F: FnMut(T) -> R,
    {
        self.into_iter().map(f).collect()
    }

    fn filter<F>(self, mut f: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        self.into_iter().filter(|x| f(x)).collect()
    }
}

// Scala-like Option + pattern match
pub fn compute_result(x: i32) -> Option<i32> {
    match x {
        1 => Some(10),
        2 => Some(20),
        _ => None,
    }
}

// Fold like in Scala’s collections
pub trait Fold<T> {
    fn fold<F>(self, init: T, f: F) -> T
    where
        F: FnMut(T, T) -> T;
}

impl<T> Fold<T> for Vec<T>
where
    T: Copy,
{
    fn fold<F>(self, init: T, mut f: F) -> T
    where
        F: FnMut(T, T) -> T,
    {
        self.into_iter().fold(init, f)
    }
}
