use clap::Parser;
use std::{collections::BTreeSet, path::PathBuf};

use glowfic_to_epub::Thread;

/// Board 215
/// Planecrash
pub const PLANECRASH: [&[u64]; 3] = [&MAIN, &SANDBOXES, &LECTURES];

/// Board section 703
/// Main planescrash section
pub const MAIN: [u64; 10] = [
    4582, // mad investor chaos and the woman of asmodeus
    5504, // some human relationships are less universal than others
    5506, // take this report back and bring her a better report
    5508, // project lawful and their oblivious boyfriend
    5694, // my fun research project has more existential risk than I anticipated
    5930, // what the truth can destroy
    5977, // crisis of faith
    6075, // the woman of irori
    6131, // flashback: this is not a threat
    6132, // null action
];

/// Board section 717
/// planecrash sandboxes
/// experimental doomthreads
pub const SANDBOXES: [u64; 5] = [
    5775, // totally not evil
    5778, // welcome to project lawful
    5880, // I reject your alternate reality and substitute my own
    6124, // dear abrogail
    6029, // it is a beautiful day in Cheliax and you are a horrible medianworld romance novel
];

/// Board section 721
/// planecrash lectures
pub const LECTURES: [u64; 10] = [
    5785, // to hell with science
    5826, // to earth with science
    5864, // the alien maths of dath ilan
    5310, // kissing is not a human universal [linked out from thread 4582 at reply 1721818]
    5403, // sfw tldr kissing is not a human universal [linked out from thread 4582 at reply 1721818]
    5521, // tldr some human relationships
    5610, // cheating is cuddleroom technique
    5618, // sfw tldr cheating is cuddleroom technique
    5638, // in another world we could have been trade partners
    5671, // sfw tldr we could have been trade partners
];

/// Download and process all glowfic posts in the planecrash series.
#[derive(Parser, Debug)]
struct Args {
    /// Reuse already downloaded data. Images are always cached.
    #[clap(long)]
    use_cache: bool,

    /// Reformat the author/character bits to make for easier TTS listening.
    #[clap(long)]
    for_tts: bool,
}

#[tokio::main]
async fn main() {
    let Args { use_cache, for_tts } = Args::parse();

    let mut threads = vec![];

    for id in PLANECRASH.into_iter().flatten().copied() {
        println!("Downloading post {id}");

        let thread = Thread::get_cached(id, !use_cache).await.unwrap().unwrap();

        println!("Downloaded post {id} - {}", &thread.post.subject);

        threads.push(thread);
    }

    let icons: BTreeSet<_> = threads
        .iter()
        .flat_map(|thread| thread.icons())
        .cloned()
        .collect();

    for icon in icons {
        if let Err(e) = icon.retrieve_cached(false).await {
            println!("{e:?}");
        }
    }

    for thread in threads {
        let post_id = thread.post.id;

        {
            println!("Generating html document {post_id}...");

            let path = PathBuf::from(format!("./books/html/{post_id}.html"));
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, thread.to_single_html_page(for_tts)).unwrap();
        }

        {
            println!("Generating epub document {post_id}...");

            let path = PathBuf::from(format!("./books/epub/{post_id}.epub"));
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, &thread.to_epub(for_tts).await.unwrap()).unwrap();
        }
    }

    println!("Done")
}
