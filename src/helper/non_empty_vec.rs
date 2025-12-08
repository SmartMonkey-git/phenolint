#[derive(Debug, PartialEq)]
pub struct NonEmptyVec<T> {
    inner: Vec<T>,
}

impl<T> From<T> for NonEmptyVec<T> {
    fn from(value: T) -> Self {
        Self::with_single_entry(value)
    }
}

impl<T> NonEmptyVec<T> {
    pub fn with_rest(first: T, rest: Vec<T>) -> Self {
        match rest.is_empty() {
            true => Self { inner: vec![first] },
            false => Self {
                inner: std::iter::once(first).chain(rest).collect(),
            },
        }
    }

    pub fn with_single_entry(first: T) -> Self {
        // TODO: consider using `from`
        Self::with_rest(first, vec![])
    }

    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_single_element() {
        let nev = NonEmptyVec::with_single_entry(10);

        assert_eq!(nev.into_vec(), vec![10]);
    }

    #[test]
    fn test_new_multiple_elements() {
        let nev = NonEmptyVec::with_rest(1, vec![2, 3, 4]);

        assert_eq!(nev.into_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_into_vec_consumes() {
        let nev = NonEmptyVec::with_rest(5, vec![6, 7]);
        let inner = nev.into_vec();

        assert_eq!(inner, vec![5, 6, 7]);
    }

    #[test]
    fn test_with_non_copy_types() {
        let s1 = String::from("Hello");
        let s2 = String::from("World");

        let nev = NonEmptyVec::with_rest(s1.clone(), vec![s2.clone()]);

        assert_eq!(
            nev,
            NonEmptyVec {
                inner: vec![s1, s2]
            }
        );
    }
}
