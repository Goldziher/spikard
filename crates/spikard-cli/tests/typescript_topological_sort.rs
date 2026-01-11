//! Test TypeScript code generation with complex nested schemas
//! Verifies that schemas are generated in topologically sorted order
//! to avoid "variable used before declaration" errors

use spikard_cli::codegen::{NodeDtoStyle, TypeScriptGenerator};
use std::fs;

#[test]
fn typescript_schema_generation_handles_complex_nested_with_correct_ordering() {
    let schema_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testing_data/openapi_schemas/complex_nested.json"
    );
    let schema_content = fs::read_to_string(schema_path).expect("Failed to read schema file");
    let spec: openapiv3::OpenAPI = serde_json::from_str(&schema_content).expect("Failed to parse OpenAPI schema");

    let generator = TypeScriptGenerator::new(spec, NodeDtoStyle::Zod);
    let output = generator.generate().expect("Failed to generate TypeScript");

    println!("Generated TypeScript code:\n{output}");

    // Verify that all expected schemas were generated
    assert!(
        output.contains("export const SubtaskSchema"),
        "Should generate Subtask schema"
    );
    assert!(
        output.contains("export const MemberSchema"),
        "Should generate Member schema"
    );
    assert!(
        output.contains("export const TaskSchema"),
        "Should generate Task schema"
    );
    assert!(
        output.contains("export const OrganizationSchema"),
        "Should generate Organization schema"
    );
    assert!(
        output.contains("export const ProjectSchema"),
        "Should generate Project schema"
    );

    // Verify no TypeScript errors in ordering - dependencies should come before their usage
    // Member is referenced by Organization, Task, and Project
    let member_pos = output
        .find("export const MemberSchema")
        .expect("Member schema not found");
    let organization_pos = output
        .find("export const OrganizationSchema")
        .expect("Organization schema not found");
    let task_pos = output.find("export const TaskSchema").expect("Task schema not found");
    let project_pos = output
        .find("export const ProjectSchema")
        .expect("Project schema not found");

    assert!(
        member_pos < organization_pos,
        "Member should be defined before Organization (Member is referenced by Organization)"
    );
    assert!(
        member_pos < task_pos,
        "Member should be defined before Task (Member is referenced by Task)"
    );
    assert!(
        member_pos < project_pos,
        "Member should be defined before Project (Member is referenced by Project)"
    );

    // TaskTimeline should come before Task (Task references it)
    let task_timeline_pos = output
        .find("export const TaskTimelineSchema")
        .expect("TaskTimeline schema not found");
    let task_schema_def_pos = output
        .find("export const TaskSchema")
        .expect("Task schema definition not found");
    assert!(
        task_timeline_pos < task_schema_def_pos,
        "TaskTimeline should be defined before Task (Task references TaskTimeline)"
    );

    // Subtask should come before Task (Task references it in array)
    let subtask_pos = output
        .find("export const SubtaskSchema")
        .expect("Subtask schema not found");
    assert!(
        subtask_pos < task_schema_def_pos,
        "Subtask should be defined before Task (Task references Subtask in array)"
    );

    // Milestone should come before Timeline (Timeline references it in array)
    let milestone_pos = output
        .find("export const MilestoneSchema")
        .expect("Milestone schema not found");
    let timeline_def_pos = output
        .find("export const TimelineSchema")
        .expect("Timeline definition not found");
    assert!(
        milestone_pos < timeline_def_pos,
        "Milestone should be defined before Timeline (Timeline references Milestone)"
    );

    // Timeline should come before Project (Project references Timeline)
    let project_def_pos = output
        .find("export const ProjectSchema")
        .expect("Project definition not found");
    assert!(
        timeline_def_pos < project_def_pos,
        "Timeline should be defined before Project (Project references Timeline)"
    );

    // Verify schema definitions use correct Zod syntax
    assert!(output.contains("z.object"), "Should generate Zod objects");
    assert!(output.contains("z.array"), "Should generate Zod arrays");
    assert!(output.contains("z.string"), "Should generate Zod strings");
    assert!(output.contains("export type"), "Should generate type exports");

    // Note: Forward reference check is complex due to line number tracking
    // The topological sort implementation ensures dependencies come before their usage
    // as verified by the assertions above
}

#[test]
fn typescript_generation_handles_simple_schema_without_cycles() {
    let simple_spec = serde_json::json!({
        "openapi": "3.0.0",
        "info": { "title": "Simple API", "version": "1.0.0" },
        "paths": {},
        "components": {
            "schemas": {
                "User": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string" },
                        "name": { "type": "string" }
                    },
                    "required": ["id", "name"]
                }
            }
        }
    });

    let spec: openapiv3::OpenAPI = serde_json::from_value(simple_spec).expect("Failed to create spec");

    let generator = TypeScriptGenerator::new(spec, NodeDtoStyle::Zod);
    let output = generator.generate().expect("Failed to generate TypeScript");

    assert!(
        output.contains("export const UserSchema"),
        "Should generate User schema"
    );
    assert!(output.contains("z.object"), "Should use Zod z.object");
    assert!(output.contains("id: z.string()"), "Should have id as string");
    assert!(output.contains("name: z.string()"), "Should have name as string");
}
