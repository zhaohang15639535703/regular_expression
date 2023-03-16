#![allow(unused, non_camel_case_types)]
use super::graph::{LexemeCategory,Graph};
/// **正则运算**的数据结构定义
pub struct regularExpression {
    pub regular_id: i32,
    pub name: String,
    /// 正则运算符
    pub operator_symbol: char,
    /// 左操作数
    pub operand_id1: i32,
    /// 右操作数
    pub operand_id2: i32,
    /// 左操作数的类型
    pub type1: OperandType,
    /// 右操作数的类型
    pub type2: OperandType,
    /// 运算结果的类型
    pub result_type: OperandType,
    /// 词的category属性值
    pub category: LexemeCategory,
    /// 对应的NFA
    pub p_nfa: Graph,
}
/// 操作数的类型
pub enum OperandType {
    /// 字符
    CHAR,
    /// 字符集
    CHARSET,
    /// 正则表达式
    REGULAR,
    /// 无
    NONE,
}

