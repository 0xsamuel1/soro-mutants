use cargo_soro_mutants::report::{
    serialize_json_with_breakdowns, summary_by_function, OutcomeCounts,
    ReportMutant, ReportOutcome, ReportResult, ReportSpan,
};

#[test]
fn test_per_function_summary_counts() {
    let results = vec![
        ReportResult {
            mutant: ReportMutant {
                id: "M0001".into(),
                operator: "AUTH-001".into(),
                file: "src/lib.rs".into(),
                function: Some("initialize".into()),
                span: ReportSpan { line: 10, column: 5 },
                original: "orig".into(),
                replacement: "repl".into(),
                description: "desc".into(),
            },
            outcome: ReportOutcome::Killed,
        },
        ReportResult {
            mutant: ReportMutant {
                id: "M0002".into(),
                operator: "AUTH-002".into(),
                file: "src/lib.rs".into(),
                function: Some("initialize".into()),
                span: ReportSpan { line: 12, column: 5 },
                original: "orig".into(),
                replacement: "repl".into(),
                description: "desc".into(),
            },
            outcome: ReportOutcome::Survived,
        },
        ReportResult {
            mutant: ReportMutant {
                id: "M0003".into(),
                operator: "AUTH-001".into(),
                file: "src/lib.rs".into(),
                function: Some("transfer".into()),
                span: ReportSpan { line: 25, column: 5 },
                original: "orig".into(),
                replacement: "repl".into(),
                description: "desc".into(),
            },
            outcome: ReportOutcome::Killed,
        },
        ReportResult {
            mutant: ReportMutant {
                id: "M0004".into(),
                operator: "AUTH-001".into(),
                file: "src/lib.rs".into(),
                function: None,
                span: ReportSpan { line: 30, column: 5 },
                original: "orig".into(),
                replacement: "repl".into(),
                description: "desc".into(),
            },
            outcome: ReportOutcome::Killed,
        },
    ];

    let by_func = summary_by_function(&results);
    assert_eq!(by_func.len(), 2);
    assert_eq!(
        by_func["initialize"],
        OutcomeCounts {
            generated: 2,
            killed: 1,
            survived: 1,
            unviable: 0,
            timeout: 0,
        }
    );
    assert_eq!(
        by_func["transfer"],
        OutcomeCounts {
            generated: 1,
            killed: 1,
            survived: 0,
            unviable: 0,
            timeout: 0,
        }
    );

    let json = serialize_json_with_breakdowns(&results, None).expect("serialization succeeds");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["by_function"]["initialize"]["generated"], 2);
    assert_eq!(parsed["by_function"]["transfer"]["killed"], 1);
}
