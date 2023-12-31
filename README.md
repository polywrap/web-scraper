# Web-Scraper
A wrap of the popular [scraper](https://crates.io/crates/scraper) crate.

## Integrate
### 1. Polywrap Setup
`polywrap.graphql`:
```graphql
#import * into WebScraper from "wrapscan.io/polywrap/web-scraper@1"
```

`polywrap.yaml`:
```yaml
format: 0.3.0
project:
  name: my-app
  type: app/typescript|python|rust|kotlin|swift
source:
  schema: ./polywrap.graphql
```

Codegen:
```bash
$ polywrap codegen
```

### 2. Run The Wrap

`app/typescript`:
```typescript
const webScraper = new WebScraper();

await webScraper.get_text({
  url: "...",
});
```

`app/python`:
```python
web_scraper = WebScraper()

result = web_scraper.get_text({
    "url": "..."
})
```

`app/rust`:
```rust
let web_scraper = WebScraper::new();

let result = web_scraper.get_text(
    &WebScraperArgsGetText{
        url: "..."
    }
).unwrap();
```

`app/kotlin`:
```kotlin
val webScraper = WebScraper(client)

val result = webScraper.get_text(
  WebScraperArgsGetText("...")
).getOrThrow()
```

`app/swift`:
```swift
let web_scraper = WebScraper()
try? web_scraper.get_text(
  args: WebScraperArgsGetText(url: "...")
)
```
