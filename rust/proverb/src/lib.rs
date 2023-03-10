pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = vec![];

    for i in 0..(list.len()) {
        let item = if i + 1 == list.len() {
            format!("And all for the want of a {}.", list[0])
        } else {
            format!("For want of a {} the {} was lost.", list[i], list[i + 1])
        };
        proverb.push(item);
    }
    proverb.join("\n")
}
