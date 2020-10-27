use crate::{cfg::*, ir::*};
use std::{
  collections::HashSet,
  collections::{HashMap, VecDeque},
  convert::TryInto,
};

// dfs
// fn _postorder(
//   cfg: &ControlFlowGraph,
//   u: usize,
//   traversal: &mut Vec<usize>,
//   visited: &mut HashSet<usize>,
// ) {
//   if visited.contains(&u) {
//     return;
//   }
//   visited.insert(u);
//   for v in cfg.basic_blocks[u].ancestors() {
//     _postorder(cfg, *v, traversal, visited);
//   }
//   traversal.push(u);
// }

// fn postorder(cfg: &ControlFlowGraph) -> Vec<usize> {
//   let mut ordering = vec![];
//   _postorder(cfg, 0, &mut ordering, &mut HashSet::new());

//   ordering
// }

fn first_processed_pred(
  node: usize,
  cfg: &ControlFlowGraph,
  dominators: &Vec<Option<usize>>,
) -> Option<usize> {
  let node_bb = cfg.at(node);
  let preds = node_bb.predecessors();

  for pred in preds {
    match dominators[*pred] {
      None => {
        continue;
      }
      Some(_) => {
        return Some(*pred);
      }
    }
  }

  None
}

fn intersect(
  u: usize,
  v: usize,
  dominators: &Vec<Option<usize>>,
  node_to_postorder: &Vec<usize>,
) -> usize {
  let mut finger1 = u;
  let mut finger2 = v;

  while node_to_postorder[finger1] != node_to_postorder[finger2] {
    while node_to_postorder[finger1] < node_to_postorder[finger2] {
      finger1 = dominators[finger1].unwrap();
    }
    while node_to_postorder[finger2] < node_to_postorder[finger1] {
      finger2 = dominators[finger2].unwrap();
    }
  }

  assert!(
    finger1 == finger2,
    "fingers should be equal o/w idk what's happening",
  );

  finger1
}

// find dominators
// I know this is confusing as hell, I am sorry
// consult ```A Simple, Fast, Dominance Algorithm```
fn dominators(cfg: &ControlFlowGraph) -> Vec<usize> {
  let postorder = cfg.postorder();
  let mut rev_postorder = postorder.clone();
  rev_postorder.reverse();

  let mut node_to_postorder: Vec<usize> = vec![usize::MAX; postorder.len()]; // map from bb index to postorder number
  for i in 0..postorder.len() {
    node_to_postorder[postorder[i]] = i;
  }
  println!("postorder: {:?}", postorder);
  println!("rev_postorder: {:?}", rev_postorder);
  println!("node_to_postorder: {:?}", node_to_postorder);
  let mut dominators = vec![None; cfg.basic_blocks.len()];

  dominators[0] = Some(0);
  let mut changed = true;

  // run the internal procedure until we see that our dominator tree has not changed
  while changed {
    changed = false;
    for node in &rev_postorder {
      let node = *node;
      if node == 0 {
        continue;
      }

      let mut new_idom = first_processed_pred(node, &cfg, &dominators).unwrap();
      let node_bb = cfg.at(node);
      let preds = node_bb.predecessors();
      for pred in preds {
        if dominators[*pred] == None {
          continue;
        }

        new_idom = intersect(*pred, new_idom, &dominators, &node_to_postorder);
      }

      match dominators[node] {
        None => {
          dominators[node] = Some(new_idom);
          changed = true;
        }
        Some(idom) => {
          if idom != new_idom {
            dominators[node] = Some(new_idom);
            changed = true;
          }
        }
      }
    }
  }

  dominators.into_iter().map(|dom| dom.unwrap()).collect()
}

fn dominance_frontiers(cfg: &ControlFlowGraph, dominators: &Vec<usize>) -> Vec<HashSet<usize>> {
  let mut frontiers = vec![HashSet::new(); dominators.len()];
  for u in 0..cfg.basic_blocks.len() {
    let bb = cfg.at(u);
    if bb.ancestors().len() > 1 {
      continue;
    }

    for pred in bb.predecessors() {
      let mut runner = *pred;
      while runner != dominators[u] {
        frontiers[runner].insert(u);
        runner = dominators[runner];
      }
    }
  }

  return frontiers;
}

fn rename_lit(lit: &IrLiteral, var_stack: &Vec<usize>) -> IrLiteral {
  match lit {
    IrLiteral::Num(n) => IrLiteral::Num(*n),
    IrLiteral::Bool(b) => IrLiteral::Bool(*b),
    IrLiteral::Var(IrVar::Temp(var, ty, _)) => {
      IrLiteral::Var(IrVar::Temp(*var, *ty, var_stack[*var as usize]))
    }
    _ => panic!("Error in ssa::rename_lit, found ident temp"),
  }
}

fn rename_expr(expr: &IrExpr, var_stack: &Vec<usize>) -> IrExpr {
  match expr {
    // should not see phis at this point
    IrExpr::Phi(phis) => IrExpr::Phi(phis.clone()),
    IrExpr::Literal(lit) => IrExpr::Literal(rename_lit(lit, var_stack)),
    IrExpr::Binop(op, lit1, lit2) => IrExpr::Binop(
      *op,
      rename_lit(lit1, var_stack),
      rename_lit(lit2, var_stack),
    ),
  }
}

// dfs over cfg, rename each basic block
fn rename_cfg(
  cfg: &mut ControlFlowGraph,
  dom_tree: &Vec<Vec<usize>>,
  visited: &mut Vec<bool>,
  bb_index: usize,
  var_stack: &mut Vec<usize>,
) -> () {
  println!("var_stack: {:?}", var_stack);
  if visited[bb_index] {
    return;
  }
  visited[bb_index] = true;
  let bb = cfg.at_mut(bb_index);
  let bb_phis = bb.phis_mut();

  // rename defs in PHIs
  for i in 0..bb_phis.len() {
    let bb_phi = match &bb_phis[i] {
      IrCmd::Asgn(IrVar::Temp(var, ty, _), phi) => {
        let new_cmd = IrCmd::Asgn(
          IrVar::Temp(*var, *ty, var_stack[*var as usize] + 1),
          phi.clone(),
        );
        var_stack[*var as usize] += 1;
        new_cmd
      }
      _ => panic!("Error in ssa::rename_cfg, encountered non-PHI when scanning PHIs"),
    };

    bb_phis[i] = bb_phi;
  }

  let bb_cmds = bb.cmds_mut();

  // then rename the rest of the block
  for i in 0..bb_cmds.len() {
    let cmd = match &bb_cmds[i] {
      IrCmd::Asgn(IrVar::Temp(var, ty, _), expr) => {
        let new_cmd = IrCmd::Asgn(
          IrVar::Temp(*var, *ty, var_stack[*var as usize] + 1),
          rename_expr(expr, var_stack),
        );
        var_stack[*var as usize] += 1;
        new_cmd
      }
      IrCmd::Asgn(_, _) => panic!("Error in ssa::rename_cfg, should not have string temps now"),
      IrCmd::Return(e) => IrCmd::Return(rename_lit(e, var_stack)),
      IrCmd::Label(l) => IrCmd::Label(*l),
      IrCmd::Goto(l) => IrCmd::Goto(*l),
      IrCmd::Cond(c, e1, e2) => IrCmd::Cond(rename_lit(c, var_stack), *e1, *e2),
    };
    // println!("old cmd: {}, new cmd: {}", bb_cmds[i], cmd);
    bb_cmds[i] = cmd;
  }

  let ancestors = cfg.at(bb_index).ancestors().clone();
  // add new version of each var to each ancestor in CFG
  for ancestor_index in ancestors {
    let bb = cfg.at_mut(ancestor_index);
    let bb_phis = bb.phis_mut();
    for i in 0..bb_phis.len() {
      match &mut bb_phis[i] {
        IrCmd::Asgn(IrVar::Temp(var, ty, _), IrExpr::Phi(phis)) => {
          phis.push((
            IrVar::Temp(*var, *ty, var_stack[*var as usize]),
            IrLabel {
              label: bb_index as i32,
            },
          ));
        }
        _ => panic!("Error in ssa::rename_cfg, encountered non-PHI when scanning PHIs"),
      }
    }
  }

  // for child in dominator tree, rename all variables
  for bb_child in &dom_tree[bb_index] {
    rename_cfg(cfg, dom_tree, visited, *bb_child, var_stack);
  }

  // pop all defs in current block from var_stack
  // do this by iterating through commands, and when we see a def of v, subtract
  // from var_stack
  let bb_cmds_immut = cfg.at(bb_index).cmds();
  for i in 0..bb_cmds_immut.len() {
    match &bb_cmds_immut[i] {
      IrCmd::Asgn(IrVar::Temp(var, ty, _), _) => {
        var_stack[*var as usize] -= 1;
      }
      _ => {}
    };
  }
}

/*
 * top level renaming variable subroutine
 * for each basic block:
 * - replace non-phi uses of v with most recent version of v
 * - for each def of v, increment counter for v
 * - for each ancestor of node, add v to PHI node with most recent version
 * - recuse on each child of dominator tree
 */
fn rename_vars_and_insert_phis(
  cfg: &mut ControlFlowGraph,
  dom_tree: &Vec<Vec<usize>>,
  // map from variable to all bb indices that need a PHI node for that variable
  phi: &Vec<HashSet<usize>>,
  tmp_ty_map: &HashMap<i32, IrType>,
  tmp_count: usize,
) -> () {
  // mapping from temp number to current version
  let mut var_stack = vec![0; tmp_count];
  // mapping from block to actual phi definitions at the beginning
  for u in 0..phi.len() {
    let bbs = &phi[u];
    for bb in bbs {
      cfg
        .at_mut(*bb)
        .add_new_phi(u as i32, *tmp_ty_map.get(&(u as i32)).unwrap());
      // phis_per_block[*bb].insert(u, vec![]);
    }
  }

  // will rename all cmds in place, and populate phis_per_block
  rename_cfg(
    cfg,
    dom_tree,
    &mut vec![false; cfg.basic_blocks().len()],
    0,
    &mut var_stack,
  );

  println!("cfg: {}", cfg);
}

// fn rebuild_ir_body(body: &mut Vec<IrCmd>, cfg: &ControlFlowGraph) {
//   // bfs
//   let mut queue = VecDeque::new();
//   let mut visited = vec![false; cfg.basic_blocks().len()];
//   visited[0] = true;
//   queue.push_back(0);

//   while queue.len() > 0 {
//     println!("queue: {:?}", queue);
//     let u = queue.pop_front().unwrap();
//     let bb = cfg.at(u);
//     body.push(IrCmd::Label(IrLabel { label: u as i32 }));
//     for phi in bb.phis() {
//       body.push(phi.clone());
//     }

//     let mut seen_jmp = false;
//     for cmd in bb.cmds() {
//       match cmd {
//         // we will be generating new labels
//         IrCmd::Label(_) => {
//           continue;
//         }
//         IrCmd::Cond(c, _l1, _l2) => {
//           assert!(
//             !seen_jmp,
//             "ssa::rebuild_ir_body: two branches within a single basic block",
//           );
//           assert!(
//             bb.ancestors().len() == 2,
//             "ssa::rebuild_ir_body: saw conditional, but number of branches != 2"
//           );

//           seen_jmp = true;
//           body.push(IrCmd::Cond(
//             c.clone(),
//             IrLabel {
//               label: bb.ancestors()[0] as i32,
//             },
//             IrLabel {
//               label: bb.ancestors()[1] as i32,
//             },
//           ));
//         }
//         IrCmd::Goto(l) => {
//           assert!(
//             !seen_jmp,
//             "ssa::rebuild_ir_body: two jmps within a single basic block",
//           );
//           assert!(
//             bb.ancestors().len() == 1,
//             "ssa::rebuild_ir_body: saw goto, but number of branches != 1"
//           );

//           seen_jmp = true;
//           body.push(IrCmd::Goto(IrLabel {
//             label: bb.ancestors()[0] as i32,
//           }));
//         }
//         _ => {
//           body.push(cmd.clone());
//         }
//       }
//     }

//     // if we still haven't seen a jump, we need to add one
//     if seen_jmp == false && bb.ancestors().len() > 0 {
//       println!("{}, {:?}", u, bb.ancestors());
//       assert!(
//         bb.ancestors().len() == 1,
//         "if we are missing a goto, ancestors should be length 1"
//       );
//       body.push(IrCmd::Goto(IrLabel {
//         label: bb.ancestors()[0] as i32,
//       }));
//     }

//     for ancestor in bb.ancestors() {
//       if visited[*ancestor] {
//         continue;
//       }

//       visited[*ancestor] = true;
//       queue.push_back(*ancestor);
//     }
//   }
// }

fn ssa_ir(
  ir: &mut IrFunction,
  cfg: &mut ControlFlowGraph,
  dom_tree: Vec<Vec<usize>>,
  dfs: Vec<HashSet<usize>>,
  tmp_ty_map: &HashMap<i32, IrType>,
  tmp_count: usize,
) -> () {
  // map from bb index to variables defined in that bb
  let mut orig = vec![HashSet::new(); cfg.basic_blocks.len()];
  // map from variable to all bb indices it is defined in
  let mut defsites = vec![vec![]; tmp_count];
  // map from variable to all bb indices that need a PHI node for that variable
  let mut phi: Vec<HashSet<usize>> = vec![HashSet::new(); tmp_count];

  for bb_index in 0..cfg.basic_blocks().len() {
    let bb = cfg.at(bb_index);
    let cmds = bb.cmds();
    for cmd in cmds {
      match cmd {
        IrCmd::Asgn(IrVar::Temp(var, _, _), _) => {
          orig[bb_index].insert(var);
          defsites[*var as usize].push(bb_index);
        }
        _ => {
          continue;
        }
      }
    }
  }

  for var in 0..tmp_count {
    let work_list = &mut defsites[var];
    while work_list.len() > 0 {
      let bb_index = work_list.pop().unwrap();
      for succ in &dfs[bb_index] {
        if phi[var].contains(&succ) {
          continue;
        }

        phi[var].insert(*succ);
        if !orig[*succ].contains(&(var as i32)) {
          // why do we want to add the bb in the dominance frontier if it
          // does not define `var`?
          // it is because we will add a phi node to this node for var,
          // which counts as a definition
          work_list.push(*succ);
        }
      }
    }
  }

  rename_vars_and_insert_phis(cfg, &dom_tree, &phi, tmp_ty_map, tmp_count);

  // rebuild IR by traversing the CFG
  // let mut new_body = vec![];
  // rebuild_ir_body(&mut new_body, &cfg);
  // ir.body = new_body;

  println!("{}", ir);
}

pub fn gen_ssa(
  ir: &mut IrFunction,
  cfg: &mut ControlFlowGraph,
  tmp_ty_map: &HashMap<i32, IrType>,
  tmp_count: usize,
) -> () {
  let dominators = dominators(cfg);
  let dominance_frontiers = dominance_frontiers(cfg, &dominators);
  let mut dom_tree = vec![vec![]; dominators.len()];
  for i in 0..dominators.len() {
    dom_tree[dominators[i]].push(i);
  }

  println!("dominators: {:?}", dominators);
  println!("frontiers: {:?}", dominance_frontiers);

  ssa_ir(
    ir,
    cfg,
    dom_tree,
    dominance_frontiers,
    tmp_ty_map,
    tmp_count,
  );
}
