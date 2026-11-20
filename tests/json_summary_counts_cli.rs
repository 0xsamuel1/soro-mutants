use cargo_soro_mutants::report::{
    serialize_json_with_breakdowns, summary_by_file, summary_by_operator, OutcomeCounts,
    ReportMutant, ReportOutcome, ReportResult, ReportSpan,
};

#[test]
fn test_per_operator_and_per_file_summary_counts() {
    let results = vec![
        ReportResult {
            mutant: ReportMutant {
                id: "M0001".into(),
                operator: "AUTH-001".into(),
                file: "src/admin.rs".into(),
                function: Some("check".into()),
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
                operator: "AUTH-001".into(),
                file: "src/admin.rs".into(),
                function: Some("check".into()),
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
                operator: "TOKEN-001".into(),
                file: "src/token.rs".into(),
                function: Some("transfer".into()),
                span: ReportSpan { line: 20, column: 5 },
                original: "orig".into(),
                replacement: "repl".into(),
                description: "desc".into(),
            },
            outcome: ReportOutcome::Unviable,
        },
    ];

    let by_op = summary_by_operator(&results);
    assert_eq!(by_op.len(), 2);
    assert_eq!(
        by_op["AUTH-001"],
        OutcomeCounts {
            generated: 2,
            killed: 1,
            survived: 1,
            unviable: 0,
            timeout: 0,
        }
    );
    assert_eq!(
        by_op["TOKEN-001"],
        OutcomeCounts {
            generated: 1,
            killed: 0,
            survived: 0,
            unviable: 1,
            timeout: 0,
        }
    );

    let by_file = summary_by_file(&results);
    assert_eq!(by_file.len(), 2);
    assert_eq!(by_file["src/admin.rs"].generated, 2);
    assert_eq!(by_file["src/token.rs"].generated, 1);

    let json = serialize_json_with_breakdowns(&results, None).expect("serialization succeeds");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["by_operator"]["AUTH-001"]["killed"], 1);
    assert_eq!(parsed["by_operator"]["AUTH-001"]["survived"], 1);
    assert_eq!(parsed["by_file"]["src/admin.rs"]["generated"], 2);
}
