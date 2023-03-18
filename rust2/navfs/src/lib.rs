mod ls;

#[cfg(test)]
mod tests {
    #[test]
    fn ls_test() {
        crate::ls::ls(None);
    }
}
