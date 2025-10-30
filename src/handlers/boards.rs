use axum::{extract::{Path, State}, response::Html};

use crate::{
    db::{BoardRepository, DbPool, ThreadRepository},
    error::{AppError, Result},
};

pub async fn list(State(pool): State<DbPool>) -> Result<Html<String>> {
    let boards = BoardRepository::list_active(&pool).await?;
    
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Boards - haich2</title>
    <style>
        body {{ 
            font-family: 'Courier New', monospace; 
            max-width: 900px; 
            margin: 0 auto; 
            padding: 20px;
            background-color: #3C4267;
            color: #2DD2C1;
            line-height: 1.5;
        }}
        .board {{ 
            border: 2px solid #50589C; 
            margin: 15px 0; 
            padding: 20px;
            background-color: #636CCB;
            border-radius: 4px;
        }}
        .board h3 {{ 
            margin: 0 0 10px 0; 
            color: #2DD2C1;
            font-size: 1.4em;
        }}
        .stats {{ 
            color: #2DD2C1; 
            font-size: 0.9em; 
            opacity: 0.8;
        }}
        a {{ 
            color: #2DD2C1; 
            text-decoration: none; 
            font-weight: bold;
        }}
        a:hover {{ 
            text-decoration: underline; 
            color: #ffffff;
        }}
        .header {{ 
            text-align: center; 
            margin-bottom: 30px; 
            border-bottom: 2px solid #50589C;
            padding-bottom: 20px;
        }}
        .header h1 {{
            color: #2DD2C1;
            font-size: 2.5em;
            margin: 0;
        }}
        .nav {{ margin: 20px 0; }}
        .nav a {{ 
            margin-right: 20px; 
            padding: 8px 16px;
            background-color: #50589C;
            border-radius: 4px;
        }}
        .description {{
            color: #ffffff;
            margin: 10px 0;
        }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Chorum</h1>
        <p>Mine Your Voice - Proof of Work Imageboard</p>
        <div class="nav">
            <a href="/">Home</a>
            <a href="/boards">Boards</a>
        </div>
    </div>
    
    <h2>Available Boards</h2>
    {}
    
    <hr style="border-color: #50589C;">
    <p><small>Each post requires solving a cryptographic puzzle</small></p>
</body>
</html>"#,
        boards
            .iter()
            .map(|board| format!(
                r#"<div class="board">
                    <h3><a href="/boards/{}">[{}] - {}</a></h3>
                    <div class="description">{}</div>
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

pub async fn show(State(pool): State<DbPool>, Path(slug): Path<String>) -> Result<Html<String>> {
    let board = BoardRepository::find_by_slug(&pool, &slug)
        .await?
        .ok_or(AppError::NotFound)?;
    
    let threads = ThreadRepository::list_by_board(&pool, board.id, 50).await?;
    
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>[{}] {} - haich2</title>
    <style>
        body {{ 
            font-family: 'Courier New', monospace; 
            max-width: 900px; 
            margin: 0 auto; 
            padding: 20px;
            background-color: #3C4267;
            color: #2DD2C1;
            line-height: 1.5;
        }}
        .thread {{ 
            border: 1px solid #50589C; 
            margin: 10px 0; 
            padding: 15px;
            background-color: #636CCB;
            border-radius: 4px;
        }}
        .thread-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;
        }}
        .thread-title {{ 
            font-weight: bold;
            color: #2DD2C1;
            font-size: 1.1em;
        }}
        .thread-meta {{ 
            color: #2DD2C1; 
            font-size: 0.85em; 
            opacity: 0.8;
        }}
        .thread-content {{
            color: #ffffff;
            margin: 10px 0;
            max-height: 100px;
            overflow: hidden;
        }}
        a {{ 
            color: #2DD2C1; 
            text-decoration: none; 
        }}
        a:hover {{ 
            text-decoration: underline; 
            color: #ffffff;
        }}
        .header {{ 
            text-align: center; 
            margin-bottom: 30px; 
            border-bottom: 2px solid #50589C;
            padding-bottom: 20px;
        }}
        .header h1 {{
            color: #2DD2C1;
            font-size: 2em;
            margin: 0;
        }}
        .nav {{ margin: 20px 0; }}
        .nav a {{ 
            margin-right: 20px; 
            padding: 8px 16px;
            background-color: #50589C;
            border-radius: 4px;
        }}
        .new-thread-btn {{
            background-color: #2DD2C1;
            color: #3C4267;
            padding: 12px 24px;
            border-radius: 4px;
            font-weight: bold;
            display: inline-block;
            margin: 20px 0;
        }}
        .pow-indicator {{
            color: #2DD2C1;
            font-size: 0.8em;
            opacity: 0.7;
        }}
        .board-header {{
            background-color: #50589C;
            padding: 15px;
            border-radius: 4px;
            margin-bottom: 20px;
        }}
        .sort-controls {{
            background-color: #50589C;
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
            display: flex;
            gap: 10px;
            align-items: center;
        }}
        .sort-btn {{
            background-color: #3C4267;
            color: #2DD2C1;
            border: 1px solid #2DD2C1;
            padding: 8px 16px;
            border-radius: 4px;
            cursor: pointer;
            font-family: 'Courier New', monospace;
        }}
        .sort-btn:hover, .sort-btn.active {{
            background-color: #2DD2C1;
            color: #3C4267;
        }}
        .thread.pow-highlight {{
            border-color: #2DD2C1;
            box-shadow: 0 0 10px rgba(45, 210, 193, 0.3);
            transform: scale(1.02);
            transition: all 0.3s ease;
        }}
        .pow-difficulty {{
            display: inline-block;
            padding: 2px 6px;
            border-radius: 3px;
            font-size: 0.8em;
            margin-left: 10px;
        }}
        .pow-easy {{ background-color: #4CAF50; color: white; }}
        .pow-medium {{ background-color: #FF9800; color: white; }}
        .pow-hard {{ background-color: #F44336; color: white; }}
    </style>
    <script>
        let currentSort = 'newest';
        let threadsData = [];
        
        function calculatePowDifficulty(hash) {{
            if (!hash || hash === 'pending') return 0;
            let leadingZeros = 0;
            for (let char of hash) {{
                if (char === '0') leadingZeros++;
                else break;
            }}
            return leadingZeros + (hash.startsWith('21e8') ? 10 : 0);
        }}
        
        function getPowDifficultyClass(difficulty) {{
            if (difficulty >= 15) return 'pow-hard';
            if (difficulty >= 8) return 'pow-medium';
            return 'pow-easy';
        }}
        
        function sortThreads(sortType) {{
            currentSort = sortType;
            document.querySelectorAll('.sort-btn').forEach(btn => btn.classList.remove('active'));
            document.querySelector(`[data-sort="${{sortType}}"]`).classList.add('active');
            
            const container = document.getElementById('threads-container');
            const threads = Array.from(container.querySelectorAll('.thread'));
            
            threads.sort((a, b) => {{
                switch(sortType) {{
                    case 'pow':
                        const diffA = calculatePowDifficulty(a.dataset.powHash);
                        const diffB = calculatePowDifficulty(b.dataset.powHash);
                        return diffB - diffA;
                    case 'replies':
                        const repliesA = parseInt(a.dataset.replies) || 0;
                        const repliesB = parseInt(b.dataset.replies) || 0;
                        return repliesB - repliesA;
                    case 'oldest':
                        return new Date(a.dataset.created) - new Date(b.dataset.created);
                    default: // newest
                        return new Date(b.dataset.created) - new Date(a.dataset.created);
                }}
            }});
            
            threads.forEach(thread => container.appendChild(thread));
        }}
        
        function highlightPowThreads() {{
            document.querySelectorAll('.thread').forEach(thread => {{
                const powHash = thread.dataset.powHash;
                const difficulty = calculatePowDifficulty(powHash);
                
                thread.addEventListener('mouseenter', () => {{
                    if (powHash && powHash !== 'pending') {{
                        thread.classList.add('pow-highlight');
                        thread.querySelector('.pow-difficulty').style.display = 'inline-block';
                    }}
                }});
                
                thread.addEventListener('mouseleave', () => {{
                    thread.classList.remove('pow-highlight');
                    thread.querySelector('.pow-difficulty').style.display = 'none';
                }});
            }});
        }}
        
        document.addEventListener('DOMContentLoaded', () => {{
            highlightPowThreads();
        }});
    </script>
</head>
<body>
    <div class="header">
        <h1>[{}] {}</h1>
        <p>{}</p>
        <div class="nav">
            <a href="/">Home</a>
            <a href="/boards">Boards</a>
            <a href="/threads/new/{}" class="new-thread-btn">New Thread</a>
        </div>
    </div>
    
    <div class="board-header">
        <strong>Board Rules:</strong> All posts require proof-of-work. Be respectful. Hover over content to see PoW difficulty.
    </div>
    
    <div class="sort-controls">
        <strong>Sort by:</strong>
        <button class="sort-btn active" data-sort="newest" onclick="sortThreads('newest')">Newest</button>
        <button class="sort-btn" data-sort="pow" onclick="sortThreads('pow')">PoW Difficulty</button>
        <button class="sort-btn" data-sort="replies" onclick="sortThreads('replies')">Replies</button>
        <button class="sort-btn" data-sort="oldest" onclick="sortThreads('oldest')">Oldest</button>
    </div>
    
    <h3>Threads ({})</h3>
    <div id="threads-container">
    {}
    </div>
</body>
</html>"#,
        board.slug,
        board.name,
        board.slug,
        board.name,
        board.description.as_deref().unwrap_or("No description"),
        board.id,
        threads.len(),
        if threads.is_empty() {
            "<p>No threads yet. Be the first to post!</p>".to_string()
        } else {
            threads
                .iter()
                .map(|thread| {
                    let pow_hash = thread.pow_hash.as_deref().unwrap_or("pending");
                    let difficulty = if pow_hash == "pending" { 0 } else {
                        pow_hash.chars().take_while(|&c| c == '0').count() + 
                        if pow_hash.starts_with("21e8") { 10 } else { 0 }
                    };
                    let difficulty_class = if difficulty >= 15 { "pow-hard" } else if difficulty >= 8 { "pow-medium" } else { "pow-easy" };
                    
                    format!(
                    r#"<div class="thread" data-pow-hash="{}" data-replies="{}" data-created="{}">
                        <div class="thread-header">
                            <div class="thread-title">
                                <a href="/threads/{}">{}</a>
                                <span class="pow-difficulty {} " style="display: none;">PoW: {}</span>
                            </div>
                            <div class="thread-meta">
                                {} replies • {}
                                <div class="pow-indicator">Hash: {}</div>
                            </div>
                        </div>
                        <div class="thread-content">
                            {}
                        </div>
                    </div>"#,
                    pow_hash,
                    thread.reply_count,
                    thread.created_at.to_rfc3339(),
                    thread.id,
                    thread.title,
                    difficulty_class,
                    difficulty,
                    thread.reply_count,
                    thread.created_at.format("%Y-%m-%d %H:%M"),
                    pow_hash.chars().take(12).collect::<String>(),
                    thread.content.chars().take(200).collect::<String>()
                )})
                .collect::<Vec<_>>()
                .join("\n")
        }
    );

    Ok(Html(html))
}