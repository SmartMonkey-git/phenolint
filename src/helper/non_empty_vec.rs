#[derive(Debug, PartialEq)]
pub struct NonEmptyVec<T> {
    inner: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub fn new(first: T, rest: Option<Vec<T>>) -> Self {
        match rest {
            None => Self { inner: vec![first] },
            Some(rest) => Self {
                inner: std::iter::once(first).chain(rest).collect(),
            },
        }
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
        let nev = NonEmptyVec::new(10, None);

        assert_eq!(nev.into_vec(), vec![10]);
    }

    #[test]
    fn test_new_multiple_elements() {
        let nev = NonEmptyVec::new(1, Some(vec![2, 3, 4]));

        assert_eq!(nev.into_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_into_vec_consumes() {
        let nev = NonEmptyVec::new(5, Some(vec![6, 7]));
        let inner = nev.into_vec();

        assert_eq!(inner, vec![5, 6, 7]);
    }

    #[test]
    fn test_with_non_copy_types() {
        let s1 = String::from("Hello");
        let s2 = String::from("World");

        let nev = NonEmptyVec::new(s1.clone(), Some(vec![s2.clone()]));

        assert_eq!(
            nev,
            NonEmptyVec {
                inner: vec![s1, s2]
            }
        );
    }
}
