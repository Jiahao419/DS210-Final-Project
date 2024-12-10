use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::collections::HashMap;
use plotters::prelude::*;
use statrs::statistics::Statistics;

#[derive(Debug, Deserialize)]
struct CombinedGame {
    id: String,
    name: String,
    date: String,
    reviews: u32,
    plays: u32,
    playing: u32,
    backlogs: u32,
    wishlists: u32,
    developer: String,
    genre: String,
    platform: String,
    final_rating: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path("games_combined_cleaned.csv")?;
    let mut combined_data = Vec::new();

    for result in rdr.deserialize::<CombinedGame>() {
        match result {
            Ok(game) => combined_data.push(game),
            Err(e) => eprintln!("{}", e),
        }
    }

    println!("Loaded {} combined game records.", combined_data.len());

    let filtered_data: Vec<_> = combined_data.iter().filter(|g| g.final_rating > 0.0).collect();

    let plays_vec: Vec<f64> = filtered_data.iter().map(|g| g.plays as f64).collect();
    let final_rating_vec: Vec<f64> = filtered_data.iter().map(|g| g.final_rating).collect();
    let wishlist_vec: Vec<f64> = filtered_data.iter().map(|g| g.wishlists as f64).collect();
    let reviews_vec: Vec<f64> = filtered_data.iter().map(|g| g.reviews as f64).collect();

    let corr_final_rating = pearson_correlation(&plays_vec, &final_rating_vec);
    let corr_wishlist = pearson_correlation(&plays_vec, &wishlist_vec);
    let corr_reviews = pearson_correlation(&plays_vec, &reviews_vec);

    println!("--------------------------- Correlations with plays (only games with final_rating>0) -------------------------------");
    println!("Correlation(plays, final_rating): {:.4}", corr_final_rating);
    println!("Correlation(plays, wishlists): {:.4}", corr_wishlist);
    println!("Correlation(plays, reviews): {:.4}", corr_reviews);
    println!("-----------------------------------------------------------------------------------------------------------------------");

    analyze_categorical("developer", &filtered_data, 10)?;
    analyze_categorical("genre", &filtered_data, 10)?;
    analyze_categorical("platform", &filtered_data, 10)?;

    plot_scatter(&filtered_data, |g| g.final_rating, |g| g.plays as f64, 
                 "final_rating_vs_plays.png", "Final Rating", "Plays")?;

    plot_scatter(&filtered_data, |g| g.wishlists as f64, |g| g.plays as f64, 
                 "wishlists_vs_plays.png", "Wishlists", "Plays")?;

    Ok(())
}

fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }
    let mean_x = x.mean();
    let mean_y = y.mean();
    let mut num = 0.0;
    let mut denom_x = 0.0;
    let mut denom_y = 0.0;
    for i in 0..x.len() {
        let dx = x[i] - mean_x;
        let dy = y[i] - mean_y;
        num += dx * dy;
        denom_x += dx * dx;
        denom_y += dy * dy;
    }
    let denom = (denom_x * denom_y).sqrt();
    if denom == 0.0 {
        0.0
    } else {
        num / denom
    }
}

fn analyze_categorical(field_name: &str, games: &[&CombinedGame], top_n: usize) -> Result<(), Box<dyn Error>> {
    let mut map: HashMap<String, Vec<u32>> = HashMap::new();
    for g in games {
        let field_values = match field_name {
            "developer" => &g.developer,
            "genre" => &g.genre,
            "platform" => &g.platform,
            _ => &g.developer,
        };

        let vals = field_values.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        for val in vals {
            map.entry(val.to_string()).or_default().push(g.plays);
        }
    }

    let mut stats: Vec<(String, f64, f64)> = map.into_iter().map(|(k, vals)| {
        let mean = vals.iter().map(|&v| v as f64).sum::<f64>() / vals.len() as f64;
        let med = median(&vals);
        (k, mean, med)
    }).collect();

    stats.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("-------------------- Top {} {} by average plays (final_rating>0) ---------------------------------", top_n, field_name);
    for (i, (k, mean, median)) in stats.iter().take(top_n).enumerate() {
        println!("{}: {} -> mean: {:.2}, median: {:.2}", i+1, k, mean, median);
    }
    println!("----------------------------------------------------------------------------------------------");

    Ok(())
}


fn median(vals: &[u32]) -> f64 {
    if vals.is_empty() {
        return 0.0;
    }
    let mut sorted = vals.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len()/2;
    if sorted.len()%2==0 {
        (sorted[mid-1]+sorted[mid]) as f64 / 2.0
    } else {
        sorted[mid] as f64
    }
}

fn plot_scatter<Fx, Fy>(
    games: &[&CombinedGame],
    x_fn: Fx,
    y_fn: Fy,
    filename: &str,
    x_label: &str,
    y_label: &str,
) -> Result<(), Box<dyn Error>>
where
    Fx: Fn(&CombinedGame) -> f64,
    Fy: Fn(&CombinedGame) -> f64,
{
    if games.is_empty() {
        return Ok(());
    }

    let xs: Vec<f64> = games.iter().map(|g| x_fn(*g)).collect();
    let ys: Vec<f64> = games.iter().map(|g| y_fn(*g)).collect();

    let min_x = xs.iter().cloned().fold(f64::MAX, f64::min);
    let max_x = xs.iter().cloned().fold(f64::MIN, f64::max);
    let min_y = ys.iter().cloned().fold(f64::MAX, f64::min);
    let max_y = ys.iter().cloned().fold(f64::MIN, f64::max);

    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} vs {}", x_label, y_label), ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()?;

    chart.draw_series(
        xs.iter().zip(ys.iter()).map(|(&x, &y)| {
            Circle::new((x, y), 3, BLUE.filled())
        })
    )?;

    println!("Scatter plot of {} vs {} saved to {}", x_label, y_label, filename);
    Ok(())
}
