use rhai::{Engine, EvalAltResult};

#[test]
fn test_internal_fn() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();

    assert_eq!(engine.eval::<i64>("fn addme(a, b) { a+b } addme(3, 4)")?, 7);
    assert_eq!(engine.eval::<i64>("fn bob() { return 4; 5 } bob()")?, 4);

    Ok(())
}

#[test]
fn test_big_internal_fn() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();

    assert_eq!(
        engine.eval::<i64>(
            r"
                fn mathme(a, b, c, d, e, f) {
                    a - b * c + d * e - f
                }
                mathme(100, 5, 2, 9, 6, 32)
            ",
        )?,
        112
    );

    Ok(())
}
