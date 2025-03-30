// @generated automatically by Diesel CLI.

diesel::table! {
    training (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        hidden -> Integer,
        sets -> Integer,
        warmup_s -> Integer,
        exercise_s -> Integer,
        rest_s -> Integer,
    }
}
