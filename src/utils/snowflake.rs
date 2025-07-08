use std::sync::LazyLock;
use snowflake_me::{decompose, Snowflake};

static INSTANCE: LazyLock<Snowflake> = LazyLock::new(|| {
    Snowflake::new().expect("Failed to create Snowflake instance")
});

pub struct SnowflakeGenerator;

impl SnowflakeGenerator {
    pub fn generate() -> u64 {
        INSTANCE.next_id().expect("Generate failed")
    }

    pub fn parse(id: u64) -> (u64, u8, u8, u8) {
        let parts = decompose(id);
        (
            parts.time,
            parts.data_center_id as u8,
            parts.machine_id as u8,
            parts.sequence as u8,
        )
    }
}