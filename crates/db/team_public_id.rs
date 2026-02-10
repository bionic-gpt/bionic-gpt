use sqids::Sqids;

const MIN_LENGTH: u8 = 6;

fn codec() -> Sqids {
    Sqids::builder()
        .min_length(MIN_LENGTH)
        .build()
        .expect("sqids configuration must be valid")
}

pub fn encode(team_id: i32) -> Option<String> {
    if team_id <= 0 {
        return None;
    }

    let encoded = codec().encode(&[team_id as u64]).ok()?;
    if encoded.is_empty() {
        return None;
    }

    Some(encoded)
}

pub fn decode(public_id: &str) -> Option<i32> {
    let decoded = codec().decode(public_id);
    let first = *decoded.first()?;
    i32::try_from(first).ok()
}
