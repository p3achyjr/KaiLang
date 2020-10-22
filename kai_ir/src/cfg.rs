use crate::ir::*;
use std::fmt;
use std::{collections::HashMap, fmt::Display};

pub struct BasicBlock {
  ancestors: Vec<usize>,
  predecessors: Vec<usize>,
  cmd_range: (usize, usize), // range of commands this basic block encapsulates
}

pub struct ControlFlowGraph<'a> {
  pub basic_blocks: &'a mut Vec<BasicBlock>,
}

impl<'a> ControlFlowGraph<'a> {
  pub fn build_cfg(&mut self, ir: &IrFunction) -> &ControlFlowGraph<'a> {
    let cmds = &ir.body;
    let label_to_line_map = self.find_labels(&ir.body);
    self.build_and_insert_block(cmds, 0, &label_to_line_map, &mut HashMap::new());

    self
  }

  fn find_labels(&self, cmds: &Vec<IrCmd>) -> HashMap<IrLabel, usize> {
    let mut map = HashMap::new();

    for i in 0..cmds.len() {
      match cmds[i] {
        IrCmd::Label(label) => map.insert(label, i),
        _ => None,
      };
    }

    map
  }

  fn new_bb(&self, start: usize, end: usize) -> BasicBlock {
    BasicBlock {
      ancestors: vec![],
      predecessors: vec![],
      cmd_range: (start, end),
    }
  }

  fn link(&mut self, pred_bb_index: usize, succ_bb_index: usize) {
    self.basic_blocks[pred_bb_index]
      .ancestors
      .push(succ_bb_index);
    self.basic_blocks[succ_bb_index]
      .predecessors
      .push(pred_bb_index);
  }

  // fn new_basic_block(start:i32, end:i32, line_bb_index_map: &HashMap<i32, i32>) -> i32 {
  //   let bb = self.new_bb(line, line_);
  //   let bb_index = self.basic_blocks.len();
  //   self.basic_blocks.push(&mut bb);
  //   line_bb_index_map.insert(line, bb_index);

  //   bb_index
  // }

  // returns an index into self.basic_blocks
  fn build_and_insert_block(
    &mut self,
    cmds: &Vec<IrCmd>,
    line: usize,
    label_to_line_map: &HashMap<IrLabel, usize>,
    line_bb_index_map: &mut HashMap<usize, usize>,
  ) -> usize {
    if line_bb_index_map.contains_key(&line) {
      return *line_bb_index_map.get(&line).unwrap();
    }

    // we will scan cmds starting from line, looking for a <cond>, <goto>, or <label>
    // once we find a cond or goto, we will insert the current range as a basic block
    // into self.basic_blocks, and into line_bb_map.
    // we will then go to the line of each next label, build/insert the basic block there,
    // and return those basic blocks to the calling function.
    for line_ in (line + 1)..cmds.len() {
      match cmds[line_] {
        IrCmd::Label(_) => {
          let bb = self.new_bb(line, line_);
          let bb_index = self.basic_blocks.len();
          self.basic_blocks.push(bb);
          line_bb_index_map.insert(line, bb_index);
          let ancestor_bb_index =
            self.build_and_insert_block(cmds, line_, label_to_line_map, line_bb_index_map);
          self.link(bb_index, ancestor_bb_index);
          return bb_index;
        }
        IrCmd::Goto(label) => {
          let bb = self.new_bb(line, line_ + 1);
          let bb_index = self.basic_blocks.len();
          self.basic_blocks.push(bb);
          line_bb_index_map.insert(line, bb_index);
          let ancestor_bb_index = self.build_and_insert_block(
            cmds,
            *label_to_line_map.get(&label).unwrap(),
            label_to_line_map,
            line_bb_index_map,
          );
          self.link(bb_index, ancestor_bb_index);
          return bb_index;
        }
        IrCmd::Cond(_, label1, label2) => {
          let bb = self.new_bb(line, line_ + 1);
          let bb_index = self.basic_blocks.len();
          self.basic_blocks.push(bb);
          line_bb_index_map.insert(line, bb_index);
          let ancestor_bb_index_1 = self.build_and_insert_block(
            cmds,
            *label_to_line_map.get(&label1).unwrap(),
            label_to_line_map,
            line_bb_index_map,
          );
          let ancestor_bb_index_2 = self.build_and_insert_block(
            cmds,
            *label_to_line_map.get(&label2).unwrap(),
            label_to_line_map,
            line_bb_index_map,
          );
          self.link(bb_index, ancestor_bb_index_1);
          self.link(bb_index, ancestor_bb_index_2);
          return bb_index;
        }
        IrCmd::Return(_) => {
          let bb = self.new_bb(line, line_ + 1);
          let bb_index = self.basic_blocks.len();
          self.basic_blocks.push(bb);
          line_bb_index_map.insert(line, bb_index);

          // no outgoing edges
          return bb_index;
        }
        _ => continue,
      }
    }

    panic!("should never reach here");
  }

  // fn find_next_branch_index(cmds: Vec<IrCmd>, line: usize) -> i32 {
  //   for line_ in (line + 1)..cmds.len() {
  //     match cmds[line_] {
  //       IrCmd::Label(_) => return line_, // bb stops at line before this
  //       IrCmd::Goto(_) => return line_ + 1,
  //       IrCmd::Cond(_, _, _) => return line_ + 1,
  //       IrCmd::Ret(_) => return line_ + 1,
  //     }
  //   }
  // }
}

impl<'a> Display for ControlFlowGraph<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for i in 0..self.basic_blocks.len() {
      write!(
        f,
        "{}, next: {:?}, prev: {:?} range: {:?}\n",
        i,
        self.basic_blocks[i].ancestors,
        self.basic_blocks[i].predecessors,
        self.basic_blocks[i].cmd_range,
      )?;
    }

    Ok(())
  }
}
