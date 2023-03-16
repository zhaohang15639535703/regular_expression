#![allow(unused, non_camel_case_types)]
/// 词的**类别**
pub enum LexemeCategory {
    /// 整数常量
    INTEGER_CONST,
    /// 实数常量
    FLOAT_CONST,
    /// 科学计数法常量
    SCIENTIFIC_CONST,
    /// 数值运算词
    OPERATOR,
    /// 注释
    NOTE,
    /// 字符串常量
    STRING_CONST,
    /// 空格常量
    SPACE_CONST,
    /// 比较运算词
    COMPARE_OPERATOR,
    /// 变量词
    ID,
    /// 逻辑运算词
    LOGIC_OPERATOR,
    /// 空
    EMPTY,
}

/// 图的数据结构
pub struct Graph {
    pub graph_id: i32,
    pub num_of_states: i32,
    pub p_edge_table: Vec<Edge>,
    pub p_state_table: Vec<State>,
}

/// Edge数据结构，存储状态转换的边
pub struct Edge {
    pub from_state: i32,
    pub next_state: i32,
    pub driver_id: i32,
    pub driver_type: DriverType,
}

/// State数据结构，存储状态
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
    
    /// 添加一个状态
    fn add_state(&mut self, state_type: StateType, category: LexemeCategory)->i32{
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
        });
        state_id
    }

    /// 添加一条边
    fn add_edge(&mut self, from_state: i32,next_state:i32,driver_type: DriverType,driver_id:i32) {
        self.p_edge_table.push(Edge {
            from_state,
            next_state,
            driver_id,
            driver_type
        })
    }

    /// 获取MATCH状态在该图中p_state_table的索引位置
    fn get_match_state_pos(&self)->i32{
        (self.p_state_table.len()-1) as i32
    }

    /// 针对一个字符或者一个字符集，创建其NFA。其NFA的基本特征是：只包含两个状态（0状态和1状态），且结束状态（即1状态）无出边
    pub fn generate_basic_nfa(driver_type: DriverType, driver_id: i32) -> Graph {
        let mut new_graph = Graph {
            graph_id: 0,
            num_of_states: 2,
            p_edge_table: Vec::new(),
            p_state_table: Vec::new(),
        };
        let from_state = new_graph.add_state(StateType::UNMATCH,LexemeCategory::EMPTY);
        let next_state = new_graph.add_state(StateType::MATCH, LexemeCategory::EMPTY);
        new_graph.add_edge(from_state,next_state,driver_type,driver_id);
        new_graph
    }
    // pub fn union(&self,graph:&Graph)->Graph{
        
    // }
}

#[cfg(test)]
mod test{
    #[test]
    fn test1(){

    }
}