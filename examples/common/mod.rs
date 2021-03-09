mod common;
pub mod ray_tracing_in_one_weekend;
pub mod ray_tracing_next_week;

pub fn init_log(level: &'static str) {
    let env = env_logger::Env::default().default_filter_or(level);
    env_logger::init_from_env(env);
}
