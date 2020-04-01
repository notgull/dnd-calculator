table! {
    actiontakers (id) {
        id -> Int4,
        room_id -> Nullable<Int4>,
        name -> Varchar,
        x -> Float8,
        y -> Float8,
        armor_class -> Int4,
        health -> Int4,
        dead -> Bool,
    }
}

table! {
    items (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
    }
}

table! {
    itemscarried (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        template_id -> Nullable<Int4>,
    }
}

table! {
    moves (id) {
        id -> Int4,
        name -> Text,
        hit_type -> Int4,
        hit_radius -> Nullable<Int4>,
        dice_count -> Int4,
        dice_type -> Int4,
        dice_modifier -> Int4,
        stat_boost -> Int4,
        saving_throw -> Nullable<Int4>,
        effect -> Nullable<Int4>,
        effect_severity -> Nullable<Int4>,
    }
}

table! {
    moveusage (id) {
        id -> Int4,
        move_id -> Nullable<Int4>,
        template_id -> Nullable<Int4>,
    }
}

table! {
    room (id) {
        id -> Int4,
    }
}

table! {
    templates (id) {
        id -> Int4,
        name -> Text,
        health -> Int4,
        armor_class -> Int4,
        description -> Text,
    }
}

joinable!(actiontakers -> room (room_id));
joinable!(itemscarried -> items (item_id));
joinable!(itemscarried -> templates (template_id));
joinable!(moveusage -> moves (move_id));
joinable!(moveusage -> templates (template_id));

allow_tables_to_appear_in_same_query!(
    actiontakers,
    items,
    itemscarried,
    moves,
    moveusage,
    room,
    templates,
);
