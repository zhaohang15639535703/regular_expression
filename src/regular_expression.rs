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
    pub p_nfa: Graph,
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
    //空
    EMPTY,
}

/// 图的数据结构
pub struct Graph {
    pub graph_id: i32,
    pub num_of_states: i32,
    pub p_edge_table: Vec<Edge>,
    pub p_state_table: Vec<State>,
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

impl Graph {
    fn add_state(&mut self, state_type: StateType, category: LexemeCategory) {
        // find the max state_id and plus 1,default 0
        let mut state_id = -1;
        for item in self.p_state_table.iter() {
            if item.state_id > state_id {
                state_id = item.state_id;
            }
        }
        state_id += 1;
        //push state into graph.p_state_table
        self.p_state_table.push(State {
            state_id,
            state_type,
            category,
        })
    }
    fn add_edge(&mut self, edge: Edge) {
        self.p_edge_table.push(edge);
    }

    pub fn generate_basic_nfa(driver_type: DriverType, driver_id: i32) -> Graph {
        
        Graph {
            graph_id: 0,
            num_of_states: 0,
            p_edge_table: vec![],
            p_state_table: vec![],
        }
    }
}
