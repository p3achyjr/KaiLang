use crate::ir::*;
use std::{collections::HashMap, fmt::Display};
use std::{collections::HashSet, fmt};

pub struct BasicBlock {
  ancestors: Vec<usize>,
  predecessors: Vec<usize>,
  cmd_range: (usize, usize),
  cmds: Vec<IrCmd>, // range of commands this basic block encapsulates
  phis: Vec<IrCmd>, // phi functions that go at the top of this bb
}

pub struct ControlFlowGraph<'a> {
  pub basic_blocks: &'a mut Vec<BasicBlock>,
}

impl BasicBlock {
  pub fn ancestors(&self) -> &Vec<usize> {
    return &self.ancestors;
  }

  pub fn predecessors(&self) -> &Vec<usize> {
    return &self.predecessors;
  }

  pub fn cmds(&self) -> &Vec<IrCmd> {
    return &self.cmds;
  }

  pub fn cmds_mut(&mut self) -> &mut Vec<IrCmd> {
    return &mut self.cmds;
  }

  pub fn phis(&self) -> &Vec<IrCmd> {
    return &self.phis;
  }

  pub fn phis_mut(&mut self) -> &mut Vec<IrCmd> {
    return &mut self.phis;
  }

  pub fn add_new_phi(&mut self, var: i32, ty: IrType) {
    self
      .phis
      .push(IrCmd::Asgn(IrVar::Temp(var, ty, 0), IrExpr::Phi(vec![])));
  }

  // pub fn add_new_phi(&mut self, var: i32) {
  //   self.phis.push((var, IrExpr::Phi(vec![])));
  // }

  // pub fn add_new_version(&mut self, var: i32, version: usize) {
  //   for i in 0..self.phis.len() {
  //     let (v, phi) = &self.phis[i];
  //     if *v != var {
  //       continue;
  //     }
  //     self.phis[i] = match phi {
  //       IrExpr::Phi(phis) => {
  //         phis.push(IrVar::Temp(var, version));
  //         (*v, IrExpr::Phi(phis))
  //       }
  //       _ => panic!("Should never see non-PHI expr in BB"),
  //     };

  //     return;
  //   }

  //   // did not find existing phis, so add a new one
  //   self
  //     .phis
  //     .push((var, IrExpr::Phi(vec![IrVar::Temp(var, version)])));
  // }
}

impl<'a> ControlFlowGraph<'a> {
  pub fn at(&self, u: usize) -> &BasicBlock {
    &self.basic_blocks[u]
  }

  pub fn at_mut(&mut self, u: usize) -> &mut BasicBlock {
    &mut self.basic_blocks[u]
  }

  pub fn basic_blocks(&self) -> &Vec<BasicBlock> {
    return self.basic_blocks;
  }

  pub fn postorder(&self) -> Vec<usize> {
    let mut ordering = vec![];
    self._postorder(0, &mut ordering, &mut HashSet::new());

    ordering
  }

  pub fn build_cfg(&mut self, ir: &IrFunction) -> &ControlFlowGraph<'a> {
    let cmds = &ir.body;
    let label_to_line_map = self.find_labels(&ir.body);
    self.build_and_insert_block(cmds, 0, &label_to_line_map, &mut HashMap::new());

    self
  }

  fn _postorder(&self, u: usize, traversal: &mut Vec<usize>, visited: &mut HashSet<usize>) {
    if visited.contains(&u) {
      return;
    }
    visited.insert(u);
    for v in self.basic_blocks[u].ancestors() {
      self._postorder(*v, traversal, visited);
    }
    traversal.push(u);
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

  fn new_bb(&self, start: usize, end: usize, cmds: &Vec<IrCmd>) -> BasicBlock {
    let mut section = vec![];
    for i in start..end {
      section.push(cmds[i].clone());
    }
    BasicBlock {
      ancestors: vec![],
      predecessors: vec![],
      cmd_range: (start, end),
      cmds: section,
      phis: vec![],
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
          let bb = self.new_bb(line, line_, cmds);
          let bb_index = self.basic_blocks.len();
          self.basic_blocks.push(bb);
          line_bb_index_map.insert(line, bb_index);
          let ancestor_bb_index =
            self.build_and_insert_block(cmds, line_, label_to_line_map, line_bb_index_map);
          self.link(bb_index, ancestor_bb_index);
          return bb_index;
        }
        IrCmd::Goto(label) => {
          let bb = self.new_bb(line, line_ + 1, cmds);
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
          let bb = self.new_bb(line, line_ + 1, cmds);
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
          let bb = self.new_bb(line, line_ + 1, cmds);
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

    for i in 0..self.basic_blocks.len() {
      write!(f, "{}:\n", i,)?;
      write!(f, "phis: ")?;
      for phi in &self.basic_blocks[i].phis {
        write!(f, "{}\n", phi)?;
      }
      write!(f, "cmds: ")?;
      for cmd in &self.basic_blocks[i].cmds {
        write!(f, "{}\n", cmd)?;
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}
