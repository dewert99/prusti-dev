// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use encoder::vir;
use encoder::Encoder;
use self::branch_ctxt::*;
use std::collections::HashMap;
use std::collections::HashSet;
use encoder::vir::CfgReplacer;
use encoder::foldunfold::action::Action;
use encoder::foldunfold::perm::*;
use encoder::foldunfold::permissions::RequiredPermissionsGetter;
use encoder::vir::ExprFolder;
use encoder::vir::ExprIterator;
use prusti_interface::config;
use prusti_interface::report::Log;
use std;

mod perm;
mod permissions;
mod state;
mod branch_ctxt;
mod semantics;
mod places_utils;
mod action;


pub fn add_folding_unfolding(mut function: vir::Function, predicates: HashMap<String, vir::Predicate>) -> vir::Function {
    if function.body.is_none() {
        return function
    }

    let formal_vars = function.formal_args.clone();
    let mut bctxt = BranchCtxt::new(formal_vars, &predicates);
    // Inhale preconditions
    for pre in &function.pres {
        bctxt.apply_stmt(&vir::Stmt::Inhale(pre.clone()));
    }

    let body = function.body.unwrap();

    let perms: Vec<_> = body
        .get_required_permissions(&predicates)
        .into_iter()
        .collect();

    // Add appropriate unfolding around this expression
    let new_body = bctxt
        .obtain_permissions(perms)
        .into_iter()
        .rev()
        .fold(
            body,
            |expr, action| action.to_expr(expr)
        );

    function.body = Some(new_body);
    function
}


pub fn add_fold_unfold<'p, 'v: 'p, 'r: 'v, 'a: 'r, 'tcx: 'a>(encoder: &'p Encoder<'v, 'r, 'a, 'tcx>, cfg: vir::CfgMethod) -> vir::CfgMethod {
    let cfg_vars = cfg.get_all_vars();
    let predicates = encoder.get_used_viper_predicates_map();
    let initial_bctxt = BranchCtxt::new(cfg_vars, &predicates);
    FoldUnfold::new(encoder, initial_bctxt, &cfg).replace_cfg(&cfg)
}

#[derive(Clone)]
struct FoldUnfold<'p, 'v: 'p, 'r: 'v, 'a: 'r, 'tcx: 'a> {
    encoder: &'p Encoder<'v, 'r, 'a, 'tcx>,
    initial_bctxt: BranchCtxt<'p>,
    bctxt_at_label: HashMap<String, BranchCtxt<'p>>,
    dump_debug_info: bool,
    check_foldunfold_state: bool,
    cfg: &'p vir::CfgMethod,
}

impl<'p, 'v: 'p, 'r: 'v, 'a: 'r, 'tcx: 'a> FoldUnfold<'p, 'v, 'r, 'a, 'tcx> {
    pub fn new(encoder: &'p Encoder<'v, 'r, 'a, 'tcx>, initial_bctxt: BranchCtxt<'p>, cfg: &'p vir::CfgMethod) -> Self {
        FoldUnfold {
            encoder,
            initial_bctxt,
            bctxt_at_label: HashMap::new(),
            dump_debug_info: config::dump_debug_info(),
            check_foldunfold_state: config::check_foldunfold_state(),
            cfg,
        }
    }

    fn replace_expr(&self, expr: &vir::Expr, curr_bctxt: &BranchCtxt<'p>) -> vir::Expr {
        ExprReplacer::new(curr_bctxt.clone(), &self.bctxt_at_label).fold(expr.clone())
    }

    fn replace_old_expr(&self, expr: &vir::Expr, curr_bctxt: &BranchCtxt<'p>) -> vir::Expr {
        expr.clone().map_old_expr(
            |label, inner_expr| {
                let inner_bctxt = &self.bctxt_at_label[label];
                self.replace_expr(&inner_expr, inner_bctxt).old(label)
            }
        )
    }

    /// Insert "unfolding in" in old expressions
    fn rewrite_stmt_with_unfoldings_in_old(&self, stmt: vir::Stmt, bctxt: &BranchCtxt<'p>) -> vir::Stmt {
        stmt.map_expr(|e| self.replace_old_expr(&e, bctxt))
    }

    /// Insert "unfolding in" expressions
    fn rewrite_stmt_with_unfoldings(&self, stmt: vir::Stmt, bctxt: &BranchCtxt<'p>) -> vir::Stmt {
        match stmt {
            vir::Stmt::Inhale(expr) => {
                // Compute inner state
                let mut inner_bctxt = bctxt.clone();
                let inner_state = inner_bctxt.mut_state();
                inner_state.insert_all_perms(
                    expr.get_permissions(bctxt.predicates())
                        .into_iter()
                        .filter(|p| p.is_pred())
                );

                // Rewrite statement
                vir::Stmt::Inhale(self.replace_expr(&expr, &inner_bctxt))
            }
            vir::Stmt::TransferPerm(lhs, rhs) => {
                // Compute rhs state
                let mut rhs_bctxt = bctxt.clone();
                /*
                let rhs_state = rhs_bctxt.mut_state();
                rhs_state.insert_all_perms(
                    rhs.get_permissions(bctxt.predicates())
                        .into_iter()
                        .filter(|p| p.is_pred())
                );
                */

                // Rewrite statement
                vir::Stmt::TransferPerm(
                    self.replace_expr(&lhs, &bctxt),
                    self.replace_expr(&rhs, &rhs_bctxt)
                )
            }
            _ => stmt.map_expr(|e| self.replace_expr(&e, bctxt)),
        }
    }
}

impl<'p, 'v: 'p, 'r: 'v, 'a: 'r, 'tcx: 'a> vir::CfgReplacer<BranchCtxt<'p>> for FoldUnfold<'p, 'v, 'r, 'a, 'tcx> {
    /// Dump the current CFG, for debugging purposes
    fn current_cfg(&self, new_cfg: &vir::CfgMethod) {
        if self.dump_debug_info {
            let source_path = self.encoder.env().source_path();
            let source_filename = source_path.file_name().unwrap().to_str().unwrap();
            let method_name = new_cfg.name();
            Log::report_with_writer(
                "graphviz_method_during_foldunfold",
                format!("{}.{}.dot", source_filename, method_name),
                |writer| new_cfg.to_graphviz(writer)
            );
        }
    }

    fn compatible_back_edge(left: &BranchCtxt<'p>, right: &BranchCtxt<'p>) -> bool {
        let left_state = left.state();
        let right_state = right.state();

        left_state.acc() == right_state.acc() &&
            left_state.pred() == right_state.pred() &&
            left_state.framing_stack() == right_state.framing_stack()
    }

    /// Give the initial branch context
    fn initial_context(&mut self) -> BranchCtxt<'p> {
        self.initial_bctxt.clone()
    }

    /// Replace some statements, mutating the branch context
    fn replace_stmt(&mut self, stmt: &vir::Stmt, is_last_before_return: bool, bctxt: &mut BranchCtxt<'p>) -> Vec<vir::Stmt> {
        debug!("replace_stmt: ----->>>>> {} <<<<<-----", stmt);
        let mut stmt = stmt.clone();

        // Store state for old expressions
        match stmt {
            vir::Stmt::Label(ref label) => {
                let mut labelled_bctxt = bctxt.clone();
                let labelled_state = labelled_bctxt.mut_state();
                labelled_state.replace_places(|place| place.old(label));
                self.bctxt_at_label.insert(label.to_string(), labelled_bctxt);
            }

            vir::Stmt::PackageMagicWand(vir::Expr::MagicWand(box ref lhs, _), ..) |
            vir::Stmt::ApplyMagicWand(vir::Expr::MagicWand(box ref lhs, _)) => {
                // TODO: This should be done also for magic wand expressions inside inhale/exhale.
                let label = "lhs".to_string();
                let mut labelled_bctxt = bctxt.clone();
                let labelled_state = labelled_bctxt.mut_state();
                labelled_state.remove_all();
                vir::Stmt::Inhale(lhs.clone()).apply_on_state(labelled_state, bctxt.predicates());
                if let vir::Expr::PredicateAccessPredicate(ref name, ref args, frac) = lhs {
                    labelled_state.insert_acc(args[0].clone(), *frac);
                }
                labelled_state.replace_places(|place| place.old(&label));
                self.bctxt_at_label.insert(label.to_string(), labelled_bctxt);
            }

            _ => {} // Nothing
        }

        let mut stmts: Vec<vir::Stmt> = vec![];

        // Insert "unfolding in" inside old expressions
        stmt = self.rewrite_stmt_with_unfoldings_in_old(stmt, &bctxt);

        // 1. Obtain "preferred" permissions (i.e. due to "weak obtain" statements)
        let preferred_perms: Vec<_> = stmt
            .get_preferred_permissions(bctxt.predicates())
            .into_iter()
            .filter(|p| p.is_curr())
            .collect();

        let obtainable_preferred_perms: Vec<_> = preferred_perms.into_iter()
            .filter(|p| !(p.is_curr() && bctxt.state().is_prefix_of_some_moved(&p.get_place())))
            .filter(|p| !(p.is_curr() && bctxt.state().moved().iter().any(|mp| p.get_place().has_prefix(mp))))
            .collect();

        if !obtainable_preferred_perms.is_empty() {
            stmts.extend(
                bctxt
                    .obtain_permissions(obtainable_preferred_perms)
                    .iter()
                    .map(|a| a.to_stmt())
            );

            if self.check_foldunfold_state {
                stmts.push(vir::Stmt::comment("Assert content of fold/unfold state"));
                stmts.push(vir::Stmt::Assert(bctxt.state().as_vir_expr(), vir::Position::new(0, 0, "check fold/unfold state".to_string())));
            }
        }

        // 2. Obtain required *curr* permissions. *old* requirements will be handled at step 4.
        let perms: Vec<_> = stmt
            .get_required_permissions(bctxt.predicates())
            .into_iter()
            .filter(|p| p.is_curr())
            .collect();
        debug!("required permissions: {{\n{}\n}}", perms.iter().map(|x| format!("  {:?}", x)).collect::<Vec<_>>().join(",\n"));

        if !perms.is_empty() {
            stmts.extend(
                bctxt
                    .obtain_permissions(perms)
                    .iter()
                    .map(|a| a.to_stmt())
            );

            if self.check_foldunfold_state && !is_last_before_return {
                stmts.push(vir::Stmt::comment("Assert content of fold/unfold state"));
                stmts.push(vir::Stmt::Assert(bctxt.state().as_vir_expr(), vir::Position::new(0, 0, "check fold/unfold state".to_string())));
            }
        }

        // 3. Replace special statements
        stmt = match stmt {
            vir::Stmt::ExpireBorrowsIf(ref guard, ref then_branch, ref else_branch) => {
                // Do the special join for restoring permissions of expiring loans
                let mut then_bctxt = bctxt.clone();
                let mut else_bctxt = bctxt.clone();
                let mut new_then_stmts = vec![];
                let mut new_else_stmts = vec![];
                for then_stmt in then_branch.iter() {
                    new_then_stmts.extend(
                        self.replace_stmt(then_stmt, false, &mut then_bctxt)
                    );
                }
                for else_stmt in else_branch.iter() {
                    new_else_stmts.extend(
                        self.replace_stmt(else_stmt, false, &mut else_bctxt)
                    );
                }
                let (then_actions, else_actions) = then_bctxt.join(else_bctxt);
                *bctxt = then_bctxt;
                new_then_stmts.extend(
                    then_actions.iter().map(|a| a.to_stmt())
                );
                new_else_stmts.extend(
                    else_actions.iter().map(|a| a.to_stmt())
                );
                // Restore dropped permissions
                for action in then_actions.iter() {
                    if let Action::Drop(ref perm) = action {
                        bctxt.mut_state().insert_perm(perm.clone());
                        bctxt.mut_state().insert_dropped(perm.clone());
                    }
                }
                for action in else_actions.iter() {
                    if let Action::Drop(ref perm) = action {
                        bctxt.mut_state().insert_perm(perm.clone());
                        bctxt.mut_state().insert_dropped(perm.clone());
                    }
                }
                vir::Stmt::ExpireBorrowsIf(guard.clone(), new_then_stmts, new_else_stmts)
            }

            vir::Stmt::PackageMagicWand(vir::Expr::MagicWand(box ref lhs, box ref rhs), ref old_package_stmts, ref position) => {
                let mut package_bctxt = bctxt.clone();
                let mut package_stmts = vec![];
                for stmt in old_package_stmts {
                    package_stmts.extend(
                        self.replace_stmt(stmt, false, &mut package_bctxt)
                    );
                }
                vir::Stmt::package_magic_wand(lhs.clone(), rhs.clone(), package_stmts, position.clone())
            }

            stmt => stmt,
        };

        // 4. Add "unfolding" expressions in statement. This handles *old* requirements.
        debug!("Add unfoldings in stmt {}", stmt);
        stmt = self.rewrite_stmt_with_unfoldings(stmt, &bctxt);

        // 5. Apply effect of statement on state
        bctxt.apply_stmt(&stmt);
        stmts.push(stmt);

        // Delete lhs state
        self.bctxt_at_label.remove("lhs");

        stmts
    }

    /// Inject some statements and replace a successor, mutating the branch context
    fn replace_successor(&mut self, succ: &vir::Successor, bctxt: &mut BranchCtxt<'p>) -> (Vec<vir::Stmt>, vir::Successor) {
        debug!("replace_successor: {}", succ);
        let exprs: Vec<&vir::Expr> = match succ {
            &vir::Successor::GotoSwitch(ref guarded_targets, _) => {
                guarded_targets.iter().map(|g| &g.0).collect()
            },
            &vir::Successor::GotoIf(ref expr, _, _) => vec![expr],
            _ => vec![]
        };

        let grouped_perms: HashMap<_, _> = exprs.iter().flat_map(
            |e| e.get_required_permissions(bctxt.predicates())
        ).group_by_label();

        let mut stmts: Vec<vir::Stmt> = vec![];

        /*
        if !grouped_perms.is_empty() && self.dump_debug_info {
            stmts.push(vir::Stmt::comment(format!("[foldunfold] Access permissions: {{{}}}", bctxt.state().display_acc())));
            stmts.push(vir::Stmt::comment(format!("[foldunfold] Predicate permissions: {{{}}}", bctxt.state().display_pred())));
        }
        */

        let mut some_perms_required = false;
        for (label, perms) in grouped_perms.into_iter() {
            debug!("Obtain at label {:?} permissions {:?}", label, perms);
            if !perms.is_empty() {
                some_perms_required = true;
                let mut opt_old_bctxt = label.map(
                    |label_name| self.bctxt_at_label.get(&label_name).unwrap().clone()
                );
                let label_bctxt = opt_old_bctxt.as_mut().unwrap_or(bctxt);
                stmts.extend(
                    label_bctxt
                        .obtain_permissions(perms)
                        .iter()
                        .map(|a| a.to_stmt())
                        .collect::<Vec<_>>()
                );
            }
        }

        if some_perms_required && self.check_foldunfold_state {
            stmts.push(vir::Stmt::comment("Assert content of fold/unfold state"));
            stmts.push(vir::Stmt::Assert(bctxt.state().as_vir_expr(), vir::Position::new(0, 0, "check fold/unfold state".to_string())));
        }

        // Add "fold/unfolding in" expressions in successor
        let repl_expr = |expr: &vir::Expr| -> vir::Expr {
            self.replace_expr(expr, bctxt)
        };

        let new_succ= match succ {
            vir::Successor::Undefined => vir::Successor::Undefined,
            vir::Successor::Return => vir::Successor::Return,
            vir::Successor::Goto(target) => vir::Successor::Goto(*target),
            vir::Successor::GotoSwitch(guarded_targets, default_target) => {
                vir::Successor::GotoSwitch(
                    guarded_targets
                        .iter()
                        .map(|(cond, targ)| (repl_expr(cond), targ.clone()))
                        .collect::<Vec<_>>(),
                    *default_target
                )
            },
            vir::Successor::GotoIf(condition, then_target, else_target) => {
                vir::Successor::GotoIf(repl_expr(condition), *then_target, *else_target)
            },
        };

        (stmts, new_succ)
    }

    /// Prepend some statements to an existing join point, returning the merged branch context.
    fn prepend_join(&mut self, bcs: Vec<&BranchCtxt<'p>>) -> (Vec<Vec<vir::Stmt>>, BranchCtxt<'p>) {
        trace!("[enter] prepend_join(..{})", &bcs.len());
        assert!(bcs.len() > 0);
        if bcs.len() == 1 {
            (vec![vec![]], bcs[0].clone())
        } else {
            // Define two subgroups
            let mid = bcs.len() / 2;
            let left_bcs = &bcs[..mid];
            let right_bcs = &bcs[mid..];

            // Join the subgroups
            let (left_stmts_vec, mut left_bc) = self.prepend_join(left_bcs.to_vec());
            let (right_stmts_vec, right_bc) = self.prepend_join(right_bcs.to_vec());

            // Join the recursive calls
            let (merge_actions_left, merge_actions_right) = left_bc.join(right_bc);
            let merge_bc = left_bc;

            let mut branch_stmts_vec: Vec<Vec<vir::Stmt>> = vec![];
            for left_stmts in left_stmts_vec {
                let mut branch_stmts = left_stmts.clone();
                branch_stmts.extend(merge_actions_left.iter().map(|a| a.to_stmt()).collect::<Vec<_>>());
                branch_stmts_vec.push(branch_stmts);
            }
            for right_stmts in right_stmts_vec {
                let mut branch_stmts = right_stmts.clone();
                branch_stmts.extend(merge_actions_right.iter().map(|a| a.to_stmt()).collect::<Vec<_>>());
                branch_stmts_vec.push(branch_stmts);
            }

            trace!("[exit] prepend_join(..{}): {:?}", &bcs.len(), &branch_stmts_vec);
            (branch_stmts_vec, merge_bc)
        }
    }
}

struct ExprReplacer<'b, 'a: 'b> {
    curr_bctxt: BranchCtxt<'a>,
    bctxt_at_label: &'b HashMap<String, BranchCtxt<'a>>,
    lhs_bctxt: Option<BranchCtxt<'a>>,
}

impl<'b, 'a: 'b> ExprReplacer<'b, 'a>{
    pub fn new(curr_bctxt: BranchCtxt<'a>, bctxt_at_label: &'b HashMap<String, BranchCtxt<'a>>) -> Self {
        ExprReplacer {
            curr_bctxt,
            bctxt_at_label,
            lhs_bctxt: None
        }
    }
}

impl<'b, 'a: 'b> ExprFolder for ExprReplacer<'b, 'a> {
    fn fold_magic_wand(&mut self, lhs: Box<vir::Expr>, rhs: Box<vir::Expr>) -> vir::Expr {
        // Compute lhs state
        let mut lhs_bctxt = self.curr_bctxt.clone();
        let lhs_state = lhs_bctxt.mut_state();
        lhs_state.remove_all();
        vir::Stmt::Inhale(*lhs.clone()).apply_on_state(lhs_state, self.curr_bctxt.predicates());
        if let box vir::Expr::PredicateAccessPredicate(ref name, ref args, frac) = lhs {
            lhs_state.insert_acc(args[0].clone(), frac);
        }
        lhs_state.replace_places(|place| place.old("lhs"));

        // Compute rhs state
        let mut rhs_bctxt = self.curr_bctxt.clone();
        let rhs_state = rhs_bctxt.mut_state();
        rhs_state.remove_all();
        rhs_state.insert_all_perms(
            rhs.get_permissions(self.curr_bctxt.predicates())
                .into_iter()
                .filter(|p| p.is_pred())
        );

        // Store states
        let mut tmp_curr_bctxt = rhs_bctxt;
        std::mem::swap(&mut self.curr_bctxt, &mut tmp_curr_bctxt);
        self.lhs_bctxt = Some(lhs_bctxt);

        // Rewrite rhs
        let new_rhs = self.fold_boxed(rhs);

        // Restore states
        self.lhs_bctxt = None;
        std::mem::swap(&mut self.curr_bctxt, &mut tmp_curr_bctxt);

        // Rewrite lhs and build magic wand
        vir::Expr::MagicWand(self.fold_boxed(lhs), new_rhs)
    }

    fn fold_labelled_old(&mut self, label: String, expr: Box<vir::Expr>) -> vir::Expr {
        debug!("fold_labelled_old {}: {}", label, expr);

        let mut tmp_curr_bctxt = if label == "lhs" && self.lhs_bctxt.is_some() {
            self.lhs_bctxt.as_ref().unwrap().clone()
        } else {
            self.bctxt_at_label.get(&label).unwrap().clone()
        };

        // Replace old[label] with curr
        tmp_curr_bctxt.mut_state().replace_places(
            |place| place.map_labels(
                |opt_label| {
                    if opt_label == label.clone() {
                        None
                    } else {
                        Some(opt_label)
                    }
                }
            )
        );

        // Store states
        std::mem::swap(&mut self.curr_bctxt, &mut tmp_curr_bctxt);

        // Rewrite inner expression
        let inner_expr = self.fold_boxed(expr);

        // Restore states
        std::mem::swap(&mut self.curr_bctxt, &mut tmp_curr_bctxt);

        // Rebuild expression
        vir::Expr::LabelledOld(label, inner_expr)
    }

    fn fold(&mut self, expr: vir::Expr) -> vir::Expr {
        debug!("fold {}", expr);

        let perms: Vec<_> = expr
            .get_required_permissions(self.curr_bctxt.predicates())
            .into_iter()
            .collect();

        // Add appropriate unfolding around this old expression
        self.curr_bctxt
            .obtain_permissions(perms)
            .into_iter()
            .rev()
            .fold(
                expr,
                |expr, action| action.to_expr(expr)
            )
    }
}
