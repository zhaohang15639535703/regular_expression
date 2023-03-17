#![allow(unused, non_camel_case_types)]

/// 词的**类别**
#[derive(PartialEq, Eq, Clone)]
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
#[derive(Clone)]
pub struct Graph {
    pub graph_id: i32,
    pub num_of_states: i32,
    pub p_edge_table: Vec<Edge>,
    pub p_state_table: Vec<State>,
}

/// Edge数据结构，存储状态转换的边
#[derive(Clone)]
pub struct Edge {
    pub from_state: i32,
    pub next_state: i32,
    pub driver_id: i32,
    pub driver_type: DriverType,
}

/// State数据结构，存储状态
#[derive(Clone)]
pub struct State {
    pub state_id: i32,
    pub state_type: StateType,
    pub category: LexemeCategory,
}

#[derive(PartialEq, Eq, Clone)]
pub enum DriverType {
    //空字符
    NULL,
    //字符
    CHAR,
    //字符集
    CHARSET,
}

#[derive(PartialEq, Eq, Clone)]
pub enum StateType {
    MATCH,
    UNMATCH,
}

impl Graph {
    /// 添加一个状态
    fn add_state(&mut self, state_type: StateType, category: LexemeCategory) -> i32 {
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
    fn add_edge(
        &mut self,
        from_state: i32,
        next_state: i32,
        driver_type: DriverType,
        driver_id: i32,
    ) {
        self.p_edge_table.push(Edge {
            from_state,
            next_state,
            driver_id,
            driver_type,
        })
    }
    /// 开始状态是否有入边
    fn is_start_state_has_edge(&self) -> bool {
        for item in self.p_edge_table.iter() {
            if item.next_state == 0 {
                return true;
            }
        }
        false
    }
    /// 开始状态的category是否为空
    fn is_start_state_category_empty(&self) -> bool {
        match self.p_state_table.get(0) {
            Some(value) if value.category == LexemeCategory::EMPTY => true,
            Some(_) => false,
            None => panic!("no from_state in this graph"),
        }
    }
    /// 结束状态是否有出边
    fn is_end_state_has_edge(&self) -> bool {
        for item in self.p_edge_table.iter() {
            if item.from_state == (self.p_state_table.len() - 1) as i32 {
                return true;
            }
        }
        false
    }
    /// 结束状态的category是否为空
    fn is_end_state_category_empty(&self) -> bool {
        match self.p_state_table.get(self.p_state_table.len() - 1) {
            Some(value) if value.category == LexemeCategory::EMPTY => true,
            Some(_) => false,
            None => panic!("no end_state in this graph"),
        }
    }

    /// 获取MATCH状态在该图中p_state_table的索引位置
    fn get_match_state_pos(&self) -> i32 {
        (self.p_state_table.len() - 1) as i32
    }
    /// union 中的等价转换
    fn equivalent_transform(&mut self) {
        // s的开始状态有入边或开始状态的category不为空，结束状态无出边且category属性为空
        if self.is_start_state_has_edge() && !self.is_start_state_category_empty() {
            
        }
    }

    /// 针对一个字符或者一个字符集，创建其NFA。其NFA的基本特征是：只包含两个状态（0状态和1状态），且结束状态（即1状态）无出边
    pub fn generate_basic_nfa(driver_type: DriverType, driver_id: i32) -> Graph {
        let mut new_graph = Graph {
            graph_id: 0,
            num_of_states: 2,
            p_edge_table: Vec::new(),
            p_state_table: Vec::new(),
        };
        let from_state = new_graph.add_state(StateType::UNMATCH, LexemeCategory::EMPTY);
        let next_state = new_graph.add_state(StateType::MATCH, LexemeCategory::EMPTY);
        new_graph.add_edge(from_state, next_state, driver_type, driver_id);
        new_graph
    }
    /// 最简NFA构造法并运算 s|t
    pub fn union(&self, graph: &Graph) -> Graph {
        let (mut s, mut t) = (self.clone(), graph.clone());
        s.equivalent_transform();
        t.equivalent_transform();
        // s的开始状态无入边，结束状态无出边且category为空 && t的开始状态无入边，结束状态无出边category为空

        let from_state = State {
            state_id: 0,
            state_type: StateType::UNMATCH,
            category: LexemeCategory::EMPTY,
        };
        assert!(self.p_state_table.len() >= 2);
        assert!(graph.p_state_table.len() >= 2);
        // self.p_state_table.len()-1 +graph.p_state_table.len()-1 -1
        let state_id = (self.p_state_table.len() + graph.p_state_table.len() - 3) as i32;
        let to_state = State {
            state_id,
            state_type: StateType::MATCH,
            category: LexemeCategory::EMPTY,
        };
        let mut p_state_table: Vec<State> = Vec::new();
        // 序号重排列后加入将原来的两个p_state_table
        for (index, item) in self.p_state_table.iter().enumerate() {
            //开始状态和结束状态忽略
            if index == 0 || index == self.p_state_table.len() - 1 {
                continue;
            }
            p_state_table.push(item.clone());
        }
        for (index, item) in graph.p_state_table.iter().enumerate() {
            if index == 0 || index == graph.p_state_table.len() - 1 {
                continue;
            }
            let mut next_state = item.clone();
            next_state.state_id = (index + self.p_state_table.len() - 2) as i32;
            p_state_table.push(next_state);
        }
        let mut p_edge_table: Vec<Edge> = Vec::new();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {}
}
