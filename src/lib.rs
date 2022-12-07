use std::fmt;

#[derive(Clone)]
pub struct Node{
    value: i32,
    left_branch: Option<Box<Node>>,
    right_branch: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32)->Node{
        Node { value: value, left_branch: None, right_branch: None }
    }

    pub fn insert(&mut self, value: i32){
        if self.value == value{
            return
        }
        let new_node = Some(Box::new(Node::new(value)));
        if value < self.value {
            match self.left_branch.as_mut() {
                None => self.left_branch = new_node,
                Some(left) => left.insert(value),
            }
        } else {
            match self.right_branch.as_mut() {
                None => self.right_branch = new_node,
                Some(right) => right.insert(value),
            }
        }
    }

    pub fn search(&self, target: i32)->bool{
      match self.value{
        value if target == value => true,
        value if target < value => self.left_branch.as_ref().unwrap().search(target),
        value if target > value => self.right_branch.as_ref().unwrap().search(target),
        _ => false,
      }  
    }

    fn print_pre_order(&self, tree_print: &mut Vec<String>, (mut padding_left, mut padding_right): (i32, i32), mut max_padding: i32)-> String{
               
        max_padding = max_padding.max(padding_left+padding_right);
        
        match padding_left + padding_right {
            0 => print!(""),
            1 => tree_print.push("|-".to_string()),
            _ => {
                tree_print.push(" ".to_string().repeat((padding_left + padding_right -1) as usize));
                tree_print.push("|-".to_string());
            },
        }
        tree_print.push(self.value.to_string());
        tree_print.push("\n".to_string());

        match self.left_branch{
            Some(_) =>{
                padding_left += 1;
                self.left_branch.as_ref().unwrap().print_pre_order(tree_print,(padding_left, padding_right),max_padding);
            }             
            None => (),
        }
        match self.right_branch{
            Some(_) =>{
                if padding_left + padding_right > max_padding{
                    padding_left -= 1;
                }
                padding_right += 1;
                
                self.right_branch.as_ref().unwrap().print_pre_order(tree_print, (padding_left, padding_right),max_padding);
            },
            None => (),
        }
        tree_print.join("")
    }

}

impl fmt::Display for Node{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{}", self.print_pre_order(&mut vec!["".to_string()],(0,0),0))
    }
}
