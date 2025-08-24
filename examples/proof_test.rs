use egglog::{EGraph, SerializeConfig};
use egglog_bridge::ProofStore;

fn main() {
    env_logger::init();
    let mut egraph = egglog::EGraph::with_tracing();
    declare(&mut egraph);
    let expr1 = egraph
        .parser
        .get_expr_from_string(None, "(MAdd (MNum 2) (MNum 1))")
        .unwrap();
    let expr2 = egraph
        .parser
        .get_expr_from_string(None, "(MNum 3)")
        .unwrap();
    let (_, v1) = egraph.eval_expr(&expr1).unwrap();
    let (_, v2) = egraph.eval_expr(&expr2).unwrap();
    egraph
        .serialize(SerializeConfig::default())
        .to_dot_file("examples/proof_test_src.dot")
        .unwrap();

    egraph
        .parse_and_run_program(
            None,
            "
        (run 10)
    ",
        )
        .unwrap();
    egraph
        .serialize(SerializeConfig::default())
        .to_dot_file("examples/proof_test.dot")
        .unwrap();
    let mut prf_store = ProofStore::default();
    let proof = egraph
        .backend
        .explain_terms_equal(v1, v2, &mut prf_store)
        .unwrap();
    prf_store
        .print_eq_proof(proof, &mut std::io::stdout())
        .unwrap();
    // let Ok(proof) = proof else { panic!() };
    // proof.lhs.dump_explanation(&mut std::io::stdout()).unwrap();
    // proof.rhs.dump_explanation(&mut std::io::stdout()).unwrap();
    // explain_term
    // println!("{:#?}", proof);
    // egraph.backend.dump_debug_info();
}

fn declare(egraph: &mut EGraph) {
    egraph
        .parse_and_run_program(
            None,
            "
        (datatype Math
            (MNum i64)
            (MVar String)
            (MAdd Math Math)
            (MSub Math Math)
            (MMul Math Math)
            (MDiv Math Math)
            (MMod Math Math)
            (MMin Math Math)
            (MMax Math Math)
            (MAnd Math Math)
            (MOr Math Math)
            (MGte Math Math)
            (MLt Math Math)
            (MFloorTo Math Math)
            (MReplace Math Math Math)
            (MAccum) ; this marks that we feed the output (also marked with MAccum) back in
        )

        ; Communative
        (rewrite (MAdd a b) (MAdd b a))
        ;(rewrite (MMul a b) (MMul b a))
        ;(rewrite (MMin a b) (MMin b a))
        ;(rewrite (MMax a b) (MMax b a))
        ;(rewrite (MAnd a b) (MAnd b a))
        ;(rewrite (MOr a b) (MOr b a))

        ; Associative
        (rewrite (MAdd (MAdd a b) c) (MAdd a (MAdd b c)))
        (rewrite (MMul (MMul a b) c) (MMul a (MMul b c)))
        ;(rewrite (MDiv (MDiv a b) c) (MDiv a (MMul b c)))
        ;(rewrite (MDiv (MMul a b) c) (MMul a (MDiv b c)))
        ;(rewrite (MMul a (MDiv b c)) (MDiv (MMul a b) c))

        ; Distributive
        ;(rewrite (MMul a (MAdd b c)) (MAdd (MMul a b) (MMul a c)))
        ;(rewrite (MDiv (MAdd a b) c) (MAdd (MDiv a c) (MDiv b c)))

        ; Constant folding
        (rewrite (MAdd (MNum a) (MNum b)) (MNum (+ a b)))
        (rewrite (MSub (MNum a) (MNum b)) (MNum (- a b)))
        (rewrite (MMul (MNum a) (MNum b)) (MNum (* a b)) :when ((< a 10000) (< b 10000)))
        (rewrite (MDiv (MNum a) (MNum b)) (MNum (/ a b)) :when ((!= 0 b) (= 0 (% a b))))
        (rewrite (MMax (MNum a) (MNum b)) (MNum (max a b)))
        (rewrite (MMin (MNum a) (MNum b)) (MNum (min a b)))
        (rewrite (MAnd (MNum a) (MNum b)) (MNum (& a b)))
        ;(rewrite (MOr (MNum a) (MNum b)) (MNum (| a b)))
    ",
        )
        .unwrap();
}
