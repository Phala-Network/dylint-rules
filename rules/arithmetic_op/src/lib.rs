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
    /// Instead of using arithmetic operators directly, it is recommended to use the `checked_*`
    /// methods, such as `checked_mul()` or `checked_add()`, to perform arithmetic operations. These
    /// methods will return None if an overflow occurs, allowing you to handle the error safely.
    /// For example, instead of using the `*` operator to multiply two numbers, you can use
    /// `checked_mul()` to perform the same operation in a safe manner:
    ///
    /// ```rust
    /// // Bad: directly using the `*` operator
    /// let amount = price * items;

    /// // Good: using the `checked_mul()` method
    /// let amount = price.checked_mul(items);
    /// ```
    pub ARITHMETIC_OP,
    Warn,
    "Lint against directly using arithmetic operators in code"
}

impl<'tcx> LateLintPass<'tcx> for ArithmeticOp {
    fn check_expr(&mut self, ctx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        use rustc_hir::BinOpKind::*;
        match expr.kind {
            ExprKind::AssignOp(op, a, b) | ExprKind::Binary(op, a, b) => match op.node {
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
            },
            _ => (),
        }
    }
}
