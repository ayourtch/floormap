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
        FullText -> Text,
        ParentFloorPlanUUID -> Text,
        FloorMapFileName -> Text,
        FloorMapFileVersion -> Integer,
        Locked -> Bool,
        LockedBy -> Nullable<Text>,
        LockedAt -> Nullable<Timestamp>,
        SortOrder -> Integer,
        ClipLeft -> Integer,
        ClipTop -> Integer,
        ClipWidth -> Integer,
        ClipHeight -> Integer,
        LegendTop -> Integer,
        LegendLeft -> Integer,
        LegendFontSize -> Integer,
        UpdatedAt -> Timestamp,
    }
}

table! {
    FloorPlans (FloorPlanUUID) {
        FloorPlanUUID -> Text,
        Deleted -> Bool,
        Active -> Bool,
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
    Logs (LogUUID) {
        LogUUID -> Text,
        LogTimestamp -> Timestamp,
        Key1 -> Integer,
        Key2 -> Integer,
        Key3UUID -> Text,
        Key4UUID -> Text,
        Username -> Text,
        Source -> Text,
        Message -> Text,
        Data1 -> Text,
        Data2 -> Text,
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
        LabelSize -> Integer,
        Description -> Text,
        Meta -> Text,
        ParentMapUUID -> Text,
        TypeObjectUUID -> Nullable<Text>,
        MapX -> Integer,
        MapY -> Integer,
        ArrowX -> Integer,
        ArrowY -> Integer,
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

table! {
    Uploads (UploadUUID) {
        UploadUUID -> Text,
        RelatedToUUID -> Text,
        Deleted -> Bool,
        CreatedBy -> Text,
        CreatedAt -> Timestamp,
        UpdatedAt -> Timestamp,
        OriginalFileName -> Text,
        ServerFileName -> Text,
        ServerFileSize -> Integer,
        MimeType -> Text,
        Checksum -> Text,
        ChecksumType -> Text,
        Key1 -> Integer,
        Key2 -> Integer,
        Key3UUID -> Text,
        Key4UUID -> Text,
        Message -> Text,
        Data1 -> Text,
        Data2 -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    Comments, FloorMaps, FloorPlans, Jobs, Logs, MapObjects, Services, Uploads,
);
