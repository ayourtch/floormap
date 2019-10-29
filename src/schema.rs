table! {
    Comments (RecordUUID) {
        RecordUUID -> Text,
        Deleted -> Bool,
        ChangesetID -> Integer,
        CommentID -> Integer,
    }
}

table! {
    Jobs (RecordUUID) {
        RecordUUID -> Text,
        JobGrouName -> Text,
        InstanceID -> Integer,
        JobID -> Text,
        JobPID -> Integer,
        ParentJobID -> Nullable<Text>,
        changeset_id -> Integer,
        patchset_id -> Integer,
        command -> Text,
        command_pid -> Nullable<Integer>,
        remote_host -> Nullable<Text>,
        status_message -> Text,
        status_updated_at -> Nullable<Timestamp>,
        started_at -> Nullable<Timestamp>,
        finished_at -> Nullable<Timestamp>,
        return_success -> Bool,
        return_code -> Nullable<Integer>,
        trigger_event_id -> Nullable<Text>,
    }
}

table! {
    Services (ServiceUUID) {
        ServiceUUID -> Text,
        Deleted -> Bool,
        MenuOrder -> Integer,
        ServiceName -> Text,
        ServiceLabel -> Text,
    }
}

table! {
    counters (name) {
        name -> Text,
        value -> Integer,
    }
}

table! {
    timestamps (name) {
        name -> Text,
        value -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(Comments, Jobs, Services, counters, timestamps,);
