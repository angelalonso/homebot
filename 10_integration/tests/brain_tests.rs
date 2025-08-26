// tests/brain_tests.rs

use std::collections::BTreeMap;
use std::time::Duration;
use tokio::runtime::Runtime;

use homebot::brain::*; // <-- replace `my_robot` with the name in your Cargo.toml
use homebot::loggin::Log;

fn test_log() -> Log {
    let log = Log::init("DEBUG".to_string());
    return log
}

fn test_brain() -> Brain {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        Brain::init(test_log(), BTreeMap::new(), true, 10)
            .await
            .unwrap()
    })
}

#[tokio::test]
async fn test_brain_initializes() {
    let brain = Brain::init(test_log(), BTreeMap::new(), true, 10)
        .await
        .unwrap();
    assert!(brain.get_current().is_empty());
    assert!(brain.get_incoming().is_empty());
    assert!(brain.rules.is_empty());
}

#[test]
fn test_add_and_sort_rules() {
    let mut brain = test_brain();
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

    // Highest priority rule should be first
    assert_eq!(brain.rules[0].name, "high");
    assert_eq!(brain.rules[1].name, "low");
}

#[test]
fn test_add_default_rules() {
    let mut brain = test_brain();
    brain.add_default_rules();
    let names: Vec<_> = brain.rules.iter().map(|r| r.name.clone()).collect();
    assert!(names.contains(&"emergency_stop".to_string()));
    assert!(names.contains(&"normal_operation".to_string()));
}

#[tokio::test]
async fn test_update_triggers_rule() {
    let mut brain = Brain::init(test_log(), BTreeMap::new(), true, 10)
        .await
        .unwrap();

    // Add a simple always-true rule
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

    // Run update (this will call evaluate_rules internally)
    let _output = brain.update(test_log(), "off".to_string()).await;

    // The rule should have scheduled an action
    assert!(!brain.get_incoming().is_empty());
}

#[tokio::test]
async fn test_update_distance_behavior() {
    let mut brain = Brain::init(test_log(), BTreeMap::new(), true, 10)
        .await
        .unwrap();

    // No rules, just check update drives motors depending on distance
    let output = brain.update(test_log(), "off".to_string()).await;

    // Check that the output contains motor settings
    // (Exact behavior depends on your Output implementation)
    // Here we just assert we can clone it without panic
    let _ = output.clone();
}

