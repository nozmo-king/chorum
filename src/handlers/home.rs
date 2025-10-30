use axum::extract::State;
use axum::response::Html;

use crate::{
    db::{BoardRepository, DbPool},
    error::Result,
};

pub async fn index(State(pool): State<DbPool>) -> Result<Html<String>> {
    let boards = BoardRepository::list_active(&pool).await?;
    
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Chorum - Proof of Work Imageboard</title>
    <style>
        body {{ font-family: sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }}
        .board {{ border: 1px solid #ccc; margin: 10px 0; padding: 15px; }}
        .board h3 {{ margin: 0 0 10px 0; }}
        .stats {{ color: #666; font-size: 0.9em; }}
        a {{ color: #0066cc; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        .header {{ text-align: center; margin-bottom: 30px; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Chorum</h1>
        <p>Proof of Work Imageboard - Mine Your Voice</p>
    </div>
    
    <h2>Boards</h2>
    {}
    
    <hr>
    <p><small>Powered by Rust + Axum + SQLite</small></p>
</body>
</html>"#,
        boards
            .iter()
            .map(|board| format!(
                r#"<div class="board">
                    <h3><a href="/boards/{}">[{}] - {}</a></h3>
                    <p>{}</p>
                    <div class="stats">
                        {} threads, {} posts
                    </div>
                </div>"#,
                board.slug,
                board.slug,
                board.name,
                board.description.as_deref().unwrap_or("No description"),
                board.thread_count,
                board.post_count
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(Html(html))
}