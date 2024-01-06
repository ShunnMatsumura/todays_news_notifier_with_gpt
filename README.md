# todays_news_notifier_with_gpt
毎日のニュースを要約して通知してSlackに通知
# 設計
```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```

# 取得対象
| サイト名 | RSSフィードのURL |
|----------|-----------------|
| Search Engine Journal | https://www.searchenginejournal.com/feed/ |
| Moz Blog | https://moz.com/blog/feed |
| CSS-Tricks | https://css-tricks.com/feed/ |
| Frontend Focus | https://frontendfoc.us/rss |
| The New Stack | https://thenewstack.io/feed/ |
| DZone Web Dev | https://dzone.com/web-development-programming-tutorials-tools-news/rss |
| GitHub Blog | https://github.blog/all.atom |
| DevOps.com | https://devops.com/feed/ |
| The Cloudflare Blog | https://blog.cloudflare.com/rss/ |
| Engineering Management Institute | https://engineeringmanagementinstitute.org/feed/ |
| LeadDev | https://leaddev.com/rss.xml |
| The Hacker News | https://feeds.feedburner.com/TheHackersNews |
| Krebs on Security | https://krebsonsecurity.com/feed/ |
