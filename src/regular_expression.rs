#![allow(unused, non_camel_case_types)]
/// **正则运算**的数据结构定义
pub struct regularExpression {
    pub regular_id: i32,
    pub name: String,
    //正则运算符
    pub operator_symbol: char,
    //左操作数
    pub operand_id1: i32,
    //右操作数
    pub operand_id2: i32,
    //左操作数的类型
    pub type1: Option<OperandType>,
    //右操作数的类型
    pub type2: Option<OperandType>,
    //运算结果的类型
    pub result_type: Option<OperandType>,
    //词的category属性值
    pub category: LexemeCategory,
    //对应的NFA
    pub p_nfa: Box<Graph>,
}
/// 操作数的类型
pub enum OperandType {
    //字符
    CHAR,
    //字符集
    CHARSET,
    //正则表达式
    REGULAR,
}

/// 词的**类别**
pub enum LexemeCategory {
    //整数常量
    INTEGER_CONST,
    //实数常量
    FLOAT_CONST,
    //科学计数法常量
    SCIENTIFIC_CONST,
    //数值运算词
    OPERATOR,
    //注释
    NOTE,
    //字符串常量
    STRING_CONST,
    //空格常量
    SPACE_CONST,
    //比较运算词
    COMPARE_OPERATOR,
    //变量词
    ID,
    //逻辑运算词
    LOGIC_OPERATOR,
}


/// 图的数据结构
pub struct Graph {
    pub graph_id: i32,
    pub num_of_states: i32,
    pub p_edge_table: Box<Vec<Box<Edge>>>,
    pub p_state_table: Box<Vec<Box<State>>>,
}

/// Edge数据结构
pub struct Edge {
    pub from_state: i32,
    pub next_state: i32,
    pub driver_state: i32,
    pub driver_type: DriverType,
}

pub struct State {
    pub state_id: i32,
    pub state_type: StateType,
    pub category: LexemeCategory,
}

pub enum DriverType {
    //空字符
    NULL,
    //字符
    CHAR,
    //字符集
    CHARSET,
}

pub enum StateType {
    MATCH,
    UNMATCH,
}

