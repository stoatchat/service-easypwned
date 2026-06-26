use clap::Parser;

use crate::download_coordinator::DownloadCoordinator;
use crate::downloader_http::DownloaderHttp;
use crate::sink::bloom::SinkBloom;
use crate::sink::stdout::SinkStdout;

pub mod download_coordinator;
pub mod downloader_http;
pub mod sink;

#[derive(Clone)]
pub struct DownloadConfig {
    pub opt: Opt,
    pub number_of_downloader: u32,
}

#[derive(Parser, Debug, Clone)]
pub struct Opt {
    #[arg(long = "sink-bloom-file")]
    sink_bloom_file: Option<String>,
    #[arg(long = "sink-stdout", default_value_t = false)]
    sink_stdout: bool,
    #[arg(long = "parallel", default_value = "60")]
    parallel: u32,
    #[arg(long = "capacity", default_value = "1010000000")]
    capacity: usize,
    #[arg(long = "fp-rate", default_value = "0.01")]
    fp_rate: f64,
}

pub async fn download(config: DownloadConfig) {
    let (sinks_jhs, sinks_senders) = {
        let mut jhs = vec![];
        let mut senders = vec![];

        if config.opt.sink_stdout {
            let (jh, sender) = SinkStdout::spawn();
            jhs.push(jh);
            senders.push(sender);
        };

        if let Some(ref _v) = config.opt.sink_bloom_file {
            let (jh, sender) = SinkBloom::spawn(config.clone());
            jhs.push(jh);
            senders.push(sender);
        };

        (jhs, senders)
    };

    if sinks_jhs.is_empty() {
        eprintln!("you need to define a sink, try --sink_stdout");
        return;
    }

    let (_coordinator_jh, coordinator) = DownloadCoordinator::spawn(sinks_senders);

    for _i in 0..config.number_of_downloader {
        DownloaderHttp::spawn(coordinator.clone());
    }

    for jh in sinks_jhs {
        jh.await.expect("sink crashed");
        eprintln!("finish sink")
    }
}

#[tokio::main]
async fn main() -> ::anyhow::Result<(), ::anyhow::Error> {
    let opt: Opt = Opt::parse();

    let download_config = DownloadConfig {
        number_of_downloader: opt.parallel,
        opt,
    };

    download(download_config).await;

    return Ok(());
}
