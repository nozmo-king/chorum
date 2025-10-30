use axum::{
    extract::{Path, State},
    response::Html,
    Form,
};
use serde::Deserialize;

use crate::{
    db::{BoardRepository, DbPool, PostRepository, ThreadRepository},
    error::{AppError, Result},
};

#[derive(Deserialize)]
pub struct NewThreadForm {
    pub title: String,
    pub body: String,
    pub user_pubkey_hex: String,
}

pub async fn show(State(pool): State<DbPool>, Path(id): Path<i64>) -> Result<Html<String>> {
    let thread = ThreadRepository::find_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;
    
    let board = BoardRepository::find_by_slug(&pool, "b").await?.unwrap(); // TODO: get actual board
    let posts = PostRepository::list_by_thread(&pool, id).await?;
    
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>{} - haich2</title>
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
        .thread-op {{ 
            border: 2px solid #2DD2C1; 
            margin: 15px 0; 
            padding: 20px;
            background-color: #636CCB;
            border-radius: 4px;
        }}
        .post {{ 
            border: 1px solid #50589C; 
            margin: 10px 0; 
            padding: 15px;
            background-color: #636CCB;
            border-radius: 4px;
        }}
        .post-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;
            padding-bottom: 5px;
            border-bottom: 1px solid #50589C;
        }}
        .post-meta {{ 
            color: #2DD2C1; 
            font-size: 0.85em; 
            opacity: 0.8;
        }}
        .post-content {{
            color: #ffffff;
            margin: 15px 0;
            white-space: pre-wrap;
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
        .nav {{ margin: 20px 0; }}
        .nav a {{ 
            margin-right: 20px; 
            padding: 8px 16px;
            background-color: #50589C;
            border-radius: 4px;
        }}
        .thread-title {{
            color: #2DD2C1;
            font-size: 1.5em;
            font-weight: bold;
            margin-bottom: 10px;
        }}
        .pow-info {{
            background-color: #50589C;
            padding: 10px;
            border-radius: 4px;
            font-size: 0.9em;
            margin: 10px 0;
        }}
        .reply-form {{
            background-color: #50589C;
            padding: 20px;
            border-radius: 4px;
            margin: 30px 0;
        }}
        .reply-form textarea {{
            width: 100%;
            height: 100px;
            background-color: #3C4267;
            color: #2DD2C1;
            border: 1px solid #636CCB;
            padding: 10px;
            border-radius: 4px;
            font-family: 'Courier New', monospace;
        }}
        .reply-form input {{
            background-color: #3C4267;
            color: #2DD2C1;
            border: 1px solid #636CCB;
            padding: 8px;
            border-radius: 4px;
            margin: 5px 0;
        }}
        .reply-btn {{
            background-color: #2DD2C1;
            color: #3C4267;
            padding: 10px 20px;
            border: none;
            border-radius: 4px;
            font-weight: bold;
            cursor: pointer;
        }}
    </style>
    <script>
        // Simple PoW miner simulation
        async function minePoW(canonicalBytes, requiredPrefix) {{
            let nonce = 0;
            while (nonce < 1000000) {{
                const input = canonicalBytes + nonce.toString(16).padStart(16, '0');
                const hash = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(input));
                const hashHex = Array.from(new Uint8Array(hash))
                    .map(b => b.toString(16).padStart(2, '0'))
                    .join('');
                
                if (hashHex.startsWith(requiredPrefix)) {{
                    return {{ nonce, hash: hashHex }};
                }}
                nonce++;
                
                if (nonce % 10000 === 0) {{
                    document.getElementById('mining-status').textContent = `Mining... tried ${{nonce}} nonces`;
                    await new Promise(resolve => setTimeout(resolve, 1));
                }}
            }}
            throw new Error('Failed to find solution');
        }}
        
        async function submitReply() {{
            const body = document.getElementById('reply-body').value;
            const pubkey = document.getElementById('user-pubkey').value;
            
            if (!body.trim() || !pubkey.trim()) {{
                alert('Please fill in all fields');
                return;
            }}
            
            document.getElementById('mining-status').textContent = 'Starting PoW mining...';
            document.getElementById('reply-btn').disabled = true;
            
            try {{
                // Begin challenge
                const beginResponse = await fetch('/api/pow/reply/begin', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{
                        client_op_id: crypto.randomUUID(),
                        post_draft: {{ 
                            body: body,
                            attachments: [],
                            refs: [],
                            title: ""
                        }},
                        user_pubkey_hex: pubkey,
                        thread_id: {thread_id},
                        parent_id: null,
                        timestamp_i64: Math.floor(Date.now() / 1000)
                    }})
                }});
                
                const beginData = await beginResponse.json();
                console.log('Begin response:', beginData);
                
                // Mine PoW
                document.getElementById('mining-status').textContent = 'Mining proof of work...';
                const solution = await minePoW(beginData.canonical_bytes, beginData.required_prefix_hex);
                
                document.getElementById('mining-status').textContent = `Found solution! Nonce: ${{solution.nonce}}`;
                
                // Commit reply
                const commitResponse = await fetch('/api/pow/reply/commit', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{
                        op_id: beginData.op_id,
                        challenge_id: beginData.challenge_id,
                        post_draft: {{ 
                            body: body,
                            attachments: [],
                            refs: [],
                            title: ""
                        }},
                        proof: {{
                            nonce_u64: solution.nonce,
                            miner_version: 1,
                            timestamp_i64: Math.floor(Date.now() / 1000)
                        }},
                        user_pubkey_hex: pubkey,
                        thread_id: {thread_id},
                        parent_id: null
                    }})
                }});
                
                const commitData = await commitResponse.json();
                console.log('Commit response:', commitData);
                
                document.getElementById('mining-status').textContent = 'Reply posted successfully!';
                setTimeout(() => location.reload(), 2000);
                
            }} catch (error) {{
                console.error('Error:', error);
                document.getElementById('mining-status').textContent = 'Error: ' + error.message;
                document.getElementById('reply-btn').disabled = false;
            }}
        }}
    </script>
</head>
<body>
    <div class="header">
        <div class="nav">
            <a href="/">Home</a>
            <a href="/boards">Boards</a>
            <a href="/boards/{board_slug}">Back to Board</a>
        </div>
    </div>
    
    <div class="thread-op">
        <div class="thread-title">{title}</div>
        <div class="post-header">
            <div class="post-meta">
                Anonymous • {} • Post #{id}
            </div>
            <div class="post-meta">
                PoW: {pow_hash}
            </div>
        </div>
        <div class="post-content">{content}</div>
        <div class="pow-info">
            <strong>Proof of Work:</strong> Hash {pow_hash} • Nonce: {pow_nonce}
        </div>
    </div>
    
    <h3>Replies ({})</h3>
    {posts}
    
    <div class="reply-form">
        <h3>Post Reply</h3>
        <p><strong>Note:</strong> Posting requires solving a proof-of-work challenge.</p>
        <div>
            <label>Public Key (secp256k1):</label><br>
            <input type="text" id="user-pubkey" placeholder="02abc123..." style="width: 400px;">
        </div>
        <div>
            <label>Reply:</label><br>
            <textarea id="reply-body" placeholder="Your reply..."></textarea>
        </div>
        <button id="reply-btn" class="reply-btn" onclick="submitReply()">Mine & Post Reply</button>
        <div id="mining-status" style="margin-top: 10px; color: #2DD2C1;"></div>
    </div>
</body>
</html>"#,
        thread.title,
        thread_id = id,
        board_slug = "b", // TODO: get actual board slug
        title = thread.title,
        id = thread.id,
        content = thread.content,
        pow_hash = thread.pow_hash.as_deref().unwrap_or("pending").chars().take(16).collect::<String>(),
        pow_nonce = thread.pow_nonce.unwrap_or(0),
        created_at = thread.created_at.format("%Y-%m-%d %H:%M"),
        reply_count = posts.len(),
        posts = if posts.is_empty() {
            "<p>No replies yet.</p>".to_string()
        } else {
            posts
                .iter()
                .map(|post| format!(
                    r#"<div class="post">
                        <div class="post-header">
                            <div class="post-meta">
                                Anonymous • {} • Post #{}
                            </div>
                            <div class="post-meta">
                                PoW: {}
                            </div>
                        </div>
                        <div class="post-content">{}</div>
                    </div>"#,
                    post.created_at.format("%Y-%m-%d %H:%M"),
                    post.id,
                    post.pow_hash.as_deref().unwrap_or("pending").chars().take(8).collect::<String>(),
                    post.content
                ))
                .collect::<Vec<_>>()
                .join("\n")
        }
    );

    Ok(Html(html))
}

pub async fn new_form(State(pool): State<DbPool>, Path(board_id): Path<i64>) -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>New Thread - haich2</title>
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
        .form-container {{
            background-color: #636CCB;
            padding: 30px;
            border-radius: 8px;
            border: 2px solid #50589C;
        }}
        .form-group {{
            margin: 20px 0;
        }}
        label {{
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
        }}
        input, textarea {{
            width: 100%;
            background-color: #3C4267;
            color: #2DD2C1;
            border: 1px solid #50589C;
            padding: 12px;
            border-radius: 4px;
            font-family: 'Courier New', monospace;
            font-size: 14px;
        }}
        input[type="text"] {{
            height: 40px;
        }}
        textarea {{
            height: 150px;
            resize: vertical;
        }}
        .submit-btn {{
            background-color: #2DD2C1;
            color: #3C4267;
            padding: 15px 30px;
            border: none;
            border-radius: 4px;
            font-weight: bold;
            cursor: pointer;
            font-size: 16px;
        }}
        .submit-btn:hover {{
            background-color: #ffffff;
        }}
        .submit-btn:disabled {{
            background-color: #50589C;
            color: #2DD2C1;
            cursor: not-allowed;
        }}
        .nav {{ margin: 20px 0; }}
        .nav a {{ 
            margin-right: 20px; 
            padding: 8px 16px;
            background-color: #50589C;
            border-radius: 4px;
            color: #2DD2C1;
            text-decoration: none;
        }}
        .pow-info {{
            background-color: #50589C;
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
        }}
        #mining-status {{
            margin-top: 15px;
            padding: 10px;
            border-radius: 4px;
            background-color: #50589C;
            display: none;
        }}
    </style>
    <script>
        async function minePoW(canonicalBytes, requiredPrefix) {{
            let nonce = 0;
            while (nonce < 1000000) {{
                const input = canonicalBytes + nonce.toString(16).padStart(16, '0');
                const hash = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(input));
                const hashHex = Array.from(new Uint8Array(hash))
                    .map(b => b.toString(16).padStart(2, '0'))
                    .join('');
                
                if (hashHex.startsWith(requiredPrefix)) {{
                    return {{ nonce, hash: hashHex }};
                }}
                nonce++;
                
                if (nonce % 10000 === 0) {{
                    document.getElementById('mining-status').textContent = `Mining... tried ${{nonce}} nonces`;
                    await new Promise(resolve => setTimeout(resolve, 1));
                }}
            }}
            throw new Error('Failed to find solution');
        }}
        
        async function submitThread() {{
            const title = document.getElementById('title').value;
            const body = document.getElementById('body').value;
            const pubkey = document.getElementById('user-pubkey').value;
            
            if (!title.trim() || !body.trim() || !pubkey.trim()) {{
                alert('Please fill in all fields');
                return;
            }}
            
            const statusEl = document.getElementById('mining-status');
            statusEl.style.display = 'block';
            statusEl.textContent = 'Starting PoW mining...';
            document.getElementById('submit-btn').disabled = true;
            
            try {{
                // Begin challenge
                const beginResponse = await fetch('/api/pow/thread/begin', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{
                        client_op_id: crypto.randomUUID(),
                        post_draft: {{ 
                            title: title,
                            body: body,
                            attachments: [],
                            refs: []
                        }},
                        user_pubkey_hex: pubkey,
                        timestamp_i64: Math.floor(Date.now() / 1000)
                    }})
                }});
                
                const beginData = await beginResponse.json();
                console.log('Begin response:', beginData);
                
                // Mine PoW
                statusEl.textContent = 'Mining proof of work... (this may take a while)';
                const solution = await minePoW(beginData.canonical_bytes, beginData.required_prefix_hex);
                
                statusEl.textContent = `Found solution! Nonce: ${{solution.nonce}} • Submitting thread...`;
                
                // Commit thread
                const commitResponse = await fetch('/api/pow/thread/commit', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{
                        op_id: beginData.op_id,
                        challenge_id: beginData.challenge_id,
                        post_draft: {{ 
                            title: title,
                            body: body,
                            attachments: [],
                            refs: []
                        }},
                        proof: {{
                            nonce_u64: solution.nonce,
                            miner_version: 1,
                            timestamp_i64: Math.floor(Date.now() / 1000)
                        }},
                        user_pubkey_hex: pubkey
                    }})
                }});
                
                const commitData = await commitResponse.json();
                console.log('Commit response:', commitData);
                
                statusEl.textContent = 'Thread created successfully! Redirecting...';
                setTimeout(() => window.location.href = `/threads/${{commitData.thread_id}}`, 2000);
                
            }} catch (error) {{
                console.error('Error:', error);
                statusEl.textContent = 'Error: ' + error.message;
                document.getElementById('submit-btn').disabled = false;
            }}
        }}
    </script>
</head>
<body>
    <div class="nav">
        <a href="/">Home</a>
        <a href="/boards">Boards</a>
        <a href="/boards/b">Back to Board</a>
    </div>
    
    <div class="form-container">
        <h2>Create New Thread</h2>
        
        <div class="pow-info">
            <strong>Proof of Work Required:</strong> Creating a thread requires solving a cryptographic puzzle. 
            This prevents spam and ensures quality posts. The mining process may take 30 seconds to several minutes.
        </div>
        
        <div class="form-group">
            <label for="user-pubkey">Public Key (secp256k1):</label>
            <input type="text" id="user-pubkey" placeholder="02abc123..." required>
            <small>Your 66-character secp256k1 public key (starts with 02 or 03)</small>
        </div>
        
        <div class="form-group">
            <label for="title">Thread Title:</label>
            <input type="text" id="title" maxlength="200" required>
        </div>
        
        <div class="form-group">
            <label for="body">Thread Content:</label>
            <textarea id="body" required></textarea>
        </div>
        
        <button id="submit-btn" class="submit-btn" onclick="submitThread()">
            Mine & Create Thread
        </button>
        
        <div id="mining-status"></div>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn create_begin(
    State(_pool): State<DbPool>,
    Path(_board_id): Path<i64>,
    Form(_form): Form<NewThreadForm>,
) -> Result<Html<String>> {
    // This endpoint is handled by the JavaScript frontend
    Ok(Html("This endpoint should be called via API".to_string()))
}