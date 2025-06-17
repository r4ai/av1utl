## About this app

動画編集ソフト

## Tech Stack

- **Framework**: Tauri
- **Package Manager**: Bun
- **Linter**: Biome
- **Frontend**:
  - **Language**: TypeScript
  - **UI Library**: React
  - **Styling**: Tailwind CSS v4
  - **Routing**: TanStack Router
- **Backend**:
  - **Language**: Rust
  - **Media Processing**: GStreamer

## Development Guide

### Commands

- **Start Development Server**: `bun run tauri dev`
- **Build for Production**: `bun run build`
- **Check with Biome**: `bun run check:write`
- **Type Check**: `bun run typecheck`
- **Run Tests**: `bun run test:ci`

### Best Practices

- コミットメッセージはconventional commitに従い、英語で記述する

  ```sh
  // OK
  $ git commit -m "feat: add new feature"

  // NG
  $ git commit -m "●●の機能を追加した"
  ```

- プログラム内のコメントアウトは英語で記述する

  ```ts
  // OK
  // This function handles the click event

  // NG
  // この関数はクリックイベントを処理します
  ```

- 関数にはアロー関数を使い、return文は省略する

  ```ts
  // NG
  function handleClick() {
    return "clicked";
  }

  // OK
  const handleClick = () => "clicked";
  ```

- interfaceよりtypeを使う

  ```ts
  // NG
  interface User {
    name: string;
    age: number;
  }

  // OK
  type User = {
    name: string;
    age: number;
  };
  ```

- 必要な場合を除き、基本的にnamed exportを使用する

  ```ts
  // NG
  export default const MyComponent = () => {
    return <div>Hello World</div>;
  };

  // OK
  export const MyComponent = () => {
    return <div>Hello World</div>;
  };
  ```

- ファイル名には常に絶対kebab-caseを使用する

  - NG: `MyComponent.tsx`, `my_context.tsx`
  - OK: `my-component.tsx`, `my-context.tsx`

### Library Usage

#### TanStack Router

- ルーティングは`src/routes`ディレクトリ内で定義する

  ```tsx
  // src/routes/index.tsx
  import { createFileRoute } from "@tanstack/react-router";

  export const Route = createFileRoute("/posts")({
    component: PostsComponent,
  });
  ```

  ファイルベースルーティングの例：

  ```tsx
  src/routes/
  ├── index.tsx              ✅ /
  ├── player/                ✅ /player
  │   ├── route.tsx          👈 layout route
  │   ├── index.tsx          ✅ /player
  │   └── $id.tsx            ✅ /player/:id
  ├── -components/           👈 dirs and files with a `-` prefix is ignored
  │   ├── PlayerControls.tsx 👈 ignored
  │   └── PlayerProgress.tsx 👈 ignored
  └── _pathless/             👈 dirs and files with a `_` prefix is pathless routes
      └── settings/
          └── index.tsx      ✅ /settings
  ```

- Dynamic Routes Segments:

  ```tsx
  // src/routes/posts/$postId.tsx --> /posts/:postId
  import { createFileRoute } from "@tanstack/react-router";

  const PostComponent = () => {
    // In a component!
    const { postId } = Route.useParams();
    return <div>Post ID: {postId}</div>;
  };

  export const Route = createFileRoute("/posts/$postId")({
    // In a loader
    loader: ({ params }) => fetchPost(params.postId),
    // Or in a component
    component: PostComponent,
  });
  ```

- Layout Routes:

  ```tsx
  // src/routes/app/route.tsx
  import { Outlet, createFileRoute } from "@tanstack/react-router";

  const AppLayoutComponent = () => {
    return (
      <div>
        <h1>App Layout</h1>
        <Outlet />
      </div>
    );
  };

  export const Route = createFileRoute("/app")({
    component: AppLayoutComponent,
  });
  ```

  This `AppLayoutComponent` will render for all routes under `/app`, and the `Outlet` will render the child routes.
