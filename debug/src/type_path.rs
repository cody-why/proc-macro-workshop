/*** 
 * @Author: plucky
 * @Date: 2022-07-20 22:49:28
 * @LastEditTime: 2022-07-20 23:55:37
 * @Description: 
 */

use std::collections::HashMap;

use syn::{visit::{self, Visit}};

// 定义一个用于实现`Visit` Trait的结构体
pub struct TypePathVisitor{
   pub generic_type_names: Vec<String>,  // 这个是筛选条件，里面记录了所有的泛型参数的名字，例如T,U,V
   pub associated_types: HashMap<String, Vec<syn::TypePath>>,  // 这里记录了所有满足条件的语法树节点,TypePath可能有多个
}

impl<'ast> Visit<'ast> for TypePathVisitor {
    // visit_type_path 遍历自动回调所有符合TypePath的节点
    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        // 例如T:Velue
        if node.path.segments.len() >= 2 {
            // 取出泛型参数的名字,例如T,U
            let generic_type_name = node.path.segments[0].ident.to_string();

            if self.generic_type_names.contains(&generic_type_name) {
                // 如果满足上面的两个筛选条件，那么就把结果存起来
                self.associated_types.entry(generic_type_name).or_insert(Vec::new()).push(node.clone());
            }
        }

        // 继续遍历下去直到所有节点
        visit::visit_type_path(self, node);
    }
}

impl TypePathVisitor {
    
}
    
