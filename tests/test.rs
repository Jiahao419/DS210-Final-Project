use std::io::Cursor;
use csv::ReaderBuilder;
use project::analysis::{CombinedGame, pearson_correlation, median, analyze_categorical, plot_scatter};

#[test]
fn test_deserialize_combined_game() {
    let data = "\
id,name,date,reviews,plays,playing,backlogs,wishlists,developer,genre,platform,final_rating
1001,Test Game,2020-01-01,10,200,5,30,20,\"Nintendo\",\"RPG\",\"Windows PC,PlayStation 4\",4.5
";
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(Cursor::new(data));
    let mut records = Vec::new();
    for result in rdr.deserialize::<CombinedGame>() {
        match result {
            Ok(game) => records.push(game),
            Err(e) => panic!("Failed to deserialize: {}", e),
        }
    }
    assert_eq!(records.len(), 1);
    let g = &records[0];
    assert_eq!(g.id, "1001");
    assert_eq!(g.name, "Test Game");
    assert_eq!(g.reviews, 10);
    assert_eq!(g.plays, 200);
    assert_eq!(g.final_rating, 4.5);
    assert!(g.developer.contains("Nintendo"));
    assert!(g.genre.contains("RPG"));
    assert!(g.platform.contains("Windows PC"));
}

#[test]
fn test_pearson_correlation_perfect_positive() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![2.0, 4.0, 6.0]; 
    let corr = pearson_correlation(&x, &y);
    assert!((corr - 1.0).abs() < 1e-9);
}

#[test]
fn test_pearson_correlation_perfect_negative() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![6.0, 4.0, 2.0]; 
    let corr = pearson_correlation(&x, &y);
    assert!((corr + 1.0).abs() < 1e-9);
}

#[test]
fn test_pearson_correlation_no_correlation() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![5.0, 5.0, 5.0];
    let corr = pearson_correlation(&x, &y);
    assert_eq!(corr, 0.0);
}

#[test]
fn test_median_even() {
    let vals = vec![2, 4, 6, 8];
    let m = median(&vals);
    assert_eq!(m, 5.0);
}

#[test]
fn test_median_odd() {
    let vals = vec![3, 1, 2];
    let m = median(&vals);
    assert_eq!(m, 2.0);
}

#[test]
fn test_median_empty() {
    let vals: Vec<u32> = vec![];
    let m = median(&vals);
    assert_eq!(m, 0.0);
}

#[test]
fn test_analyze_categorical_developer() {
    let games = vec![
        CombinedGame {
            id: "1".into(),
            name: "Game A".into(),
            date: "2020-01-01".into(),
            reviews: 10,
            plays: 100,
            playing: 5,
            backlogs: 10,
            wishlists: 20,
            developer: "DevA,DevB".into(),
            genre: "RPG".into(),
            platform: "Windows PC".into(),
            final_rating: 4.0,
        },
        CombinedGame {
            id: "2".into(),
            name: "Game B".into(),
            date: "2020-01-02".into(),
            reviews: 5,
            plays: 200,
            playing: 10,
            backlogs: 20,
            wishlists: 30,
            developer: "DevA".into(),
            genre: "RPG".into(),
            platform: "PlayStation 4".into(),
            final_rating: 4.5,
        },
    ];
    let filtered: Vec<&CombinedGame> = games.iter().filter(|g| g.final_rating > 0.0).collect();
    assert!(analyze_categorical("developer", &filtered, 10).is_ok());
}

#[test]
fn test_plot_scatter_empty() {
    let empty_data: Vec<&CombinedGame> = vec![];
    assert!(plot_scatter(&empty_data, |g| g.final_rating, |g| g.plays as f64,
        "test_empty.png", "Final Rating", "Plays").is_ok());
}

#[test]
fn test_plot_scatter_normal() {
    let games = vec![
        CombinedGame {
            id: "1".into(),
            name: "Game A".into(),
            date: "2020-01-01".into(),
            reviews: 10,
            plays: 100,
            playing: 5,
            backlogs: 10,
            wishlists: 20,
            developer: "DevA".into(),
            genre: "RPG".into(),
            platform: "Windows PC".into(),
            final_rating: 4.0,
        },
    ];
    let filtered: Vec<&CombinedGame> = games.iter().collect();
    assert!(plot_scatter(&filtered, |g| g.final_rating, |g| g.plays as f64,
        "test_scatter.png", "Final Rating", "Plays").is_ok());
}
