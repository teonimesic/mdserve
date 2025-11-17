# Code Syntax Highlighting Tests

This document tests various programming languages with syntax highlighting.

## JavaScript

```javascript
function greet(name) {
  const message = `Hello, ${name}!`;
  console.log(message);
  return message;
}

// Call the function
const result = greet("World");
```

## Python

```python
def fibonacci(n):
    """Generate Fibonacci sequence up to n terms."""
    a, b = 0, 1
    result = []
    for _ in range(n):
        result.append(a)
        a, b = b, a + b
    return result

# Generate first 10 Fibonacci numbers
print(fibonacci(10))
```

## Rust

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

## TypeScript

```typescript
interface User {
  id: number;
  name: string;
  email: string;
}

async function fetchUser(id: number): Promise<User> {
  const response = await fetch(`/api/users/${id}`);
  return response.json();
}

const user: User = await fetchUser(123);
console.log(user.name);
```

## Bash

```bash
#!/bin/bash

# Check if directory exists
if [ -d "/tmp/test" ]; then
    echo "Directory exists"
else
    mkdir -p /tmp/test
    echo "Directory created"
fi

# Loop through files
for file in *.md; do
    echo "Processing: $file"
done
```

## Go

```go
package main

import (
    "fmt"
    "time"
)

func main() {
    messages := make(chan string)

    go func() {
        time.Sleep(1 * time.Second)
        messages <- "Hello from goroutine"
    }()

    msg := <-messages
    fmt.Println(msg)
}
```

## JSON

```json
{
  "name": "mdserve",
  "version": "0.5.1",
  "description": "Fast markdown preview server",
  "features": [
    "live reload",
    "syntax highlighting",
    "mermaid diagrams"
  ],
  "config": {
    "port": 3000,
    "theme": "dark"
  }
}
```

## SQL

```sql
SELECT
    users.name,
    COUNT(orders.id) as order_count,
    SUM(orders.total) as total_spent
FROM users
LEFT JOIN orders ON users.id = orders.user_id
WHERE users.created_at >= '2025-01-01'
GROUP BY users.id, users.name
HAVING COUNT(orders.id) > 5
ORDER BY total_spent DESC;
```

## CSS

```css
:root {
  --primary-color: #3498db;
  --secondary-color: #2ecc71;
  --font-stack: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}

.container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 2rem;
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  font-family: var(--font-stack);
}

@media (min-width: 768px) {
  .container {
    flex-direction: row;
  }
}
```

## HTML

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test Page</title>
</head>
<body>
    <header>
        <h1>Welcome</h1>
        <nav>
            <a href="#home">Home</a>
            <a href="#about">About</a>
        </nav>
    </header>
    <main>
        <p>Content goes here</p>
    </main>
</body>
</html>
```
