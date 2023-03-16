#![allow(unused)]
use once_cell::unsync::Lazy;
use std::{
    cmp::Ordering,
    fmt::Display,
    mem::swap,
    ops::{Deref, DerefMut},
};

/// **字符集**的数据结构
#[derive(Debug,Clone)]
pub struct CharSet {
    //字符集id
    pub index_id: i32,
    //字符集中的段id,一个字符可以包含多个段
    pub segment_id: i32,
    //段的起始字符
    pub from_char: char,
    //段的结尾字符
    pub to_char: char,
}

//实现函数重载
trait UnionFunc<T, U> {
    fn union(&mut self, c1: T, c2: U) -> i32;
}

/// **字符集表**智能指针定义
pub struct CharSetTable {
    pub table: Vec<CharSet>,
}

impl Deref for CharSetTable {
    type Target = Vec<CharSet>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}
impl DerefMut for CharSetTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}
impl Display for CharSetTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.table)
    }
}

impl UnionFunc<char, char> for CharSetTable {
    /// 字符与字符的并运算，
    fn union(&mut self, c1: char, c2: char) -> i32 {
        let (mut index_id, mut segment_id) = (0, 0);
        index_id = match self.get_max() {
            Some(value) => value + 1,
            None => 0,
        };
        match c1.cmp(&c2) {
            //两个字符相同
            Ordering::Equal => self.push(CharSet {
                index_id,
                segment_id,
                from_char: c1,
                to_char: c1,
            }),
            //c1的下一个字符是c2
            Ordering::Less if (c1 as u32 + 1) == (c2 as u32) => self.push(CharSet {
                index_id,
                segment_id,
                from_char: c1,
                to_char: c2,
            }),
            //c2的下一个字符是c1
            Ordering::Greater if (c2 as u32 + 1) == (c1 as u32) => self.push(CharSet {
                index_id,
                segment_id,
                from_char: c2,
                to_char: c1,
            }),
            //其他情况
            _ => {
                self.push(CharSet {
                    index_id,
                    segment_id,
                    from_char: c1,
                    to_char: c1,
                });
                segment_id = 1;
                self.push(CharSet {
                    index_id,
                    segment_id,
                    from_char: c2,
                    to_char: c2,
                });
            }
        }
        index_id
    }
}
impl UnionFunc<i32, char> for CharSetTable {
    /// 字符集与字符的并运算，不考虑化简，取输入字符的段id，将输入字符作为一个
    /// 字符集加入到输入字符集的下一个段中
    fn union(&mut self, c1: i32, c2: char) -> i32 {
        // the new CharSet was added this line
        let index_id = self.copy_by_index_id(c1);
        let mut segment_id = c1;
        segment_id = match self.get_max_segment_id(c1) {
            Some(value) => value + 1,
            // certainly not to go to this branch unless the input index_id is not legal
            None => panic!("input char_set_id error,no such index_id in P_CHAR_SET_TABLE"),
        };
        self.push(CharSet {
            index_id,
            segment_id,
            from_char: c2,
            to_char: c2,
        });
        index_id
    }
}
impl UnionFunc<i32, i32> for CharSetTable {
    /// 字符集与字符集之间的并运算，遍历两个字符集，合并后生成一个新的字符集
    /// 加入到字符集表中
    fn union(&mut self, c1: i32, c2: i32) -> i32 {
        let mut char_set1: Vec<i32> = Vec::new();
        let mut char_set2: Vec<i32> = Vec::new();
        //获取char_set_id对应的字符集在self的索引位置数组
        for index in 0..self.len() {
            let curr_index_id = self[index].index_id;
            if curr_index_id == c1 {
                char_set1.push(curr_index_id);
            } else if (curr_index_id == c2) {
                char_set2.push(curr_index_id);
            } else {
                continue;
            }
        }
        //对两个索引位置数组分别遍历,生成一个新的字符集并加入到数组中
        let mut index_id = match self.get_max() {
            Some(value) => value + 1,
            None => 0,
        };
        let len = char_set1.len();
        for index1 in char_set1 {
            let from_char = self[index1 as usize].from_char;
            let to_char = self[index1 as usize].to_char;
            self.push(CharSet {
                index_id,
                segment_id: index1,
                from_char,
                to_char,
            });
        }
        for index2 in char_set2 {
            let from_char = self[index2 as usize].from_char;
            let to_char = self[index2 as usize].to_char;
            self.push(CharSet {
                index_id,
                segment_id: index2 + len as i32,
                from_char,
                to_char,
            });
        }
        index_id
    }
}

impl CharSetTable {
    /// 构造函数，初始化为空数组
    pub fn new() -> CharSetTable {
        CharSetTable { table: vec![] }
    }
    pub fn copy_by_index_id(&mut self,index_id:i32)->i32{
        let mut copy_index:Vec<usize> = vec![];
        let next_index = match self.get_max(){
            Some(value) => value+1,
            None=>0,
        };
        for i in 0..self.len(){
            if self[i].index_id == index_id{
                copy_index.push(i);
            }
        }
        for item in copy_index{
            let copy = CharSet{
                index_id: next_index,
                ..self[item].clone()
            };
            self.push(copy);
        }
        next_index
    }
    /// 获取字符表中的index_id的最大值
    fn get_max(&self) -> Option<i32> {
        if self.len() == 0 {
            return None;
        }
        let mut max = -1;
        for ele in self.iter() {
            let curr = (*ele).index_id;
            if curr > max {
                max = curr;
            }
        }
        Some(max)
    }
    /// 给定字符集的index_id，找到该字符集的最大段id
    fn get_max_segment_id(&self, index_id: i32) -> Option<i32> {
        let mut max_segment_id = -1;
        for ele in self.iter() {
            if (*ele).index_id == index_id {
                let curr_segment_id = (*ele).segment_id;
                if curr_segment_id > max_segment_id {
                    max_segment_id = curr_segment_id;
                }
            }
        }
        if max_segment_id == -1 {
            None
        } else {
            Some(max_segment_id)
        }
    }
    /// 字符的范围运算，将两个字符的较小的作为开始范围，较大的作为结束范围，
    /// 生成一个新的字符集,加入到字符集表中
    pub fn range(&mut self, from_char: char, to_char: char) -> i32 {
        let (mut from_char, mut to_char) = (from_char, to_char);
        // let from_char <= to_char
        if from_char as u32 > to_char as u32 {
            swap(&mut from_char, &mut to_char);
        }
        let (mut index_id, segment_id) = (0, 0);
        index_id = match self.get_max() {
            Some(value) => value + 1,
            None => 0,
        };
        self.push(CharSet {
            index_id,
            segment_id,
            from_char,
            to_char,
        });
        index_id
    }
    /// 字符集与字符之间的差运算,改变输入的字符集
    pub fn difference(&mut self, char_set_id: i32, c: char) -> i32 {
        // the new CharSet was added on this line
        let char_set_id = self.copy_by_index_id(char_set_id);
        //需要从self中删除的索引数组
        let mut remove_index_vec: Vec<usize> = vec![];
        //需要加入到self中
        let mut to_add_vec = Vec::new();
        for (index, ele) in self.iter_mut().enumerate() {
            //当前遍历到的index_id值
            let index_id = (*ele).index_id;
            if index_id == char_set_id {
                //如果c在该段的范围中,分情况讨论进行删除
                let (from_char, to_char) = ((*ele).from_char, (*ele).to_char);
                if from_char <= c && to_char >= c {
                    //该段为一个字符,删除该段即可,加入到待删除数组中
                    if from_char == to_char {
                        //可以保证remove_index_vec是升序的
                        remove_index_vec.push(index);
                    //该段为一个字符范围
                    } else {
                        //该字符刚好等于这个段的范围起始字符,更改该段的起始字符为下一个字符即可
                        if c == from_char {
                            (*ele).from_char = char::from_u32(from_char as u32 + 1)
                                .expect("cannot increment this char");
                        //该字符刚好等于这个段的范围结尾字符,更改该段的起始字符为上一个字符即可
                        } else if c == to_char {
                            (*ele).to_char =
                                char::from_u32(to_char as u32 - 1).expect("cannot decrease");
                        //该字符刚好处于在这个段的范围的中间，将该字符集分裂
                        //加入两个新的CharSet，删除原来的那一个CharSet即当前的index索引对应的CharSet
                        } else {
                            let to_char1 = char::from_u32(c as u32 - 1).expect("cannot decrease");
                            let from_char2 =
                                char::from_u32(c as u32 + 1).expect("cannot increment this char");
                            //initialize
                            let segment_id = -1;
                            remove_index_vec.push(index);
                            to_add_vec.push(CharSet {
                                index_id,
                                segment_id,
                                from_char,
                                to_char: to_char1,
                            });
                            to_add_vec.push(CharSet {
                                index_id,
                                segment_id,
                                from_char: from_char2,
                                to_char,
                            });
                        }
                    }
                }
            }
        }
        //先加到末尾
        for i in 0..to_add_vec.len() {
            to_add_vec[i].segment_id = match self.get_max_segment_id(to_add_vec[i].index_id) {
                Some(value) => value + 1,
                None => 0,
            };
            self.push(CharSet { ..to_add_vec[i] });
        }
        //从后向前删除
        remove_index_vec.reverse();
        for &value in remove_index_vec.iter() {
            self.remove(value);
        }
        char_set_id
    }
}

//TODO
/// 合并两个相同char_set_id,不同segment_id,无需合并则返回None
fn merge_char_set(char_set1: &CharSet, char_set2: &CharSet) -> Option<CharSet> {
    //范围合并算法
    //先保证char_set1的最左值小于或等于char_set2的最左值
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range() {
        let mut p_char_set_table = CharSetTable::new();
        let id1 = p_char_set_table.range('a', 'z');
        println!("{}", p_char_set_table);
    }

    #[test]
    fn test_char_union_char() {
        let mut p_char_set_table = CharSetTable::new();
        let char_set1 = CharSet {
            index_id: 0,
            segment_id: 0,
            from_char: 'a',
            to_char: 'z',
        };
        p_char_set_table.union('b', 'c');
    }

    #[test]
    fn test_char_set_union_char() {}

    #[test]
    fn test_char_set_union_char_set() {}

    #[test]
    fn test_difference1() {
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
    #[test]
    fn test_difference2() {
        let mut p_char_set_table = CharSetTable::new();
        let char_set1 = CharSet {
            index_id: 0,
            segment_id: 0,
            from_char: 'a',
            to_char: 'z',
        };
        let char_set2 = CharSet {
            index_id: 0,
            segment_id: 1,
            from_char: 'b',
            to_char: 'b',
        };
        let char_set3 = CharSet {
            index_id: 1,
            segment_id: 0,
            from_char: 'a',
            to_char: 'a',
        };
        p_char_set_table.push(char_set1);
        p_char_set_table.push(char_set2);
        p_char_set_table.push(char_set3);
        p_char_set_table.difference(0, 'b');
        println!("{}", p_char_set_table);
    }
    #[test]
    fn test_merge_char_set() {}

    fn clone_test(ref_char_set:&CharSet)->CharSet{
        CharSet { ..(*ref_char_set) }
    }
    #[test]
    fn test_ref(){
        let test_char_set = CharSet {
            index_id: 0,
            segment_id: 1,
            from_char: 'b',
            to_char: 'b',
        };
        let test2 = clone_test(&test_char_set);
        let test3 = test_char_set.clone();
        println!("{:#?}",test_char_set);
        println!("{:#?}",test3);
    }
}
