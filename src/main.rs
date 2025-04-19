use ani_cli_rs::{
    models::{EpisodeMeta, EpisodeStream},
    scraper::{
        fetcher::{build_http_client, fetch_episode_sources, get_episode_list, search_anime},
        parser::{parse_episode_list, parse_search_results, parse_stream_sources},
    },
};
use inquire::{Select, Text};
use std::process::{Child, Command};

/*
    Prompts user, calls approproiate functions from fetcher and parser to query
    anime api, then display parsed results from json received
*/
#[tokio::main]
async fn main() {
    let client = build_http_client();

    // Search
    let query = Text::new("Search anime:")
        .prompt()
        .expect("Search input failed");

    let raw_results = search_anime(&client, &query).await.expect("Search failed");
    let results = parse_search_results(&raw_results);
    if results.is_empty() {
        println!("No results found.");
        return;
    }

    // Anime selection
    let anime_choices: Vec<(String, usize)> = results
        .iter()
        .enumerate()
        .map(|(i, a)| {
            (
                format!(
                    "{} ({} eps) {:?}",
                    a.title, a.episode_count, a.available_translations
                ),
                i,
            )
        })
        .collect();
    let anime_labels: Vec<String> = anime_choices
        .iter()
        .map(|(label, _)| label.clone())
        .collect();

    let selected_anime_label = Select::new("Select anime:", anime_labels)
        .prompt()
        .expect("Anime selection failed");

    let (_, anime_index) = anime_choices
        .iter()
        .find(|(label, _)| label == &selected_anime_label)
        .expect("Could not find selected anime");

    let anime = &results[*anime_index];
    println!("\nSelected: {}\n", anime.title);

    // Translation type
    let translation = Select::new("Select translation:", anime.available_translations.clone())
        .prompt()
        .expect("Translation selection failed");

    // Episode input
    let episode_list = get_episode_list(&client, &anime.id)
        .await
        .expect("Failed to fetch episode list");

    let episodes = parse_episode_list(&episode_list, translation);
    if episodes.is_empty() {
        println!("No episodes available.");
        return;
    }

    // Show interactive episode menu
    let episode_number = prompt_for_episode_number(&episodes).expect("Failed to select episode");

    // Fetch streams
    println!(
        "\nFetching streams for episode {} ({:?})...",
        episode_number, translation
    );

    let stream_data = fetch_episode_sources(&client, &anime.id, &episode_number, translation)
        .await
        .expect("Failed to fetch stream sources");

    let streams = parse_stream_sources(&stream_data);
    if streams.is_empty() {
        println!("No streams found.");
        return;
    }

    // Stream provider selection
    let stream_choices: Vec<(String, usize)> = streams
        .iter()
        .enumerate()
        .map(|(i, s)| (format!("{} → {}", s.provider, s.url), i))
        .collect();
    let stream_labels: Vec<String> = stream_choices
        .iter()
        .map(|(label, _)| label.clone())
        .collect();

    let selected_stream_label = Select::new("Choose stream provider:", stream_labels)
        .prompt()
        .expect("Stream selection failed");

    let (_, stream_index) = stream_choices
        .iter()
        .find(|(label, _)| label == &selected_stream_label)
        .expect("Could not find selected stream");

    let selected_stream = &streams[*stream_index];

    // Play stream as child process
    let mut child: Child = play_stream(selected_stream, "mpv").expect("Failed to spawn player");

    // Prompt while playing
    if let Some(choice) = prompt_playback_menu() {
        if choice == "Quit" {
            let _ = child.kill();
            println!("Quitting playback.");
        } else {
            println!("Selected: {} (not yet implemented)", choice);
        }
    }
}

fn prompt_for_episode_number(episodes: &[EpisodeMeta]) -> Option<String> {
    use inquire::Select;

    const PAGE_SIZE: usize = 25;
    let mut current_page = 0;

    // Create list of "Episode 1", "Episode 2", etc.
    let mut sorted = episodes.to_vec();
    sorted.sort_by(|a, b| a.number.partial_cmp(&b.number).unwrap());

    let labels: Vec<String> = sorted
        .iter()
        .map(|ep| format!("Episode {}", ep.number))
        .collect();

    loop {
        let start = current_page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(labels.len());

        let mut page_items = labels[start..end].to_vec();
        if end < labels.len() {
            page_items.push("▶ Next page".into());
        }
        if current_page > 0 {
            page_items.insert(0, "◀ Previous page".into());
        }

        let selected = Select::new("Select episode:", page_items).prompt().ok()?;

        match selected.as_str() {
            "▶ Next page" => current_page += 1,
            "◀ Previous page" => current_page -= 1,
            label => {
                return label.strip_prefix("Episode ").map(|s| s.to_string());
            }
        }
    }
}

fn prompt_playback_menu() -> Option<String> {
    let options = vec![
        "▶ Next episode [Not implemented]",
        "◀ Previous episode [Not implemented]",
        "Replay episode [Not implemented]",
        "Choose episode [Not implemented]",
        "Quit",
    ];

    Select::new("Now playing — choose next action:", options)
        .prompt()
        .ok()
        .map(|s| s.to_string())
}

pub fn play_stream(stream: &EpisodeStream, player: &str) -> std::io::Result<Child> {
    println!("Launching {} with player {}", stream.url, player);

    Command::new(player)
        .args([
            // not sure if this works! performance varies by source website for mpv
            "--cache=yes",
            "--cache-pause",
            "--cache-pause-wait=5",
            "--demuxer-max-bytes=500M",
            "--demuxer-max-back-bytes=100M",
        ])
        .arg(&stream.url)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
}
