use super::ident::IdentLike;

pub trait Matchable {
    type Item;

    fn matched(&self, name: &str) -> Vec<&Self::Item>;
}

impl<T: IdentLike> Matchable for [T] {
    type Item = T;

    fn matched(&self, name: &str) -> Vec<&Self::Item> {
        self.iter()
            .filter(|item| item.is_match(name))
            .collect()
    }
}
