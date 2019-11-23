table! {
    Comments (RecordUUID) {
        RecordUUID -> Text,
        Deleted -> Bool,
        ChangesetID -> Integer,
        CommentID -> Integer,
    }
}

table! {
    FloorMaps (FloorMapUUID) {
        FloorMapUUID -> Text,
        Deleted -> Bool,
        Name -> Text,
        Description -> Text,
        ParentFloorPlanUUID -> Text,
        FloorPlanFileName -> Text,
        UpdatedAt -> Timestamp,
    }
}

table! {
    FloorPlans (FloorPlanUUID) {
        FloorPlanUUID -> Text,
        Deleted -> Bool,
        Name -> Text,
        Description -> Text,
        ParentFloorPlanUUID -> Nullable<Text>,
        FloorPlanPath -> Text,
        CreatedAt -> Timestamp,
        UpdatedAt -> Timestamp,
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
    MapObjects (MapObjectUUID) {
        MapObjectUUID -> Text,
        Deleted -> Bool,
        DeletedBy -> Nullable<Text>,
        DeletedAt -> Nullable<Timestamp>,
        Locked -> Bool,
        LockedBy -> Nullable<Text>,
        LockedAt -> Nullable<Timestamp>,
        Name -> Text,
        Description -> Text,
        ParentMapUUID -> Text,
        MapX -> Integer,
        MapY -> Integer,
        UpdatedAt -> Timestamp,
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

allow_tables_to_appear_in_same_query!(Comments, FloorMaps, FloorPlans, Jobs, MapObjects, Services,);
