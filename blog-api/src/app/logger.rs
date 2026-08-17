/*
 * 日志初始化：tracing 栈（替代原 log4rs）
 * - 输出：stdout + logs/blog-api.log（按天滚动，非阻塞写入）
 * - 过滤：环境变量 RUST_LOG 优先；默认 info,blog_api=debug,sea_orm=warn,hyper=warn
 * - 第三方 log 记录（sea-orm/sqlx/reqwest 等）经 tracing-log 桥接汇入 tracing
 */
use std::fs;
use std::sync::OnceLock;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

pub fn init_logger() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info,blog_api=debug,sea_orm=warn,sqlx=warn,hyper=warn,rustls=warn")
    });

    let _ = fs::create_dir_all("logs");
    let (file_writer, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::daily("logs", "blog-api.log"));
    let _ = LOG_GUARD.set(guard);

    if tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .with_target(true)
                .with_line_number(true)
                .compact(),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(file_writer)
                .with_ansi(false)
                .with_target(true)
                .with_line_number(true)
                .compact(),
        )
        .try_init()
        .is_err()
    {
        // 已有 subscriber（如测试/重复启动场景），保持现状
        return;
    }

    // 桥接 log crate 的第三方输出
    let _ = tracing_log::LogTracer::init();
}