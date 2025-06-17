# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**av1utl** is a video editing utility built with Tauri (Rust backend) and React (TypeScript frontend). The application uses GStreamer with GStreamer Editing Services (GES) for professional video processing and provides real-time video preview with timeline editing capabilities.

## Development Commands

### Primary Development
- `bun run tauri dev` - Start development server (preferred)
- `bun run dev` - Start frontend development server
- `bun run build` - Build for production
- `bun run check:write` - Run Biome linter/formatter with auto-fixes
- `bun run typecheck` - Run TypeScript type checking
- `bun run test:ci` - Run tests

### Tauri Operations
- `bun run tauri dev` - Run Tauri development mode
- `bun run tauri build` - Build Tauri application

## Architecture

### Command-Based Frontend-Backend Communication
The app uses a command pattern for React-Rust communication:
- Commands are defined in `src/lib/commands.ts` as TypeScript wrappers around Tauri's `invoke()`
- Backend processes commands via `mpsc` channels in dedicated threads
- Key commands: `AddClip`, `StartPreview`, `StopPreview`, `SeekTo`, `PlayPause`

### Video Processing Pipeline
- **GStreamer Editing Services (GES)** handles timeline-based video editing
- **AppSink** captures frames as RGBA byte arrays
- Frames streamed to frontend via Tauri events (`new-frame`)
- **HTML5 Canvas** renders video frames in real-time

### Time Handling
- Custom time utilities in `src/lib/time.ts` with type-safe units (`ns`, `s`)
- All internal timing uses nanosecond precision
- Conversion methods prevent unit confusion

## Key File Locations

### Frontend (React/TypeScript)
- `src/routes/` - TanStack Router pages with auto-generated route tree
- `src/components/video-preview.tsx` - Canvas-based video rendering
- `src/components/timeline-scrubber.tsx` - Timeline seeking control
- `src/lib/commands.ts` - Tauri command wrappers

### Backend (Rust)
- `src-tauri/src/lib.rs` - Main application logic and command handlers
- `src-tauri/examples/` - GStreamer pipeline examples

### Configuration
- Uses **Bun** as package manager (not npm/yarn)
- **Biome** for linting/formatting (not ESLint/Prettier)
- **Lefthook** for Git hooks
- **Tailwind CSS v4** for styling

## Coding Standards & Best Practices

### Code Style
- **Commit messages**: Use conventional commits in English
  - ✅ `git commit -m "feat: add new feature"`
  - ❌ `git commit -m "●●の機能を追加した"`
- **Comments**: Write code comments in English
- **Functions**: Use arrow functions with implicit returns when possible
  - ✅ `const handleClick = () => "clicked"`
  - ❌ `function handleClick() { return "clicked"; }`
- **Types**: Prefer `type` over `interface`
  - ✅ `type User = { name: string; age: number; }`
  - ❌ `interface User { name: string; age: number; }`
- **Exports**: Use named exports by default (avoid default exports unless necessary)
- **File naming**: Use kebab-case for all filenames
  - ✅ `my-component.tsx`, `my-context.tsx`
  - ❌ `MyComponent.tsx`, `my_context.tsx`

### TanStack Router Patterns
- Routes defined in `src/routes/` directory using file-based routing
- Dynamic routes: `$id.tsx` → `/:id`
- Layout routes: `route.tsx` files provide layout with `<Outlet />`
- Ignored files: prefix with `-` (e.g., `-components/`)
- Pathless routes: prefix with `_` (e.g., `_pathless/`)
- Access params with `Route.useParams()` in components
- Use loaders for data fetching: `loader: ({ params }) => fetchData(params.id)`

## Development Notes

### Tech Stack Specifics
- Frontend: React 18 + TypeScript + TanStack Router + Tailwind v4
- Backend: Rust + Tauri v2 + GStreamer + GES
- Build: Vite 6 with Bun as runtime

### Video Processing Context
This is a professional video editing application, not a simple media player. When working with video-related code, consider:
- Multi-layer timeline editing capabilities
- Professional GStreamer pipeline architecture
- Real-time frame streaming and Canvas rendering
- Timeline seeking and scrubbing functionality

### Code Patterns
- Event-driven architecture for video frame updates
- Type-safe command dispatching between frontend/backend
- Thread-safe communication patterns for video processing
- Professional video editing concepts (layers, priorities, timelines)