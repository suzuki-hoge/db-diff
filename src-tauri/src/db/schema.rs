// @generated automatically by Diesel CLI.

diesel::table! {
    projects (project_id) {
        project_id -> Text,
        name -> Text,
        rdbms -> Text,
        user -> Text,
        password -> Text,
        host -> Text,
        port -> Text,
        schema -> Text,
    }
}

diesel::table! {
    snapshot_diffs (diff_id) {
        diff_id -> Text,
        snapshot_id1 -> Text,
        snapshot_id2 -> Text,
        data -> Text,
    }
}

diesel::table! {
    snapshot_summaries (snapshot_id) {
        snapshot_id -> Text,
        project_id -> Text,
        snapshot_name -> Text,
        create_at -> Text,
    }
}

diesel::table! {
    table_snapshots (snapshot_id, table_name) {
        snapshot_id -> Text,
        table_name -> Text,
        data -> Text,
    }
}

diesel::joinable!(snapshot_summaries -> projects (project_id));
diesel::joinable!(table_snapshots -> snapshot_summaries (snapshot_id));

diesel::allow_tables_to_appear_in_same_query!(projects, snapshot_diffs, snapshot_summaries, table_snapshots,);
