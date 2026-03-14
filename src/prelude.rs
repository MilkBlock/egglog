//! This module makes it easier to use `egglog` from Rust.
//! It is intended to be imported fully.
//! ```
//! use egglog::prelude::*;
//! ```
//! See also [`rule`], [`rust_rule`], [`query`], [`BaseSort`],
//! and [`ContainerSort`].

use crate::*;
use std::any::{Any, TypeId};

// Re-exports in `prelude` for convenience.
pub use egglog::ast::{Action, Fact, Facts, GenericActions, RustSpan, Span};
pub use egglog::sort::{BigIntSort, BigRatSort, BoolSort, F64Sort, I64Sort, StringSort, UnitSort};
pub use egglog::{CommandMacro, CommandMacroRegistry};
pub use egglog::{EGraph, span};
pub use egglog::{action, actions, datatype, expr, fact, facts, sort, vars};

/// Trait for types that can be converted to/from Literal for use in validated primitives.
/// This enables automatic validator generation for literal primitives.
pub trait LiteralConvertible: Sized {
    fn to_literal(self) -> egglog_ast::generic_ast::Literal;
    fn from_literal(lit: &egglog_ast::generic_ast::Literal) -> Option<Self>;
}

impl LiteralConvertible for i64 {
    fn to_literal(self) -> egglog_ast::generic_ast::Literal {
        egglog_ast::generic_ast::Literal::Int(self)
    }
    fn from_literal(lit: &egglog_ast::generic_ast::Literal) -> Option<Self> {
        match lit {
            egglog_ast::generic_ast::Literal::Int(i) => Some(*i),
            _ => None,
        }
    }
}

impl LiteralConvertible for bool {
    fn to_literal(self) -> egglog_ast::generic_ast::Literal {
        egglog_ast::generic_ast::Literal::Bool(self)
    }
    fn from_literal(lit: &egglog_ast::generic_ast::Literal) -> Option<Self> {
        match lit {
            egglog_ast::generic_ast::Literal::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl LiteralConvertible for ordered_float::OrderedFloat<f64> {
    fn to_literal(self) -> egglog_ast::generic_ast::Literal {
        egglog_ast::generic_ast::Literal::Float(self)
    }
    fn from_literal(lit: &egglog_ast::generic_ast::Literal) -> Option<Self> {
        match lit {
            egglog_ast::generic_ast::Literal::Float(f) => Some(*f),
            _ => None,
        }
    }
}

impl LiteralConvertible for egglog::sort::F {
    fn to_literal(self) -> egglog_ast::generic_ast::Literal {
        egglog_ast::generic_ast::Literal::Float(self.0)
    }
    fn from_literal(lit: &egglog_ast::generic_ast::Literal) -> Option<Self> {
        match lit {
            egglog_ast::generic_ast::Literal::Float(f) => Some(egglog::sort::F::from(*f)),
            _ => None,
        }
    }
}

impl LiteralConvertible for egglog::sort::S {
    fn to_literal(self) -> egglog_ast::generic_ast::Literal {
        egglog_ast::generic_ast::Literal::String(self.0)
    }
    fn from_literal(lit: &egglog_ast::generic_ast::Literal) -> Option<Self> {
        match lit {
            egglog_ast::generic_ast::Literal::String(s) => Some(egglog::sort::S::new(s.clone())),
            _ => None,
        }
    }
}

impl LiteralConvertible for () {
    fn to_literal(self) -> egglog_ast::generic_ast::Literal {
        egglog_ast::generic_ast::Literal::Unit
    }
    fn from_literal(lit: &egglog_ast::generic_ast::Literal) -> Option<Self> {
        match lit {
            egglog_ast::generic_ast::Literal::Unit => Some(()),
            _ => None,
        }
    }
}

pub mod exprs {
    use super::*;

    /// Creates a variable expression.
    pub fn var(name: &str) -> Expr {
        Expr::Var(span!(), name.to_owned())
    }

    /// Creates an integer literal expression.
    pub fn int(value: i64) -> Expr {
        Expr::Lit(span!(), Literal::Int(value))
    }

    /// Creates a float literal expression.
    pub fn float(value: f64) -> Expr {
        Expr::Lit(span!(), Literal::Float(value.into()))
    }

    /// Creates a string literal expression.
    pub fn string(value: &str) -> Expr {
        Expr::Lit(span!(), Literal::String(value.to_owned()))
    }

    /// Creates a unit literal expression.
    pub fn unit() -> Expr {
        Expr::Lit(span!(), Literal::Unit)
    }

    /// Creates a boolean literal expression.
    pub fn bool(value: bool) -> Expr {
        Expr::Lit(span!(), Literal::Bool(value))
    }

    /// Creates a function call expression.
    pub fn call(f: &str, xs: Vec<Expr>) -> Expr {
        Expr::Call(span!(), f.to_owned(), xs)
    }
}

/// Create a new ruleset.
pub fn add_ruleset(egraph: &mut EGraph, ruleset: &str) -> Result<Vec<CommandOutput>, Error> {
    egraph.run_program(vec![Command::AddRuleset(span!(), ruleset.to_owned())])
}

/// Run one iteration of a ruleset.
pub fn run_ruleset(egraph: &mut EGraph, ruleset: &str) -> Result<Vec<CommandOutput>, Error> {
    egraph.run_program(vec![Command::RunSchedule(Schedule::Run(
        span!(),
        RunConfig {
            ruleset: ruleset.to_owned(),
            until: None,
        },
    ))])
}

#[macro_export]
macro_rules! sort {
    (BigInt) => {
        BigIntSort.to_arcsort()
    };
    (BigRat) => {
        BigRatSort.to_arcsort()
    };
    (bool) => {
        BoolSort.to_arcsort()
    };
    (f64) => {
        F64Sort.to_arcsort()
    };
    (i64) => {
        I64Sort.to_arcsort()
    };
    (String) => {
        StringSort.to_arcsort()
    };
    (Unit) => {
        UnitSort.to_arcsort()
    };
    ($t:expr) => {
        $t
    };
}

#[macro_export]
macro_rules! vars {
    [$($x:ident : $t:tt),* $(,)?] => {
        &[$((stringify!($x), sort!($t))),*]
    };
}

#[macro_export]
macro_rules! expr {
    ((unquote $unquoted:expr)) => { $unquoted };
    (($func:tt $($arg:tt)*)) => { exprs::call(stringify!($func), vec![$(expr!($arg)),*]) };
    ($value:literal) => { exprs::int($value) };
    ($quoted:tt) => { exprs::var(stringify!($quoted)) };
}

#[macro_export]
macro_rules! fact {
    ((= $($arg:tt)*)) => { Fact::Eq(span!(), $(expr!($arg)),*) };
    ($a:tt) => { Fact::Fact(expr!($a)) };
}

#[macro_export]
macro_rules! facts {
    ($($tree:tt)*) => { Facts(vec![$(fact!($tree)),*]) };
}

#[macro_export]
macro_rules! action {
    ((let $name:ident $value:tt)) => {
        Action::Let(span!(), String::from(stringify!($name)), expr!($value))
    };
    ((set ($f:ident $($x:tt)*) $value:tt)) => {
        Action::Set(span!(), String::from(stringify!($f)), vec![$(expr!($x)),*], expr!($value))
    };
    ((delete ($f:ident $($x:tt)*))) => {
        Action::Change(span!(), Change::Delete, String::from(stringify!($f)), vec![$(expr!($x)),*])
    };
    ((subsume ($f:ident $($x:tt)*))) => {
        Action::Change(span!(), Change::Subsume, String::from(stringify!($f)), vec![$(expr!($x)),*])
    };
    ((union $x:tt $y:tt)) => {
        Action::Union(span!(), expr!($x), expr!($y))
    };
    ((panic $message:literal)) => {
        Action::Panic(span!(), $message.to_owned())
    };
    ($x:tt) => {
        Action::Expr(span!(), expr!($x))
    };
}

#[macro_export]
macro_rules! actions {
    ($($tree:tt)*) => { GenericActions(vec![$(action!($tree)),*]) };
}

/// Add a rule to the e-graph whose right-hand side is made up of actions.
/// ```
/// use egglog::prelude::*;
///
/// let mut egraph = EGraph::default();
/// egraph.parse_and_run_program(
///     None,
///     "
/// (function fib (i64) i64 :no-merge)
/// (set (fib 0) 0)
/// (set (fib 1) 1)
/// (rule (
///     (= f0 (fib x))
///     (= f1 (fib (+ x 1)))
/// ) (
///     (set (fib (+ x 2)) (+ f0 f1))
/// ))
/// (run 10)
///     ",
/// )?;
///
/// let big_number = 20;
///
/// // check that `(fib 20)` is not in the e-graph
/// let results = query(
///     &mut egraph,
///     vars![f: i64],
///     facts![(= (fib (unquote exprs::int(big_number))) f)],
/// )?;
///
/// assert!(results.iter().next().is_none());
///
/// let ruleset = "custom_ruleset";
/// add_ruleset(&mut egraph, ruleset)?;
///
/// // add the rule from `build_test_database` to the egraph
/// rule(
///     &mut egraph,
///     ruleset,
///     facts![
///         (= f0 (fib x))
///         (= f1 (fib (+ x 1)))
///     ],
///     actions![
///         (set (fib (+ x 2)) (+ f0 f1))
///     ],
/// )?;
///
/// // run that rule 10 times
/// for _ in 0..10 {
///     run_ruleset(&mut egraph, ruleset)?;
/// }
///
/// // check that `(fib 20)` is now in the e-graph
/// let results = query(
///     &mut egraph,
///     vars![f: i64],
///     facts![(= (fib (unquote exprs::int(big_number))) f)],
/// )?;
///
/// let y = egraph.base_to_value::<i64>(6765);
/// let results: Vec<_> = results.iter().collect();
/// assert_eq!(results, [[y]]);
///
/// # Ok::<(), egglog::Error>(())
/// ```
pub fn rule(
    egraph: &mut EGraph,
    ruleset: &str,
    facts: Facts<String, String>,
    actions: Actions,
) -> Result<Vec<CommandOutput>, Error> {
    let rule = Rule {
        span: span!(),
        head: actions,
        body: facts.0,
        name: "".into(),
        ruleset: ruleset.into(),
    };

    egraph.run_program(vec![Command::Rule { rule }])
}

/// A wrapper around an `ExecutionState` for rules that are written in Rust.
/// See the [`rust_rule`] documentation for an example of how to use this.
pub struct RustRuleContext<'a, 'b, 'c> {
    exec_state: &'a mut ExecutionState<'b>,
    union_action: egglog_bridge::UnionAction,
    /// Name -> backend FunctionId (captured at rust-rule registration time).
    func_ids: &'c HashMap<String, egglog_bridge::FunctionId>,
    /// Table actions indexed by backend FunctionId (captured at rust-rule registration time).
    table_actions_by_id: &'c [Option<egglog_bridge::TableAction>],
    /// Per-match table actions, lazily populated from `table_actions_by_id` (for scratch reuse).
    table_actions_cache_by_id: Vec<(egglog_bridge::FunctionId, egglog_bridge::TableAction)>,
    /// When term encoding is enabled, functions/constructors are backed by a hidden "term table"
    /// and a separate "view table" (see proofs/term-encoding).
    ///
    /// For a function `f`, the view table is a function whose `:term-constructor` is `f`.
    /// This map is keyed by the surface `f` name and points to the view table action.
    ///
    /// Rust-side `lookup(f, ...)` must populate both tables; otherwise rules (which are
    /// instrumented to match on view tables) will observe zero matches.
    /// View table actions indexed by the term-constructor FunctionId.
    view_actions_by_id: &'c [Option<egglog_bridge::TableAction>],
    /// View proof functions: for each surface term constructor name `f`, this stores the
    /// `{f}ViewProof` table action (when proofs are enabled).
    /// View proof actions indexed by the term-constructor FunctionId.
    view_proof_actions_by_id: &'c [Option<egglog_bridge::TableAction>],
    /// Per-match view proof actions (for scratch reuse).
    view_proof_actions_cache_by_id: Vec<(egglog_bridge::FunctionId, egglog_bridge::TableAction)>,
    /// Term proof functions: for each eq sort name `S`, this stores the `{S}Proof` table action.
    term_proof_actions: &'c HashMap<String, egglog_bridge::TableAction>,
    term_proof_actions_cache: HashMap<String, egglog_bridge::TableAction>,
    /// UF functions: for each eq sort name `S`, this stores the `{S}UF` table action.
    uf_actions: &'c HashMap<String, egglog_bridge::TableAction>,
    /// UF proof functions: for each eq sort name `S`, this stores the `{S}UFProof` table action.
    uf_proof_actions: &'c HashMap<String, egglog_bridge::TableAction>,
    uf_proof_actions_cache: HashMap<String, egglog_bridge::TableAction>,
    /// Sort-to-AST constructors: for each sort name `S`, this stores the `Ast{S}` constructor action.
    to_ast_actions: &'c HashMap<String, egglog_bridge::TableAction>,
    /// Fiat constructor for the proof datatype (when proofs are enabled).
    fiat_action: &'c Option<egglog_bridge::TableAction>,
    /// Rule constructor for the proof datatype (when proofs are enabled).
    rule_action: &'c Option<egglog_bridge::TableAction>,
    /// Non-empty proof list constructor `PCons` (when proofs are enabled).
    pcons_action: &'c Option<egglog_bridge::TableAction>,
    /// Empty proof list constructor `PNil` (when proofs are enabled).
    pnil_action: &'c Option<egglog_bridge::TableAction>,
    /// Name of the proof-list empty constructor `PNil` (when proofs are enabled).
    ///
    /// Needed because in term-encoding mode we must populate the corresponding view table;
    /// `TableAction::lookup` alone only touches the hidden term table.
    pnil_name: &'c Option<String>,
    /// Name of the proof-list cons constructor `PCons` (when proofs are enabled).
    ///
    /// Needed for the same reason as `pnil_name`.
    pcons_name: &'c Option<String>,
    /// The user-provided rule name (for tagging proof terms).
    rule_name: &'c str,
    /// Output sort name for surface constructors (e.g. `Const` -> `Expr`).
    term_output_sorts_by_id: &'c [Option<String>],
    panic_id: ExternalFunctionId,
}

impl RustRuleContext<'_, '_, '_> {
    /// Convert from an egglog value to a Rust type.
    pub fn value_to_base<T: BaseValue>(&self, x: Value) -> T {
        self.exec_state.base_values().unwrap::<T>(x)
    }

    /// Convert from an egglog value to reference of Rust container type.
    ///
    /// See [`EGraph::value_to_container`].
    pub fn value_to_container<T: ContainerValue>(
        &mut self,
        x: Value,
    ) -> Option<impl Deref<Target = T>> {
        self.exec_state.container_values().get_val::<T>(x)
    }

    /// Convert from a Rust type to an egglog value.
    pub fn base_to_value<T: BaseValue>(&self, x: T) -> Value {
        self.exec_state.base_values().get::<T>(x)
    }

    /// Convert from a Rust container type to an egglog value.
    pub fn container_to_value<T: ContainerValue>(&mut self, x: T) -> Value {
        self.exec_state
            .container_values()
            .register_val::<T>(x, self.exec_state)
    }

    fn func_id(&self, table: &str) -> egglog_bridge::FunctionId {
        *self
            .func_ids
            .get(table)
            .unwrap_or_else(|| panic!("missing function id for table: {table}"))
    }

    fn get_table_action_by_id(&self, table: egglog_bridge::FunctionId) -> &egglog_bridge::TableAction {
        self.table_actions_by_id
            .get(table.rep() as usize)
            .and_then(|x| x.as_ref())
            .unwrap_or_else(|| panic!("missing table action for id: {:?}", table))
    }

    /// Do a table lookup. This is potentially a mutable operation!
    /// For more information, see `egglog_bridge::TableAction::lookup`.
    pub fn lookup(&mut self, table: &str, key: &[Value]) -> Option<Value> {
        let table_id = self.func_id(table);
        self.lookup_id(table_id, key)
    }

    /// Do a table lookup by cached backend FunctionId.
    pub fn lookup_id(&mut self, table: egglog_bridge::FunctionId, key: &[Value]) -> Option<Value> {
        let table_actions_by_id = self.table_actions_by_id;
        let action = table_actions_by_id
            .get(table.rep() as usize)
            .and_then(|x| x.as_ref())
            .unwrap_or_else(|| panic!("missing table action for id: {:?}", table));
        let out = action.lookup(self.exec_state, key);

        // In term-encoding mode, view tables store canonicalized e-nodes and are used for matching.
        // `lookup` on the term table must also ensure:
        // - the corresponding view entry exists, and
        // - (in proofs mode) the corresponding view proof / term proof entries exist,
        //   since instrumented rules query them as part of their match.
        if let Some(out) = out {
            let view_actions_by_id = self.view_actions_by_id;
            if let Some(view) = view_actions_by_id
                .get(table.rep() as usize)
                .and_then(|x| x.as_ref())
            {
                let mut view_key = Vec::with_capacity(key.len() + 1);
                view_key.extend_from_slice(key);
                view_key.push(out);
                let _ = view.lookup(self.exec_state, &view_key);

                if let Some(proof) = self.mk_fiat_term_proof_by_id(table, out) {
                    let view_proof_actions_by_id = self.view_proof_actions_by_id;
                    let has_view_proof = view_proof_actions_by_id
                        .get(table.rep() as usize)
                        .and_then(|x| x.as_ref())
                        .is_some();
                    if has_view_proof {
                        let row = view_key.iter().copied().chain(std::iter::once(proof));
                        let view_proof_action = if let Some((_, action)) = self
                            .view_proof_actions_cache_by_id
                            .iter_mut()
                            .find(|(id, _)| *id == table)
                        {
                            action
                        } else {
                            let action = view_proof_actions_by_id
                                .get(table.rep() as usize)
                                .and_then(|x| x.as_ref())
                                .unwrap_or_else(|| {
                                    panic!("missing view proof action for table id: {:?}", table)
                                })
                                .clone();
                            self.view_proof_actions_cache_by_id.push((table, action));
                            &mut self
                                .view_proof_actions_cache_by_id
                                .last_mut()
                                .expect("just pushed")
                                .1
                        };
                        view_proof_action.insert(self.exec_state, row);
                    }
                }
            }
            Some(out)
        } else {
            None
        }
    }

    fn mk_fiat_term_proof_by_id(
        &mut self,
        table: egglog_bridge::FunctionId,
        out: Value,
    ) -> Option<Value> {
        let sort_name = self
            .term_output_sorts_by_id
            .get(table.rep() as usize)?
            .as_deref()?;
        let term_proof = self.term_proof_actions.get(sort_name)?;
        let to_ast = self.to_ast_actions.get(sort_name)?;
        let fiat = self.fiat_action.as_ref()?;

        // Reuse existing term proof if present.
        if let Some(existing) = term_proof.lookup(self.exec_state, &[out]) {
            return Some(existing);
        }

        let ast = to_ast.lookup(self.exec_state, &[out])?;
        let proof = if let (Some(rule), Some(pnil)) =
            (self.rule_action.as_ref(), self.pnil_action.as_ref())
        {
            let name_val = self
                .exec_state
                .base_values()
                .get::<crate::sort::S>(self.rule_name.to_owned().into());
            let empty = pnil.lookup(self.exec_state, &[])?;
            rule.lookup(self.exec_state, &[name_val, empty, ast, ast])?
        } else {
            fiat.lookup(self.exec_state, &[ast, ast])?
        };
        let term_proof_action = if let Some(action) = self.term_proof_actions_cache.get_mut(sort_name)
        {
            action
        } else {
            let action = term_proof.clone();
            self.term_proof_actions_cache
                .insert(sort_name.to_owned(), action);
            self.term_proof_actions_cache
                .get_mut(sort_name)
                .expect("inserted action should be present")
        };
        term_proof_action.insert(self.exec_state, [out, proof].into_iter());
        Some(proof)
    }

    /// Union two values in the e-graph.
    /// For more information, see `egglog_bridge::UnionAction::union`.
    pub fn union(&mut self, x: Value, y: Value) {
        self.union_action.union(self.exec_state, x, y)
    }

    /// Union two values, and (when proofs are enabled) also record a per-sort `{Sort}UFProof`
    /// edge tagged with this rust rule's (freshened) name.
    ///
    /// This must be called *before* the union is merged/applied; otherwise the key can collapse
    /// to the canonical representative and the proof edge becomes unusable for `prove`.
    pub fn union_typed(&mut self, sort_name: &str, x: Value, y: Value, premise_proofs: &[Value]) {
        let resolved_sort_name_owned: String = if self.uf_actions.contains_key(sort_name) {
            sort_name.to_owned()
        } else if let Some(func_id) = self.func_ids.get(sort_name) {
            self.term_output_sorts_by_id
                .get(func_id.rep() as usize)
                .and_then(|x| x.as_ref())
                .cloned()
                .unwrap_or_else(|| sort_name.to_owned())
        } else {
            sort_name.to_owned()
        };
        let resolved_sort_name: &str = &resolved_sort_name_owned;
        log::debug!(
            "RustRuleContext::union_typed sort={} (resolved={}) x={:?} y={:?} rule_name={}",
            sort_name,
            resolved_sort_name,
            x,
            y,
            self.rule_name
        );
        let uf = self.uf_actions.get(resolved_sort_name);
        let has_uf_proof = self.uf_proof_actions.contains_key(resolved_sort_name);
        let to_ast = self.to_ast_actions.get(resolved_sort_name);
        let rule = self.rule_action.as_ref();
        let pnil = self.pnil_action.as_ref();
        if uf.is_none() {
            log::debug!(
                "RustRuleContext::union_typed missing UF action for sort={resolved_sort_name}"
            );
        }
        if !has_uf_proof {
            log::debug!(
                "RustRuleContext::union_typed missing UFProof action for sort={resolved_sort_name}"
            );
        }
        if to_ast.is_none() {
            log::debug!(
                "RustRuleContext::union_typed missing to_ast action for sort={resolved_sort_name}"
            );
        }
        if rule.is_none() {
            log::debug!("RustRuleContext::union_typed missing Rule constructor action");
        }
        if pnil.is_none() {
            log::debug!("RustRuleContext::union_typed missing PNil constructor action");
        }
        if let (Some(uf), true, Some(to_ast), Some(rule), Some(_pnil)) =
            (uf, has_uf_proof, to_ast, rule, pnil)
        {
            let (smaller, larger) = if x <= y { (x, y) } else { (y, x) };
            if let (Some(ast_larger), Some(ast_smaller)) = (
                to_ast.lookup(self.exec_state, &[larger]),
                to_ast.lookup(self.exec_state, &[smaller]),
            ) {
                let name_val = self
                    .exec_state
                    .base_values()
                    .get::<crate::sort::S>(self.rule_name.to_owned().into());
                let pnil_name = self.pnil_name.as_ref().map(|s| s.as_str()).unwrap_or_default();
                let pcons_name = self.pcons_name.as_ref().map(|s| s.as_str()).unwrap_or_default();
                let debug_proof_list = std::env::var_os("EGGLOG_DEBUG_PROOF_LIST").is_some()
                    && self.rule_name.contains("union_chain");
                let mut proof_list = match (!pnil_name.is_empty())
                    .then(|| self.lookup(pnil_name, &[]))
                    .flatten()
                {
                    Some(v) => v,
                    None => {
                        self.union_action.union(self.exec_state, x, y);
                        return;
                    }
                };
                if debug_proof_list {
                    eprintln!(
                        "DEBUG proof_list init: pnil_name={} pcons_name={} proof_list={:?} premise_proofs_len={}",
                        pnil_name,
                        pcons_name,
                        proof_list,
                        premise_proofs.len()
                    );
                }
                if !premise_proofs.is_empty() {
                    if pcons_name.is_empty() {
                        panic!(
                            "RustRuleContext::union_typed cannot attach premise proofs because PCons name is missing (rule_name={}, sort={})",
                            self.rule_name, resolved_sort_name
                        )
                    }
                    for prf in premise_proofs.iter().rev().copied() {
                        if debug_proof_list {
                            eprintln!(
                                "DEBUG proof_list cons: head={:?} tail={:?}",
                                prf, proof_list
                            );
                        }
                        proof_list = self
                            .lookup(pcons_name, &[prf, proof_list])
                            .unwrap_or_else(|| {
                                panic!(
                                    "RustRuleContext::union_typed failed to construct ProofList via PCons (rule_name={}, sort={})",
                                    self.rule_name,
                                    resolved_sort_name
                                )
                            });
                        if debug_proof_list {
                            eprintln!("DEBUG proof_list after: {:?}", proof_list);
                        }
                    }
                }
                if let Some(proof) = rule.lookup(
                    self.exec_state,
                    &[name_val, proof_list, ast_larger, ast_smaller],
                ) {
                    let _ = uf.lookup(self.exec_state, &[larger, smaller]);
                    let uf_proof_action = if let Some(action) =
                        self.uf_proof_actions_cache.get_mut(resolved_sort_name)
                    {
                        action
                    } else {
                        let action = self
                            .uf_proof_actions
                            .get(resolved_sort_name)
                            .unwrap_or_else(|| {
                                panic!(
                                    "missing UFProof action for sort: {resolved_sort_name}"
                                )
                            })
                            .clone();
                        self.uf_proof_actions_cache
                            .insert(resolved_sort_name.to_owned(), action);
                        self.uf_proof_actions_cache
                            .get_mut(resolved_sort_name)
                            .expect("inserted action should be present")
                    };
                    uf_proof_action.insert(
                        self.exec_state,
                        [larger, smaller, proof].into_iter(),
                    );
                    log::debug!(
                        "RustRuleContext::union_typed wrote UFProof({:?},{:?})",
                        larger,
                        smaller
                    );
                }
            }
        }

        self.union_action.union(self.exec_state, x, y)
    }

    /// Insert a row into a table.
    /// For more information, see `egglog_bridge::TableAction::insert`.
    pub fn insert(&mut self, table: &str, row: impl Iterator<Item = Value>) {
        let table_id = self.func_id(table);
        self.insert_id(table_id, row)
    }

    /// Insert a row into a table by cached backend FunctionId.
    pub fn insert_id(&mut self, table: egglog_bridge::FunctionId, row: impl Iterator<Item = Value>) {
        let action = if let Some((_, action)) = self
            .table_actions_cache_by_id
            .iter_mut()
            .find(|(id, _)| *id == table)
        {
            action
        } else {
            let action = self.get_table_action_by_id(table).clone();
            self.table_actions_cache_by_id.push((table, action));
            &mut self
                .table_actions_cache_by_id
                .last_mut()
                .expect("just pushed")
                .1
        };
        action.insert(self.exec_state, row)
    }

    /// Remove a row from a table.
    /// For more information, see `egglog_bridge::TableAction::remove`.
    pub fn remove(&mut self, table: &str, key: &[Value]) {
        let table_id = self.func_id(table);
        self.remove_id(table_id, key)
    }

    /// Remove a row from a table by cached backend FunctionId.
    pub fn remove_id(&mut self, table: egglog_bridge::FunctionId, key: &[Value]) {
        let table_actions_by_id = self.table_actions_by_id;
        let action = table_actions_by_id
            .get(table.rep() as usize)
            .and_then(|x| x.as_ref())
            .unwrap_or_else(|| panic!("missing table action for id: {:?}", table));
        action.remove(self.exec_state, key)
    }

    /// Subsume a row in a table.
    /// For more information, see `egglog_bridge::TableAction::subsume`.
    pub fn subsume(&mut self, table: &str, key: &[Value]) {
        let table_id = self.func_id(table);
        self.subsume_id(table_id, key)
    }

    /// Subsume a row in a table by cached backend FunctionId.
    pub fn subsume_id(&mut self, table: egglog_bridge::FunctionId, key: &[Value]) {
        let action = if let Some((_, action)) = self
            .table_actions_cache_by_id
            .iter_mut()
            .find(|(id, _)| *id == table)
        {
            action
        } else {
            let action = self.get_table_action_by_id(table).clone();
            self.table_actions_cache_by_id.push((table, action));
            &mut self
                .table_actions_cache_by_id
                .last_mut()
                .expect("just pushed")
                .1
        };
        action.subsume(self.exec_state, key.iter().copied())
    }

    /// Panic.
    /// You should also return `None` from your callback if you call
    /// this function, which this function hopefully makes easier by
    /// always returning `None` so that you can use `?`.
    pub fn panic(&mut self) -> Option<()> {
        self.exec_state.call_external_func(self.panic_id, &[]);
        None
    }
}

#[derive(Clone)]
struct RustRuleRhs<F: Fn(&mut RustRuleContext, &[Value]) -> Option<()>> {
    name: String,
    rule_name: String,
    inputs: Vec<ArcSort>,
    union_action: egglog_bridge::UnionAction,
    func_ids: Arc<HashMap<String, egglog_bridge::FunctionId>>,
    table_actions_by_id: Arc<Vec<Option<egglog_bridge::TableAction>>>,
    view_actions_by_id: Arc<Vec<Option<egglog_bridge::TableAction>>>,
    view_proof_actions_by_id: Arc<Vec<Option<egglog_bridge::TableAction>>>,
    term_proof_actions: Arc<HashMap<String, egglog_bridge::TableAction>>,
    uf_actions: Arc<HashMap<String, egglog_bridge::TableAction>>,
    uf_proof_actions: Arc<HashMap<String, egglog_bridge::TableAction>>,
    to_ast_actions: Arc<HashMap<String, egglog_bridge::TableAction>>,
    fiat_action: Option<egglog_bridge::TableAction>,
    rule_action: Option<egglog_bridge::TableAction>,
    pcons_action: Option<egglog_bridge::TableAction>,
    pnil_action: Option<egglog_bridge::TableAction>,
    pcons_name: Option<String>,
    pnil_name: Option<String>,
    term_output_sorts_by_id: Arc<Vec<Option<String>>>,
    panic_id: ExternalFunctionId,
    func: F,
}

impl<F: Fn(&mut RustRuleContext, &[Value]) -> Option<()>> Primitive for RustRuleRhs<F> {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_type_constraints(&self, span: &Span) -> Box<dyn TypeConstraint> {
        let sorts: Vec<_> = self
            .inputs
            .iter()
            .chain(once(&UnitSort.to_arcsort()))
            .cloned()
            .collect();
        SimpleTypeConstraint::new(self.name(), sorts, span.clone()).into_box()
    }

    fn apply(&self, exec_state: &mut ExecutionState, values: &[Value]) -> Option<Value> {
        let mut context = RustRuleContext {
            exec_state,
            union_action: self.union_action,
            func_ids: self.func_ids.as_ref(),
            table_actions_by_id: self.table_actions_by_id.as_slice(),
            table_actions_cache_by_id: Vec::new(),
            view_actions_by_id: self.view_actions_by_id.as_slice(),
            view_proof_actions_by_id: self.view_proof_actions_by_id.as_slice(),
            view_proof_actions_cache_by_id: Vec::new(),
            term_proof_actions: self.term_proof_actions.as_ref(),
            term_proof_actions_cache: Default::default(),
            uf_actions: self.uf_actions.as_ref(),
            uf_proof_actions: self.uf_proof_actions.as_ref(),
            uf_proof_actions_cache: Default::default(),
            to_ast_actions: self.to_ast_actions.as_ref(),
            fiat_action: &self.fiat_action,
            rule_action: &self.rule_action,
            pcons_action: &self.pcons_action,
            pnil_action: &self.pnil_action,
            pnil_name: &self.pnil_name,
            pcons_name: &self.pcons_name,
            rule_name: self.rule_name.as_str(),
            term_output_sorts_by_id: self.term_output_sorts_by_id.as_slice(),
            panic_id: self.panic_id,
        };
        (self.func)(&mut context, values)?;
        Some(exec_state.base_values().get(()))
    }
}

/// Add a rule to the e-graph whose right-hand side is a Rust callback.
/// ```
/// use egglog::prelude::*;
///
/// let mut egraph = EGraph::default();
/// egraph.parse_and_run_program(
///     None,
///     "
/// (function fib (i64) i64 :no-merge)
/// (set (fib 0) 0)
/// (set (fib 1) 1)
/// (rule (
///     (= f0 (fib x))
///     (= f1 (fib (+ x 1)))
/// ) (
///     (set (fib (+ x 2)) (+ f0 f1))
/// ))
/// (run 10)
///     ",
/// )?;
///
/// let big_number = 20;
///
/// // check that `(fib 20)` is not in the e-graph
/// let results = query(
///     &mut egraph,
///     vars![f: i64],
///     facts![(= (fib (unquote exprs::int(big_number))) f)],
/// )?;
///
/// assert!(results.iter().next().is_none());
///
/// let ruleset = "custom_ruleset";
/// add_ruleset(&mut egraph, ruleset)?;
///
/// // add the rule from `build_test_database` to the egraph
/// rust_rule(
///     &mut egraph,
///     "fib_rule",
///     ruleset,
///     vars![x: i64, f0: i64, f1: i64],
///     facts![
///         (= f0 (fib x))
///         (= f1 (fib (+ x 1)))
///     ],
///     move |ctx, values| {
///         let [x, f0, f1] = values else { unreachable!() };
///         let x = ctx.value_to_base::<i64>(*x);
///         let f0 = ctx.value_to_base::<i64>(*f0);
///         let f1 = ctx.value_to_base::<i64>(*f1);
///
///         let y = ctx.base_to_value::<i64>(x + 2);
///         let f2 = ctx.base_to_value::<i64>(f0 + f1);
///         ctx.insert("fib", [y, f2].into_iter());
///
///         Some(())
///     },
/// )?;
///
/// // run that rule 10 times
/// for _ in 0..10 {
///     run_ruleset(&mut egraph, ruleset)?;
/// }
///
/// // check that `(fib 20)` is now in the e-graph
/// let results = query(
///     &mut egraph,
///     vars![f: i64],
///     facts![(= (fib (unquote exprs::int(big_number))) f)],
/// )?;
///
/// let y = egraph.base_to_value::<i64>(6765);
/// let results: Vec<_> = results.iter().collect();
/// assert_eq!(results, [[y]]);
///
/// # Ok::<(), egglog::Error>(())
/// ```
pub fn rust_rule(
    egraph: &mut EGraph,
    rule_name: &str,
    ruleset: &str,
    vars: &[(&str, ArcSort)],
    facts: Facts<String, String>,
    func: impl Fn(&mut RustRuleContext, &[Value]) -> Option<()> + Clone + Send + Sync + 'static,
) -> Result<Vec<CommandOutput>, Error> {
    let prim_name = egraph.parser.symbol_gen.fresh("rust_rule_prim");
    let panic_id = egraph.backend.new_panic(format!("{prim_name}_panic"));
    let rust_rule_validator: PrimitiveValidator = std::sync::Arc::new(|termdag, _args| {
        // rust_rule callbacks perform side effects and return Unit.
        // This validator only checks/outputs the Unit result term.
        Some(termdag.lit(egglog_ast::generic_ast::Literal::Unit))
    });

    // This is the actual rule name that will appear in the proof-check program.
    // Any `Rule` proof terms we synthesize must use this exact string.
    let fresh_rule_name = egraph.parser.symbol_gen.fresh(rule_name);
    let mut func_ids: HashMap<String, egglog_bridge::FunctionId> = Default::default();
    func_ids.reserve(egraph.functions.len());
    let mut table_actions_by_id: Vec<Option<egglog_bridge::TableAction>> = Vec::new();
    for (k, v) in egraph.functions.iter() {
        func_ids.insert(k.clone(), v.backend_id);
        let idx = v.backend_id.rep() as usize;
        if table_actions_by_id.len() <= idx {
            table_actions_by_id.resize_with(idx + 1, || None);
        }
        table_actions_by_id[idx] = Some(egglog_bridge::TableAction::new(&egraph.backend, v.backend_id));
    }

    // term constructor name -> view table action
    let view_actions: HashMap<String, egglog_bridge::TableAction> = egraph
        .functions
        .iter()
        .filter_map(|(_k, v)| {
            v.decl.term_constructor.as_ref().map(|term_name| {
                (
                    term_name.clone(),
                    egglog_bridge::TableAction::new(&egraph.backend, v.backend_id),
                )
            })
        })
        .collect();

    let mut view_actions_by_id: Vec<Option<egglog_bridge::TableAction>> =
        vec![None; table_actions_by_id.len()];
    let mut term_output_sorts_by_id: Vec<Option<String>> = vec![None; table_actions_by_id.len()];
    for (term_name, view_action) in view_actions.iter() {
        let Some(term_id) = func_ids.get(term_name).copied() else {
            continue;
        };
        let idx = term_id.rep() as usize;
        view_actions_by_id[idx] = Some(view_action.clone());
        if let Some(func) = egraph.functions.get(term_name) {
            term_output_sorts_by_id[idx] = Some(func.decl.schema.output.clone());
        }
    }

    // Proof wiring (only used when proofs mode is enabled).
    //
    // Use the proof-encoding name cache rather than substring heuristics, otherwise
    // rulesets that introduce other `*ViewProof` functions (e.g. view-witness sorts)
    // can confuse the discovery and silently drop premise lists (`PCons`).
    let (fiat_action, rule_action, pcons_action, pnil_action, proof_sort) =
        if egraph.are_proofs_enabled() {
            let names = &egraph.proof_state.proof_names;
            let proof_sort = Some(names.proof_datatype.clone());
            let fiat_action = egraph
                .functions
                .get(&names.fiat_constructor)
                .map(|v| egglog_bridge::TableAction::new(&egraph.backend, v.backend_id));
            let rule_action = egraph
                .functions
                .get(&names.rule_constructor)
                .map(|v| egglog_bridge::TableAction::new(&egraph.backend, v.backend_id));
            let pcons_action = egraph
                .functions
                .get(&names.pcons)
                .map(|v| egglog_bridge::TableAction::new(&egraph.backend, v.backend_id));
            let pnil_action = egraph
                .functions
                .get(&names.pnil)
                .map(|v| egglog_bridge::TableAction::new(&egraph.backend, v.backend_id));
            (
                fiat_action,
                rule_action,
                pcons_action,
                pnil_action,
                proof_sort,
            )
        } else {
            (None, None, None, None, None)
        };

    let proof_list_sort = if egraph.are_proofs_enabled() {
        Some(egraph.proof_state.proof_names.proof_list_sort.clone())
    } else {
        None
    };

    let term_output_sorts: HashMap<String, String> = view_actions
        .keys()
        .filter_map(|term_name| {
            egraph
                .functions
                .get(term_name)
                .map(|f| (term_name.clone(), f.decl.schema.output.clone()))
        })
        .collect();

    let to_ast_actions: HashMap<String, egglog_bridge::TableAction> = term_output_sorts
        .values()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .filter_map(|sort_name| {
            let needle = format!("Ast{sort_name}");
            egraph.functions.iter().find_map(|(k, v)| {
                (k.contains(&needle) && v.decl.schema.input.as_slice() == [sort_name.as_str()])
                    .then(|| {
                        (
                            sort_name.to_string(),
                            egglog_bridge::TableAction::new(&egraph.backend, v.backend_id),
                        )
                    })
            })
        })
        .collect();

    let term_proof_actions: HashMap<String, egglog_bridge::TableAction> = proof_sort
        .as_ref()
        .map(|proof_sort| {
            term_output_sorts
                .values()
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .filter_map(|sort_name| {
                    let needle = format!("{sort_name}Proof");
                    egraph.functions.iter().find_map(|(k, v)| {
                        (k.contains(&needle)
                            && !k.contains("ViewProof")
                            && v.decl.schema.input.as_slice() == [sort_name.as_str()]
                            && &v.decl.schema.output == proof_sort)
                            .then(|| {
                                (
                                    sort_name.to_string(),
                                    egglog_bridge::TableAction::new(&egraph.backend, v.backend_id),
                                )
                            })
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let uf_proof_actions: HashMap<String, egglog_bridge::TableAction> = proof_sort
        .as_ref()
        .map(|proof_sort| {
            term_output_sorts
                .values()
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .filter_map(|sort_name| {
                    let uf_proof_name = egraph.proof_uf_proof_table_name(sort_name).ok()?;
                    let func = egraph.functions.get(uf_proof_name)?;
                    if func.decl.schema.input.as_slice() != [sort_name.as_str(), sort_name.as_str()]
                        || &func.decl.schema.output != proof_sort
                    {
                        return None;
                    }
                    Some((
                        sort_name.to_string(),
                        egglog_bridge::TableAction::new(&egraph.backend, func.backend_id),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();

    let uf_actions: HashMap<String, egglog_bridge::TableAction> = proof_sort
        .as_ref()
        .map(|_proof_sort| {
            term_output_sorts
                .values()
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .filter_map(|sort_name| {
                    let uf_name = egraph.proof_uf_table_name(sort_name).ok()?;
                    let func = egraph.functions.get(uf_name)?;
                    if func.decl.schema.input.as_slice() != [sort_name.as_str(), sort_name.as_str()]
                    {
                        return None;
                    }
                    Some((
                        sort_name.to_string(),
                        egglog_bridge::TableAction::new(&egraph.backend, func.backend_id),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();

    let view_proof_actions_by_id: Vec<Option<egglog_bridge::TableAction>> =
        if proof_sort.is_some() {
            let mut out: Vec<Option<egglog_bridge::TableAction>> = vec![None; table_actions_by_id.len()];
            for term_name in view_actions.keys() {
                let Some(term_id) = func_ids.get(term_name).copied() else {
                    continue;
                };
                let Ok(view_proof_name) = egraph.proof_view_proof_name(term_name) else {
                    continue;
                };
                let Some(func) = egraph.functions.get(view_proof_name) else {
                    continue;
                };
                out[term_id.rep() as usize] =
                    Some(egglog_bridge::TableAction::new(&egraph.backend, func.backend_id));
            }
            out
        } else {
            vec![None; table_actions_by_id.len()]
        };

    egraph.add_primitive_with_validator(
        RustRuleRhs {
            name: prim_name.clone(),
            rule_name: fresh_rule_name.clone(),
            inputs: vars.iter().map(|(_, s)| s.clone()).collect(),
            union_action: egglog_bridge::UnionAction::new(&egraph.backend),
            func_ids: Arc::new(func_ids),
            table_actions_by_id: Arc::new(table_actions_by_id),
            view_actions_by_id: Arc::new(view_actions_by_id),
            view_proof_actions_by_id: Arc::new(view_proof_actions_by_id),
            term_proof_actions: Arc::new(term_proof_actions),
            uf_actions: Arc::new(uf_actions),
            uf_proof_actions: Arc::new(uf_proof_actions),
            to_ast_actions: Arc::new(to_ast_actions),
            fiat_action,
            rule_action,
            pcons_action,
            pnil_action,
            pcons_name: egraph
                .are_proofs_enabled()
                .then(|| egraph.proof_state.proof_names.pcons.clone()),
            pnil_name: egraph
                .are_proofs_enabled()
                .then(|| egraph.proof_state.proof_names.pnil.clone()),
            term_output_sorts_by_id: Arc::new(term_output_sorts_by_id),
            panic_id,
            func,
        },
        Some(rust_rule_validator),
    );

    let rule = Rule {
        span: span!(),
        head: GenericActions(vec![GenericAction::Expr(
            span!(),
            exprs::call(
                &prim_name,
                vars.iter().map(|(v, _)| exprs::var(v)).collect(),
            ),
        )]),
        body: facts.0,
        name: fresh_rule_name,
        ruleset: ruleset.into(),
    };

    egraph.run_program(vec![Command::Rule { rule }])
}

/// The result of a query.
pub struct QueryResult {
    rows: usize,
    cols: usize,
    data: Vec<Value>,
}

impl QueryResult {
    /// Get an iterator over the query results,
    /// where each match is a `&[Value]` in the same order
    /// as the `vars` that were passed to `query`.
    pub fn iter(&self) -> impl Iterator<Item = &[Value]> {
        assert!(self.cols > 0, "no vars; use `any_matches` instead");
        assert!(self.data.len() % self.cols == 0);
        self.data.chunks_exact(self.cols)
    }

    /// Check if any matches were returned at all.
    pub fn any_matches(&self) -> bool {
        self.rows > 0
    }
}

/// Run a query over the database.
/// ```
/// use egglog::prelude::*;
///
/// let mut egraph = EGraph::default();
/// egraph.parse_and_run_program(
///     None,
///     "
/// (function fib (i64) i64 :no-merge)
/// (set (fib 0) 0)
/// (set (fib 1) 1)
/// (rule (
///     (= f0 (fib x))
///     (= f1 (fib (+ x 1)))
/// ) (
///     (set (fib (+ x 2)) (+ f0 f1))
/// ))
/// (run 10)
///     ",
/// )?;
///
/// let results = query(
///     &mut egraph,
///     vars![x: i64, y: i64],
///     facts![
///         (= (fib x) y)
///         (= y 13)
///     ],
/// )?;
///
/// let x = egraph.base_to_value::<i64>(7);
/// let y = egraph.base_to_value::<i64>(13);
/// let results: Vec<_> = results.iter().collect();
/// assert_eq!(results, [[x, y]]);
///
/// # Ok::<(), egglog::Error>(())
/// ```
pub fn query(
    egraph: &mut EGraph,
    vars: &[(&str, ArcSort)],
    facts: Facts<String, String>,
) -> Result<QueryResult, Error> {
    use std::sync::{Arc, Mutex};

    let results = Arc::new(Mutex::new(QueryResult {
        rows: 0,
        cols: vars.len(),
        data: Vec::new(),
    }));
    let results_weak = Arc::downgrade(&results);

    let ruleset = egraph.parser.symbol_gen.fresh("query_ruleset");
    add_ruleset(egraph, &ruleset)?;

    rust_rule(egraph, "query", &ruleset, vars, facts, move |_, values| {
        let arc = results_weak.upgrade().unwrap();
        let mut results = arc.lock().unwrap();
        results.rows += 1;
        results.data.extend(values);
        Some(())
    })?;

    run_ruleset(egraph, &ruleset)?;

    let ruleset = egraph.rulesets.swap_remove(&ruleset).unwrap();

    let Ruleset::Rules(rules) = ruleset else {
        unreachable!()
    };
    assert_eq!(rules.len(), 1);
    let rule = rules.into_iter().next().unwrap().1;
    egraph.backend.free_rule(rule.1);

    let Some(mutex) = Arc::into_inner(results) else {
        panic!("results_weak.upgrade() was not dropped");
    };
    Ok(mutex.into_inner().unwrap())
}

/// Declare a new sort.
pub fn add_sort(egraph: &mut EGraph, name: &str) -> Result<Vec<CommandOutput>, Error> {
    egraph.run_program(vec![Command::Sort {
        span: span!(),
        name: name.to_owned(),
        presort_and_args: None,
        uf: None,
        proof_func: None,
        unionable: true,
    }])
}

/// Declare a new function table.
pub fn add_function(
    egraph: &mut EGraph,
    name: &str,
    schema: Schema,
    merge: Option<GenericExpr<String, String>>,
) -> Result<Vec<CommandOutput>, Error> {
    egraph.run_program(vec![Command::Function {
        span: span!(),
        name: name.to_owned(),
        schema,
        merge,
        hidden: false,
        let_binding: false,
    }])
}

/// Declare a new constructor table.
pub fn add_constructor(
    egraph: &mut EGraph,
    name: &str,
    schema: Schema,
    cost: Option<DefaultCost>,
    unextractable: bool,
) -> Result<Vec<CommandOutput>, Error> {
    egraph.run_program(vec![Command::Constructor {
        span: span!(),
        name: name.to_owned(),
        schema,
        cost,
        unextractable,
        hidden: false,
        let_binding: false,
        term_constructor: None,
    }])
}

/// Declare a new relation table.
pub fn add_relation(
    egraph: &mut EGraph,
    name: &str,
    inputs: Vec<String>,
) -> Result<Vec<CommandOutput>, Error> {
    egraph.run_program(vec![Command::Relation {
        span: span!(),
        name: name.to_owned(),
        inputs,
    }])
}

/// Adds sorts and constructor tables to the database.
#[macro_export]
macro_rules! datatype {
    ($egraph:expr, (datatype $sort:ident $(($name:ident $($args:ident)* $(:cost $cost:expr)?))*)) => {
        add_sort($egraph, stringify!($sort))?;
        $(add_constructor(
            $egraph,
            stringify!($name),
            Schema {
                input: vec![$(stringify!($args).to_owned()),*],
                output: stringify!($sort).to_owned(),
            },
            [$($cost)*].first().copied(),
            false,
        )?;)*
    };
}

/// A "default" implementation of [`Sort`] for simple types
/// which just want to put some data in the e-graph. If you
/// implement this trait, do not implement `Sort` or
/// `ContainerSort. Use `add_base_sort` to register base
/// sorts with the `EGraph`. See `Sort` for documentation
/// of the methods. Do not override `to_arcsort`.
pub trait BaseSort: Any + Send + Sync + Debug {
    type Base: BaseValue;
    fn name(&self) -> &str;
    fn register_primitives(&self, _eg: &mut EGraph) {}
    fn reconstruct_termdag(&self, _: &BaseValues, _: Value, _: &mut TermDag) -> TermId;

    fn to_arcsort(self) -> ArcSort
    where
        Self: Sized,
    {
        Arc::new(BaseSortImpl(self))
    }
}

#[derive(Debug)]
struct BaseSortImpl<T: BaseSort>(T);

impl<T: BaseSort> Sort for BaseSortImpl<T> {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn column_ty(&self, backend: &egglog_bridge::EGraph) -> ColumnTy {
        ColumnTy::Base(backend.base_values().get_ty::<T::Base>())
    }

    fn register_type(&self, backend: &mut egglog_bridge::EGraph) {
        backend.base_values_mut().register_type::<T::Base>();
    }

    fn value_type(&self) -> Option<TypeId> {
        Some(TypeId::of::<T::Base>())
    }

    fn as_arc_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static> {
        self
    }

    fn register_primitives(self: Arc<Self>, eg: &mut EGraph) {
        self.0.register_primitives(eg)
    }

    /// Reconstruct a leaf base value in a TermDag
    fn reconstruct_termdag_base(
        &self,
        base_values: &BaseValues,
        value: Value,
        termdag: &mut TermDag,
    ) -> TermId {
        self.0.reconstruct_termdag(base_values, value, termdag)
    }
}

/// A "default" implementation of [`Sort`] for types which
/// just want to store a pure data structure in the e-graph.
/// If you implement this trait, do not implement `Sort` or
/// `BaseSort`. Use `add_container_sort` to register container
/// sorts with the `EGraph`. See `Sort` for documentation
/// of the methods. Do not override `to_arcsort`.
pub trait ContainerSort: Any + Send + Sync + Debug {
    type Container: ContainerValue;
    fn name(&self) -> &str;
    fn is_eq_container_sort(&self) -> bool;
    fn inner_sorts(&self) -> Vec<ArcSort>;
    fn inner_values(&self, _: &ContainerValues, _: Value) -> Vec<(ArcSort, Value)>;
    fn register_primitives(&self, _eg: &mut EGraph) {}
    fn reconstruct_termdag(
        &self,
        _: &ContainerValues,
        _: Value,
        _: &mut TermDag,
        _: Vec<TermId>,
    ) -> TermId;
    fn serialized_name(&self, container_values: &ContainerValues, value: Value) -> String;

    fn to_arcsort(self) -> ArcSort
    where
        Self: Sized,
    {
        Arc::new(ContainerSortImpl(self))
    }
}

#[derive(Debug)]
struct ContainerSortImpl<T: ContainerSort>(T);

impl<T: ContainerSort> Sort for ContainerSortImpl<T> {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn column_ty(&self, _backend: &egglog_bridge::EGraph) -> ColumnTy {
        ColumnTy::Id
    }

    fn register_type(&self, backend: &mut egglog_bridge::EGraph) {
        backend.register_container_ty::<T::Container>();
    }

    fn value_type(&self) -> Option<TypeId> {
        Some(TypeId::of::<T::Container>())
    }

    fn as_arc_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static> {
        self
    }

    fn inner_sorts(&self) -> Vec<ArcSort> {
        self.0.inner_sorts()
    }

    fn inner_values(
        &self,
        container_values: &ContainerValues,
        value: Value,
    ) -> Vec<(ArcSort, Value)> {
        self.0.inner_values(container_values, value)
    }

    fn is_container_sort(&self) -> bool {
        true
    }

    fn is_eq_container_sort(&self) -> bool {
        self.0.is_eq_container_sort()
    }

    fn serialized_name(&self, container_values: &ContainerValues, value: Value) -> String {
        self.0.serialized_name(container_values, value)
    }

    fn register_primitives(self: Arc<Self>, eg: &mut EGraph) {
        self.0.register_primitives(eg);
    }

    fn reconstruct_termdag_container(
        &self,
        container_values: &ContainerValues,
        value: Value,
        termdag: &mut TermDag,
        element_terms: Vec<TermId>,
    ) -> TermId {
        self.0
            .reconstruct_termdag(container_values, value, termdag, element_terms)
    }
}

/// Add a [`BaseSort`] to the e-graph
pub fn add_base_sort(
    egraph: &mut EGraph,
    base_sort: impl BaseSort,
    span: Span,
) -> Result<(), TypeError> {
    egraph.add_sort(BaseSortImpl(base_sort), span)
}

pub fn add_container_sort(
    egraph: &mut EGraph,
    container_sort: impl ContainerSort,
    span: Span,
) -> Result<(), TypeError> {
    egraph.add_sort(ContainerSortImpl(container_sort), span)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_database() -> Result<EGraph, Error> {
        let mut egraph = EGraph::default();
        egraph.parse_and_run_program(
            None,
            "
(function fib (i64) i64 :no-merge)
(set (fib 0) 0)
(set (fib 1) 1)
(rule (
    (= f0 (fib x))
    (= f1 (fib (+ x 1)))
) (
    (set (fib (+ x 2)) (+ f0 f1))
))
(run 10)
        ",
        )?;
        Ok(egraph)
    }

    #[test]
    fn rust_api_query() -> Result<(), Error> {
        let mut egraph = build_test_database()?;

        let results = query(
            &mut egraph,
            vars![x: i64, y: i64],
            facts![
                (= (fib x) y)
                (= y 13)
            ],
        )?;

        let x = egraph.backend.base_values().get::<i64>(7);
        let y = egraph.backend.base_values().get::<i64>(13);
        assert_eq!(results.data, [x, y]);

        Ok(())
    }

    #[test]
    fn rust_api_rule() -> Result<(), Error> {
        let mut egraph = build_test_database()?;

        let big_number = 20;

        // check that `(fib 20)` is not in the e-graph
        let results = query(
            &mut egraph,
            vars![f: i64],
            facts![(= (fib (unquote exprs::int(big_number))) f)],
        )?;

        assert!(results.data.is_empty());

        let ruleset = "custom_ruleset";
        add_ruleset(&mut egraph, ruleset)?;

        // add the rule from `build_test_database` to the egraph
        rule(
            &mut egraph,
            ruleset,
            facts![
                (= f0 (fib x))
                (= f1 (fib (+ x 1)))
            ],
            actions![
                (set (fib (+ x 2)) (+ f0 f1))
            ],
        )?;

        // run that rule 10 times
        for _ in 0..10 {
            run_ruleset(&mut egraph, ruleset)?;
        }

        // check that `(fib 20)` is now in the e-graph
        let results = query(
            &mut egraph,
            vars![f: i64],
            facts![(= (fib (unquote exprs::int(big_number))) f)],
        )?;

        let y = egraph.backend.base_values().get::<i64>(6765);
        assert_eq!(results.data, [y]);

        Ok(())
    }

    #[test]
    fn rust_api_macros() -> Result<(), Error> {
        let mut egraph = build_test_database()?;

        datatype!(&mut egraph, (datatype Expr (One) (Two Expr Expr :cost 10)));

        let ruleset = "custom_ruleset";
        add_ruleset(&mut egraph, ruleset)?;

        rule(
            &mut egraph,
            ruleset,
            facts![
                (fib 5)
                (fib x)
                (= f1 (fib (+ x 1)))
                (= 3 (unquote exprs::int(1 + 2)))
            ],
            actions![
                (let y (+ x 2))
                (set (fib (+ x 2)) (+ f1 f1))
                (delete (fib 0))
                (subsume (Two (One) (One)))
                (union (One) (Two (One) (One)))
                (panic "message")
                (+ 6 87)
            ],
        )?;

        Ok(())
    }

    #[test]
    fn rust_api_rust_rule() -> Result<(), Error> {
        let mut egraph = build_test_database()?;

        let big_number = 20;

        // check that `(fib 20)` is not in the e-graph
        let results = query(
            &mut egraph,
            vars![f: i64],
            facts![(= (fib (unquote exprs::int(big_number))) f)],
        )?;

        assert!(results.data.is_empty());

        let ruleset = "custom_ruleset";
        add_ruleset(&mut egraph, ruleset)?;

        // add the rule from `build_test_database` to the egraph
        rust_rule(
            &mut egraph,
            "demo_rule",
            ruleset,
            vars![x: i64, f0: i64, f1: i64],
            facts![
                (= f0 (fib x))
                (= f1 (fib (+ x 1)))
            ],
            move |ctx, values| {
                let [x, f0, f1] = values else { unreachable!() };
                let x = ctx.value_to_base::<i64>(*x);
                let f0 = ctx.value_to_base::<i64>(*f0);
                let f1 = ctx.value_to_base::<i64>(*f1);

                let y = ctx.base_to_value::<i64>(x + 2);
                let f2 = ctx.base_to_value::<i64>(f0 + f1);
                ctx.insert("fib", [y, f2].into_iter());

                Some(())
            },
        )?;

        // run that rule 10 times
        for _ in 0..10 {
            run_ruleset(&mut egraph, ruleset)?;
        }

        // check that `(fib 20)` is now in the e-graph
        let results = query(
            &mut egraph,
            vars![f: i64],
            facts![(= (fib (unquote exprs::int(big_number))) f)],
        )?;

        let y = egraph.backend.base_values().get::<i64>(6765);
        assert_eq!(results.data, [y]);

        Ok(())
    }
}
