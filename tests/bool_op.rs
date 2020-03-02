use rhai::{Engine, EvalAltResult};

#[test]
fn test_bool_op1() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();

    assert_eq!(engine.eval::<bool>("true && (false || true)")?, true);
    assert_eq!(engine.eval::<bool>("true & (false | true)")?, true);

    Ok(())
}

#[test]
fn test_bool_op2() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();

    assert_eq!(engine.eval::<bool>("false && (false || true)")?, false);
    assert_eq!(engine.eval::<bool>("false & (false | true)")?, false);

    Ok(())
}

#[test]
fn test_bool_op3() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();

    assert!(engine.eval::<bool>("true && (false || 123)").is_err());
    assert_eq!(engine.eval::<bool>("true && (true || 123)")?, true);
    assert!(engine.eval::<bool>("123 && (false || true)").is_err());
    assert_eq!(engine.eval::<bool>("false && (true || 123)")?, false);

    Ok(())
}

#[test]
fn test_bool_op_short_circuit() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();

    assert_eq!(
        engine.eval::<bool>(
            r"
            fn this() { true }
            fn that() { 9/0 }

            this() || that();
        "
        )?,
        true
    );

    assert_eq!(
        engine.eval::<bool>(
            r"
            fn this() { false }
            fn that() { 9/0 }

            this() && that();
        "
        )?,
        false
    );

    Ok(())
}

#[test]
#[should_panic]
fn test_bool_op_no_short_circuit1() {
    let mut engine = Engine::new();

    assert_eq!(
        engine
            .eval::<bool>(
                r"
                    fn this() { false }
                    fn that() { 9/0 }

                    this() | that();
                "
            )
            .unwrap(),
        false
    );
}

#[test]
#[should_panic]
fn test_bool_op_no_short_circuit2() {
    let mut engine = Engine::new();

    assert_eq!(
        engine
            .eval::<bool>(
                r"
                    fn this() { false }
                    fn that() { 9/0 }

                    this() & that();
                "
            )
            .unwrap(),
        false
    );
}
