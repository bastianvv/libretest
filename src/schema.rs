table! {
    linked_requirements (id) {
        id -> Int4,
        parent_id -> Int4,
        child_id -> Int4,
    }
}

table! {
    linked_test_cases (id) {
        id -> Int4,
        parent_id -> Int4,
        child_id -> Int4,
    }
}

table! {
    permissions (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    requirement_importance (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    requirement_nature (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    requirement_status (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    requirements (id) {
        id -> Int4,
        creation_date -> Timestamp,
        created_by -> Int4,
        updated_date -> Nullable<Timestamp>,
        updated_by -> Nullable<Int4>,
        code -> Nullable<Varchar>,
        status -> Int4,
        importance -> Nullable<Int4>,
        nature -> Nullable<Int4>,
        title -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    test_case_gherkin (id) {
        id -> Int4,
        tc_id -> Int4,
        script -> Nullable<Varchar>,
    }
}

table! {
    test_case_importance (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    test_case_nature (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    test_case_status (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    test_case_steps (id) {
        id -> Int4,
        tc_id -> Int4,
        step_order -> Int4,
        step_description -> Nullable<Varchar>,
        step_expected_results -> Nullable<Varchar>,
    }
}

table! {
    test_case_type (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    test_cases (id) {
        id -> Int4,
        creation_date -> Timestamp,
        created_by -> Int4,
        updated_date -> Nullable<Timestamp>,
        updated_by -> Nullable<Int4>,
        code -> Nullable<Varchar>,
        status -> Int4,
        importance -> Nullable<Int4>,
        nature -> Nullable<Int4>,
        test_type -> Int4,
        automated -> Bool,
        title -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    test_execution_status (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    test_executions (id) {
        id -> Int4,
        creation_date -> Timestamp,
        created_by -> Int4,
        updated_date -> Nullable<Timestamp>,
        updated_by -> Nullable<Int4>,
        test_plan_id -> Int4,
        test_case_id -> Int4,
        execution_status -> Int4,
        execution_date -> Nullable<Timestamp>,
        executed_by -> Nullable<Int4>,
    }
}

table! {
    test_plan_status (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    test_plans (id) {
        id -> Int4,
        creation_date -> Timestamp,
        created_by -> Int4,
        updated_date -> Nullable<Timestamp>,
        updated_by -> Nullable<Int4>,
        code -> Nullable<Varchar>,
        status -> Int4,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        title -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    traceability (id) {
        id -> Int4,
        req_id -> Int4,
        tc_id -> Int4,
    }
}

table! {
    user_group (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    user_group_permissions (id) {
        id -> Int4,
        user_group_id -> Int4,
        permission_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        admin -> Bool,
        user_group_id -> Nullable<Int4>,
    }
}

joinable!(requirements -> requirement_importance (importance));
joinable!(requirements -> requirement_nature (nature));
joinable!(requirements -> requirement_status (status));
joinable!(test_case_gherkin -> test_cases (tc_id));
joinable!(test_case_steps -> test_cases (tc_id));
joinable!(test_cases -> test_case_importance (importance));
joinable!(test_cases -> test_case_nature (nature));
joinable!(test_cases -> test_case_status (status));
joinable!(test_cases -> test_case_type (test_type));
joinable!(test_executions -> test_cases (test_case_id));
joinable!(test_executions -> test_execution_status (execution_status));
joinable!(test_executions -> test_plans (test_plan_id));
joinable!(test_plans -> test_plan_status (status));
joinable!(traceability -> requirements (req_id));
joinable!(traceability -> test_cases (tc_id));
joinable!(user_group_permissions -> permissions (permission_id));
joinable!(user_group_permissions -> user_group (user_group_id));
joinable!(users -> user_group (user_group_id));

allow_tables_to_appear_in_same_query!(
    linked_requirements,
    linked_test_cases,
    permissions,
    requirement_importance,
    requirement_nature,
    requirement_status,
    requirements,
    test_case_gherkin,
    test_case_importance,
    test_case_nature,
    test_case_status,
    test_case_steps,
    test_case_type,
    test_cases,
    test_execution_status,
    test_executions,
    test_plan_status,
    test_plans,
    traceability,
    user_group,
    user_group_permissions,
    users,
);
