use std::{cmp::max, collections::{BTreeMap, HashMap, VecDeque}, fmt::{Debug, Display}, hash::Hash, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq, Ord, Hash)]
pub struct Tree<T: Debug + Display + Hash> {
    pub left: Option<Arc<Tree<T>>>,
    pub right: Option<Arc<Tree<T>>>,
    pub data: T,
}

impl <T: Debug + Display + Hash> Tree<T> {

    pub fn new(data: T) -> Self {
        return Self {
            left: None,
            right: None,
            data,
        };
    }

    pub fn new_with_child(left: Arc<Tree<T>>, right: Arc<Tree<T>>, data: T) -> Self {

        return Self {
            left: Some(left),
            right: Some(right),
            data,
        };
    }

    /// Method for displaying the values of inside the tree.
    /// This method was built with the following ideas:
    /// All the strings in this vec should be of the same size.
    /// All vecs should be at least of length 1.
    pub fn display_string(&self) -> Vec<String> {

        /// Pushes the strings more to the right with whitespace
        fn push_right(in_vec: Vec<String>, n: usize) -> Vec<String> {

            let mut out_vec = Vec::with_capacity(in_vec.len());

            for row in in_vec {
                out_vec.push(" ".repeat(n) + &row);
            }

            return out_vec;

        }

        /// Pushes the strings more to the left with whitespace
        fn push_left(in_vec: Vec<String>, n: usize) -> Vec<String> {

            let mut out_vec = Vec::with_capacity(in_vec.len());

            for row in in_vec {
                out_vec.push(row + &" ".repeat(n));
            }

            return out_vec;

        }

        /// Function for merging two vecs of strings together like this:
        /// merge(["1", "2", "3"], ["1", "2", "3"]) -> ["11", "22", "33"]
        /// Assumes all rows are the same width
        fn merge(in1: Vec<String>, in2: Vec<String>, middle_width: usize) -> Vec<String> {

            let width1 = in1.get(0).unwrap_or(&"".to_string()).chars().count();
            let width2 = in2.get(0).unwrap_or(&"".to_string()).chars().count();
            let mut out_vec: Vec<String> = Vec::new();

            let middle = " ".repeat(middle_width);

            for i in 0..max(width1, width2) {

                out_vec.push(String::new());

                let empty = String::new();

                let left = in1.get(i).unwrap_or(&empty);
                let right = in2.get(i).unwrap_or(&empty);

                out_vec[i] = left.to_string() + &middle + &right;

            }

            return out_vec;
        }

        fn make_same_width(data: Vec<String>, width: usize) -> Vec<String> {

            let mut out_vec: Vec<String> = Vec::new();

            for i in 0..data.len() {

                let data_length = data[i].chars().count();

                if data_length < width {
                    out_vec.push(data[i].clone() + &" ".repeat(width - data_length));
                } else {
                    out_vec.push(data[i].clone());
                }

            }

            return out_vec;

        }

        let data = format!("{}", self.data);

        let mut out: Vec<String>;
        let mut out_width = data.chars().count();

        let mut left_data = Vec::new();
        let mut right_data = Vec::new();

        // Gets the top row
        out = vec![
            format!("{}", data),
        ];

        if let Some(l) = &self.left {

            left_data = l.display_string();
            left_data = push_left(left_data, out.len() / 2);

            out[0] = "  ".to_string() + &out[0];
            if out.len() == 1 { out.push(format!("{}", " ".repeat(data.chars().count()))); }
            out[1] = " /".to_string() + &out[1];
            out_width += 1;

        }

        if let Some(r) = &self.right {

            right_data = r.display_string();
            right_data = push_right(right_data, (out.len() - 1) / 2);

            out[0] = out[0].clone() + "  ";
            if out.len() == 1 { out.push(format!("{}", " ".repeat(data.chars().count()))); }
            out[1] = out[1].clone() + "\\ ";
            out_width += 1;

        }

        let left_width = left_data.get(0).unwrap_or(&String::new()).chars().count();

        if left_width > 3 { out = push_right(out, left_width - 2); }
        let mut new_rows = merge(left_data, right_data, out_width);
        let new_row_width = new_rows.get(0).unwrap_or(&String::new()).chars().count();

        out.append(&mut new_rows);
        out = make_same_width(out, new_row_width);

        let mut last_count = 0;
        for i in 0..out.len() {
            if out.get(i).unwrap().trim().len() == 0 {
                break;
            }
            last_count += 1;
        }

        return out.split_at(last_count).0.to_vec(); // removes empty lines

    }

}

impl <T: Debug + Display + Hash + Eq + PartialOrd + Clone> Tree<Node<T>> {

    /// Creates a encoding table and encodes data.
    pub fn encode(&self, data: &[T]) -> (Vec<bool>, HashMap<T, VecDeque<bool>>) {
        let table = self.make_table();
        return (self.encode_from_table(data, &table), table);
    }

    pub fn encode_from_table(&self, data: &[T], table: &HashMap<T, VecDeque<bool>>) -> Vec<bool> {

        let mut out_vec: Vec<bool> = Vec::new();

        for entry in data {
            let mut entry_data = match &table.get(entry) {
                Some(v) => (*v).clone(),
                None => panic!("Can't find value `{}` in table!", entry),
            };

            out_vec.append(&mut entry_data.make_contiguous().to_vec());
        }

        return out_vec;
    }

    pub fn decode(&self, data: &[bool]) -> Vec<T> {

        let mut out_vec: Vec<T> = Vec::new();
        let root = Arc::new(self.clone());
        let mut tmp_tree = root.clone();

        for i in data {

            if let Some(v) = &tmp_tree.data.data {
                out_vec.push(v.clone());
                tmp_tree = root.clone();
            }

            if *i {
                tmp_tree = tmp_tree.right.clone().unwrap();
            } else {
                tmp_tree = tmp_tree.left.clone().unwrap();
            }

        }

        if let Some(v) = &tmp_tree.data.data {
            out_vec.push(v.clone());
        }

        return out_vec;

    }

    /// Recursive method for creating a table for encoding text.
    pub fn make_table(&self) -> HashMap<T, VecDeque<bool>> {

        let mut out_map: HashMap<T, VecDeque<bool>> = HashMap::new();

        if let Some(k) = &self.data.data {
            out_map.insert(k.clone(), VecDeque::new());
        }

        if let Some(v) = &self.left {
            for (k, v) in v.make_table().iter() {
                let mut new_vec = v.clone();
                new_vec.push_front(false);
                // Panics on duplicate values
                out_map.insert(k.to_owned(), new_vec).ok_or(()).unwrap_err();
            }
        }

        if let Some(v) = &self.right{
            for (k, v) in v.make_table().iter() {
                let mut new_vec = v.clone();
                new_vec.push_front(true);
                // Panics on duplicate values
                out_map.insert(k.to_owned(), new_vec).ok_or(()).unwrap_err();
            }
        }

        return out_map;

    }

}

impl Tree<Node<char>> {

    pub fn from_str(data: &str) -> Arc<Self> {

        let mut queue: Vec<Arc<Tree<Node<char>>>> = {
            let mut value_map: BTreeMap<char, i32> = BTreeMap::new();

            // We initialize the value map (for probabilities)
            for v in data.chars() {
                match value_map.get(&v) {
                    Some(entry_value) => { value_map.insert(v, entry_value + 1); },
                    None => {              value_map.insert(v, 1).ok_or(()).unwrap_err(); },
                }
            }

            value_map
                .iter()
                .map(|(&k, &v)| Arc::new(Self::new(Node::new(Some(k), v))))
                .collect()
        };

        while queue.len() > 1 {
            queue.sort();

            let left = queue.remove(0);
            let right = queue.remove(0);

            // Creates a new node with left and right as subnodes.
            let new_node = Arc::new(
                Self::new_with_child(
                    left.clone(), right.clone(),
                    Node::new(
                        None,
                        left.data.prob + right.data.prob
                    )
                )
            );

            queue.push(new_node);

        }

        // Clones is only on Arc<T>.
        // The queue gets dropped then Arc<T> has a count of 1 after the function returns.
        return queue[0].clone();

    }

}

impl <T: PartialOrd + Display + Debug + Hash> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.data.partial_cmp(&other.data);
    }
}

impl <T: Display + Debug + Hash + Clone> Display for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.display_string().join("\n"));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, Hash)]
pub struct Node<T: Debug + PartialOrd + Hash> {
    pub data: Option<T>,
    pub prob: i32,
}

impl <T: Debug + PartialOrd + Hash> Node<T> {

    pub fn new(data: Option<T>, prob: i32) -> Self {
        return Self {
            data,
            prob,
        };
    }

    pub fn new_empty() -> Self {
        return Self::new(None, 0);
    }

}

impl <T: Debug + PartialOrd + Hash> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.prob.partial_cmp(&other.prob);
    }
}

impl <T: Debug + PartialOrd + Hash + Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match &self.data {
            Some(v) => format!("{}", v),
            None => "\u{2205}".to_string(),
        };
        return write!(f, "'{}'{}", value, self.prob);
    }
}
