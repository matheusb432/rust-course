use clipstash::{data::AppDatabase, domain::clip, web::renderer::Renderer};
use dotenv::dotenv;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite:data.db")]
    connection_string: String,
    // ? short enables this argument as `-t` and long as `--template-directory`
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_directory: PathBuf,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");

    let handle = rt.handle().clone();

    // NOTE runs a future and blocks the thread until it completes, similar to spawning a thread
    rt.block_on(async move {
        let renderer = Renderer::new(opt.template_directory);
        let database = AppDatabase::new(&opt.connection_string).await;

        let config = clipstash::RocketConfig { renderer, database };

        clipstash::rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server");
    })
}
