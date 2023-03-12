use safe_regular_expression::char_set::{CharSet,CharSetTable};
fn main() {
    let mut p_char_set_table = CharSetTable::new();
    let char_set1 = CharSet {
        index_id: 0,
        segment_id: 0,
        from_char: 'a',
        to_char: 'z',
    };
    p_char_set_table.push(char_set1);
    p_char_set_table.difference(0, 'b');
    println!("{}", p_char_set_table);
}
