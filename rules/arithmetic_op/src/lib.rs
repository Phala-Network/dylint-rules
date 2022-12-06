#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;

use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
    
dylint_linting::declare_late_lint! {
    /// **What it does:**
    ///
    /// This is a lint rule that warns against directly using arithmetic operators in code.
    ///
    /// **Why is this bad?**
    ///
    /// Using arithmetic operators can cause arithmetic overflows, which can be a bad situation
    /// in a blockchain infrastructure or smart contract.
    /// `ink!` framework forces overflow checks to be enabled in build scripts, but the pruntime
    /// does not.
    /// Regardless of whether the framework enables checks, we should always be careful when doing
    /// arithmetic calculations.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where a warning is issued
    /// let amount = price * items;
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// let amount = price.checked_mul(items);
    /// ```
    /// 
    /// Instead of:
    /// ```
    /// // example code where a warning is issued
    /// let released = original_stake - slashed;
    /// ```
    /// Prefer:
    /// ```rust
    /// // example code that does not raise a warning
    /// let released = original_stake.saturating_sub(slashed);
    /// ```
    pub ARITHMETIC_OP,
    Warn,
    "Lint against directly using arithmetic operators in code"
}

impl<'tcx> LateLintPass<'tcx> for ArithmeticOp {
    fn check_expr(&mut self, ctx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        use rustc_hir::BinOpKind::*;
        match expr.kind {
            ExprKind::AssignOp(op, a, b) | ExprKind::Binary(op, a, b) => {
                match op.node {
                    Add | Sub | Mul | Div => {
                        if let (ExprKind::Lit(_), _) | (_, ExprKind::Lit(_)) = (&a.kind, &b.kind) {
                            return;
                        }
                        span_lint_and_help(
                            ctx,
                            ARITHMETIC_OP,
                            expr.span,
                            "directly using the arithmetic operator",
                            None,
                            "consider using the saturating or checked version calls",
                        );
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
