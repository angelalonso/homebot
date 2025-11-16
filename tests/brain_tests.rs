// tests/brain_tests.rs

use std::collections::BTreeMap;
use std::time::Duration;
use tokio::runtime::Runtime;

use homebot::brain::*;
use homebot::loggin::Log;

fn test_log() -> Log {
    Log::init("DEBUG".to_string())
}

fn test_brain() -> Option<Brain> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match Brain::init(test_log(), BTreeMap::new(), true, 10).await {
            Ok(brain) => Some(brain),
            Err(e) if e.to_string().contains("No Arduino found") => {
                eprintln!("Skipping test (no Arduino): {e}");
                None
            }
            Err(e) => panic!("Unexpected error during Brain::init: {:?}", e),
        }
    })
}

#[tokio::test]
async fn test_brain_initializes() {
    match Brain::init(test_log(), BTreeMap::new(), true, 10).await {
        Ok(brain) => {
            assert!(brain.get_current().is_empty());
            assert!(brain.get_incoming().is_empty());
            assert!(brain.rules.is_empty());
        }
        Err(e) if e.to_string().contains("No Arduino found") => {
            eprintln!("Skipping test_brain_initializes (no Arduino): {e}");
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_add_and_sort_rules() {
    if let Some(mut brain) = test_brain() {
        let low = Rule {
            name: "low".to_string(),
            priority: 1,
            condition: Condition::Always,
            actions: vec![],
            cooldown: None,
        };
        let mut high = low.clone();
        high.name = "high".to_string();
        high.priority = 99;

        brain.add_rule(low);
        brain.add_rule(high);

        assert_eq!(brain.rules[0].name, "high");
        assert_eq!(brain.rules[1].name, "low");
    }
}

#[test]
fn test_add_default_rules() {
    if let Some(mut brain) = test_brain() {
        brain.add_default_rules();
        let names: Vec<_> = brain.rules.iter().map(|r| r.name.clone()).collect();
        assert!(names.contains(&"emergency_stop".to_string()));
        assert!(names.contains(&"normal_operation".to_string()));
    }
}

#[tokio::test]
async fn test_update_triggers_rule() {
    match Brain::init(test_log(), BTreeMap::new(), true, 10).await {
        Ok(mut brain) => {
            brain.add_rule(Rule {
                name: "always_rule".to_string(),
                priority: 100,
                condition: Condition::Always,
                actions: vec![RuleAction {
                    action_type: RuleActionType::Log("triggered".to_string()),
                    priority: 10,
                }],
                cooldown: None,
            });

            let _output = brain.update(test_log(), "off".to_string()).await;

            assert!(!brain.get_incoming().is_empty());
        }
        Err(e) if e.to_string().contains("No Arduino found") => {
            eprintln!("Skipping test_update_triggers_rule (no Arduino): {e}");
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[tokio::test]
async fn test_update_distance_behavior() {
    match Brain::init(test_log(), BTreeMap::new(), true, 10).await {
        Ok(mut brain) => {
            let output = brain.update(test_log(), "off".to_string()).await;
            let _ = output.clone(); // should not panic
        }
        Err(e) if e.to_string().contains("No Arduino found") => {
            eprintln!("Skipping test_update_distance_behavior (no Arduino): {e}");
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

