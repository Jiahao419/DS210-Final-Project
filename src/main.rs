use csv::ReaderBuilder;
use std::error::Error;

mod analysis; 
use analysis::{CombinedGame, pearson_correlation, analyze_categorical, plot_scatter};

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
