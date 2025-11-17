# Mermaid Diagrams Test

This document tests various Mermaid diagram types.

## Flowchart

```mermaid
flowchart TD
    Start([Start]) --> Input[/User Input/]
    Input --> Process{Valid?}
    Process -->|Yes| Save[(Save Data)]
    Process -->|No| Error[Show Error]
    Error --> Input
    Save --> Success([Success])
```

## Sequence Diagram

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant Server
    participant Database

    User->>Browser: Click File
    Browser->>Server: GET /api/files/test.md
    Server->>Database: Query File
    Database-->>Server: Return Content
    Server-->>Browser: Send HTML
    Browser-->>User: Display Content
```

## Class Diagram

```mermaid
classDiagram
    class MarkdownState {
        +PathBuf base_dir
        +HashMap tracked_files
        +Sender ws_tx
        +reload_file()
        +rename_file()
    }

    class FileTree {
        +String name
        +String path
        +bool isFolder
        +FileNode[] children
        +render()
    }

    class Router {
        +serve_frontend()
        +api_get_files()
        +api_get_file_content()
    }

    Router --> MarkdownState : uses
    FileTree --> Router : queries
```

## State Diagram

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Loading : User Clicks File
    Loading --> Rendering : Data Received
    Loading --> Error : Request Failed
    Rendering --> Displayed : HTML Ready
    Displayed --> Loading : File Changed
    Error --> Idle : Dismiss Error
    Displayed --> [*]
```

## Gantt Chart

```mermaid
gantt
    title MDServe Development Timeline
    dateFormat  YYYY-MM-DD
    section Backend
    Initial Setup           :done,    des1, 2025-01-01, 2025-01-07
    Live Reload Feature     :done,    des2, 2025-01-08, 2025-01-15
    Image Path Rewriting    :done,    des3, 2025-01-16, 2025-01-20
    section Frontend
    React Migration         :active,  des4, 2025-01-15, 2025-01-30
    Syntax Highlighting     :         des5, 2025-01-25, 2025-02-05
    Mermaid Integration     :         des6, 2025-02-01, 2025-02-10
```

## Entity Relationship Diagram

```mermaid
erDiagram
    USER ||--o{ ORDER : places
    USER {
        int id PK
        string name
        string email
        datetime created_at
    }
    ORDER ||--|{ ORDER_ITEM : contains
    ORDER {
        int id PK
        int user_id FK
        decimal total
        datetime created_at
    }
    ORDER_ITEM }o--|| PRODUCT : references
    ORDER_ITEM {
        int id PK
        int order_id FK
        int product_id FK
        int quantity
        decimal price
    }
    PRODUCT {
        int id PK
        string name
        decimal price
        text description
    }
```

## Pie Chart

```mermaid
pie title Programming Languages Used
    "Rust" : 45
    "TypeScript" : 30
    "CSS" : 15
    "Bash" : 10
```

## Git Graph

```mermaid
gitGraph
    commit id: "Initial commit"
    commit id: "Add file tree"
    branch feature/live-reload
    checkout feature/live-reload
    commit id: "Implement WebSocket"
    commit id: "Add file watcher"
    checkout main
    merge feature/live-reload
    commit id: "Update docs"
    branch feature/react-migration
    checkout feature/react-migration
    commit id: "Setup React"
    commit id: "Migrate components"
    checkout main
    merge feature/react-migration
```
