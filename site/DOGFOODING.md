# Official website dogfooding findings

## Finding

### Task
Represent the Lovable homepage, documentation, and philosophy pages as a Raymatic publication.

### Expected
The existing Raymatic content and presentation model should express a static product website without a frontend framework or new generator concepts.

### Actual
The homepage and two documentation pages were represented with Markdown front matter, raw semantic HTML, and one shared HTML/CSS presentation template. `check`, `build`, and `dev` operate on the publication.

### Workaround
The prototype's component hierarchy and theme switching were reduced to static HTML/CSS. All content is in page bodies because v0.1 exposes only `title` as structured front matter.

### Classification
Presentation limitation

### Product implication
The current model is sufficient for a static product site when pages share one shell, but page-specific composition and richer attributes require conventions inside content.

### Recommendation
Keep as-is for v0.1; investigate only if the same pressure appears in independent publications.

## Finding

### Task
Show real Raymatic commands, project structure, and front matter in the website.

### Expected
Examples should match the released CLI and generated project.

### Actual
The examples use `ray new`, `ray dev`, `ray check`, `ray build`, the `content/` and `presentation/` directories, and TOML `+++` front matter with a required `title`.

### Workaround
The visual prototype used `raymatic` and a different YAML-like example; those placeholders were changed to the actual executable and syntax.

### Classification
Website implementation issue

### Product implication
The model is explainable when the public website reflects the released behavior exactly.

### Recommendation
Keep as-is.

## Finding

### Task
Reproduce the Lovable reference's theme control while keeping the site static.

### Expected
The visual treatment should offer the same light/dark choice without introducing a JavaScript application or build pipeline.

### Actual
The original first pass was dark-only and omitted the reference control.

### Workaround
The presentation now uses an accessible checkbox/label and native CSS `:has()` variables to switch between dark and light tokens without JavaScript.

### Classification
Website implementation issue

### Product implication
The existing presentation boundary can express this interaction as static HTML/CSS; no Raymatic capability is missing.

### Recommendation
Keep as-is.
