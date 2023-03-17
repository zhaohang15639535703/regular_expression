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
    /// 该边的转换前的状态id
    pub from_state: i32,
    /// 该边的经过转换后的状态id
    pub next_state: i32,
    /// 驱动转换的字符或字符集id,为-1时表示空转换
    pub driver_id: i32,
    /// 驱动转换的类型
    pub driver_type: DriverType,
}

/// State数据结构，存储状态
#[derive(Clone)]
pub struct State {
    pub state_id: i32,
    /// MATCH or UNMATCH
    pub state_type: StateType,
    /// 词法属性
    pub category: LexemeCategory,
}

#[derive(PartialEq, Eq, Clone)]
pub enum DriverType {
    /// 空
    NULL,
    /// 字符
    CHAR,
    /// 字符集
    CHARSET,
}

#[derive(PartialEq, Eq, Clone)]
pub enum StateType {
    /// 匹配状态即结束状态
    MATCH,
    /// 任何非结束状态
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
    fn is_start_state_has_edge_in(&self) -> bool {
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
    fn is_end_state_has_edge_out(&self) -> bool {
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

    /// union 中的等价转换
    fn equivalent_transform(&mut self) {
        // 第一步改造
        // 开始状态有入边
        if self.is_start_state_has_edge_in() {
            // 重新排列state序号
            for item in self.p_state_table.iter_mut() {
                item.state_id = item.state_id + 1;
            }
            // 重新设置edge中存储的state序号
            for item in self.p_edge_table.iter_mut() {
                item.from_state = item.from_state + 1;
                item.next_state = item.next_state + 1;
            }
            // 在原来的开始状态前加一个状态
            self.p_state_table.insert(
                0,
                State {
                    state_id: 0,
                    state_type: StateType::UNMATCH,
                    category: LexemeCategory::EMPTY,
                },
            );
            // 加入一条由当前的开始状态到原来的开始状态的边
            self.p_edge_table.insert(
                0,
                Edge {
                    from_state: 0,
                    next_state: 1,
                    driver_id: -1,
                    driver_type: DriverType::NULL,
                },
            )
        }
        // 结束状态有出边
        if self.is_end_state_has_edge_out() {
            // 原来的结束状态的state_type变为UNMATCH,即由结束状态改为非结束状态
            let end_pos = self.p_state_table.len() - 1;
            self.p_state_table[end_pos].state_type = StateType::UNMATCH;
            // 在原来的结束状态后加一个结束状态作为新的结束状态
            self.p_state_table.push(State {
                state_id: self.p_state_table.len() as i32,
                state_type: StateType::MATCH,
                category: LexemeCategory::EMPTY,
            });
            // 加入一条原来的结束状态到当前的结束状态的边
            self.p_edge_table.push(Edge {
                from_state: (self.p_state_table.len() - 2) as i32,
                next_state: (self.p_state_table.len() - 1) as i32,
                driver_id: -1,
                driver_type: DriverType::NULL,
            })
        }
        // 第二步改造
        // 如果第一步转换后的NFA结束状态的category属性值不为空，具体做法同第一步的结束状态无出边的情况
        if self.p_state_table[self.p_state_table.len() - 1].category != LexemeCategory::EMPTY {
            // 原来的结束状态的state_type变为UNMATCH,即由结束状态改为非结束状态
            let end_pos = self.p_state_table.len() - 1;
            self.p_state_table[end_pos].state_type = StateType::UNMATCH;
            // 在原来的结束状态后加一个结束状态作为新的结束状态
            self.p_state_table.push(State {
                state_id: self.p_state_table.len() as i32,
                state_type: StateType::MATCH,
                category: LexemeCategory::EMPTY,
            });
            // 加入一条原来的结束状态到当前的结束状态的边
            self.p_edge_table.push(Edge {
                from_state: (self.p_state_table.len() - 2) as i32,
                next_state: (self.p_state_table.len() - 1) as i32,
                driver_id: -1,
                driver_type: DriverType::NULL,
            })
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
    /// 最简NFA构造法:**并运算** s|t
    pub fn union(&self, graph: &Graph) -> Graph {
        //TODO 判断传入的graph是否合法，目前仅简单的assert
        assert!(self.p_state_table.len() >= 2);
        assert!(graph.p_state_table.len() >= 2);

        let (mut s, mut t) = (self.clone(), graph.clone());
        // 判断如果不符合最简形式，先进行等价转换，保证开始状态都无入边，结束状态都无出边且category属性值都为空
        if !(!s.is_start_state_has_edge_in()
            && s.is_start_state_category_empty()
            && !t.is_start_state_has_edge_in()
            && t.is_start_state_category_empty())
        {
            s.equivalent_transform();
            t.equivalent_transform();
        }

        // 用最简NFA构造法
        // 初始化state_table
        let mut p_state_table: Vec<State> = Vec::new();
        // 加入开始状态
        p_state_table.push(State {
            state_id: 0,
            state_type: StateType::UNMATCH,
            category: LexemeCategory::EMPTY,
        });
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
            // graph加入p_state_table的索引应等于原索引(index)+self的结束状态序号-1
            next_state.state_id = (index + self.p_state_table.len() - 2) as i32;
            p_state_table.push(next_state);
        }
        // 加入结束状态
        p_state_table.push(State {
            state_id: p_state_table.len() as i32,
            state_type: StateType::MATCH,
            category: LexemeCategory::EMPTY,
        });
        // 初始化边数组p_edge_table
        let mut p_edge_table: Vec<Edge> = Vec::new();
        // 把原来的边更改到正确的序号后加入p_edge_table中
        for (index, item) in self.p_edge_table.iter().enumerate() {
            let mut edge = item.clone();
            // 如果该边的next_state为原来的结束状态的state_id(self.p_state_table.len()-1)改为当前的结束状态的state_id
            if item.next_state == (self.p_state_table.len() - 1) as i32 {
                edge.next_state = (p_state_table.len() - 1) as i32;
            }
            p_edge_table.push(edge);
        }
        for (index, item) in graph.p_edge_table.iter().enumerate() {
            let mut edge = item.clone();
            edge.from_state = (item.from_state as usize + self.p_state_table.len() - 2) as i32;
            edge.next_state = (item.from_state as usize + self.p_state_table.len() - 2) as i32;
            // 如果该边的from_state为原来的开始状态的state_id即0，改为当前的开始状态0
            if item.from_state == 0 {
                edge.from_state = 0;
            }
            p_edge_table.push(edge);
        }
        Graph {
            graph_id: 0,
            num_of_states: p_state_table.len() as i32,
            p_edge_table,
            p_state_table,
        }
    }

    /// 最简NFA构造法：**连接运算** s·t
    fn product(&self, graph: &Graph) -> Graph {
        // 分为两种情况,以下为共同行为
        let mut p_state_table: Vec<State> = Vec::new();
        let mut p_edge_table: Vec<Edge> = Vec::new();
        for item in self.p_state_table.iter() {
            p_state_table.push(item.clone());
        }
        // 把s的结束状态的StateType由MATCH改为UNMATCH
        let end_pos = p_state_table.len() - 1;
        p_state_table[end_pos].state_type = StateType::UNMATCH;
        // s的边不变
        for item in self.p_edge_table.iter() {
            p_edge_table.push(item.clone());
        }

        // 当 s 的 NFA 的结束状态 s 有出边且 t 的 NFA 的开始状态 0 有入边时
        if self.is_end_state_has_edge_out() && graph.is_start_state_has_edge_in() {
            for (index, item) in graph.p_state_table.iter().enumerate() {
                let mut state = item.clone();
                // t的状态在新的graph中的序号应等于原序号+s的终止状态序号+1
                state.state_id = (self.p_state_table.len() + index) as i32;
                p_state_table.push(state);
            }
            // 加入一条s到s+1的空转换边
            p_edge_table.push(Edge {
                from_state: (self.p_state_table.len() - 1) as i32,
                next_state: self.p_state_table.len() as i32,
                driver_id: -1,
                driver_type: DriverType::NULL,
            });
            // t的边的序号为 原来的序号+s的终止状态序号+1
            for item in graph.p_edge_table.iter() {
                let mut edge = item.clone();
                edge.from_state = item.from_state + self.p_state_table.len() as i32;
                edge.next_state = item.next_state + self.p_state_table.len() as i32;
                p_edge_table.push(edge);
            }
        // 其他情形
        } else {
            for (index, item) in graph.p_state_table.iter().enumerate() {
                let mut state = item.clone();
                // t的状态在新的graph中的序号应等于原序号+s的终止状态序号
                state.state_id = (self.p_state_table.len() - 1 + index) as i32;
                p_state_table.push(state);
            }
            for item in graph.p_edge_table.iter(){
                let mut edge = item.clone();
                edge.from_state = item.from_state + self.p_state_table.len() as i32 -1;
                edge.next_state = item.next_state + self.p_state_table.len() as i32 -1;
                p_edge_table.push(edge);
            }
        }

        Graph {
            graph_id: 0,
            num_of_states: p_state_table.len() as i32,
            p_edge_table,
            p_state_table,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {}
}
