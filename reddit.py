import praw
import secrets
import telegram
import scrape

buli = ["gladbach", "mönchengladbach", "monchengladbach", "leipzig", "bayern", "münchen", "munchen", "munich", "freiburg", "hoffenheim", "dortmund", "schalke", "leverkusen", "bayer",
        "frankfurt", "wolfsburg", "union", "berlin", "hertha", "fortuna", "düsseldorf", "dusseldorf", "werder", "bremen", "augsburg", "mainz", "köln", "koln", "cologne", "paderborn"]
hosts = ["streamja", "streamable", "twitter", "imgtc",
         "clippituser", "instagram", "vimeo", "youtu"]

bot = telegram.Bot(token=secrets.telegram_token)


def main():
    reddit = praw.Reddit(
        user_agent=secrets.reddit_user_agent,
        client_id=secrets.reddit_client_id,
        client_secret=secrets.reddit_client_secret
    )

    subreddit = reddit.subreddit("soccer")
    for submission in subreddit.stream.submissions():
        process_submission(submission)

    # submissions = subreddit.search("great goal")
    # for submission in submissions:
    #     process_submission(submission)


def process_submission(submission):
    normalized_title = submission.title.lower()
    if filter(normalized_title, submission.url):
        text = '<a href="{}">{}</a>'.format(submission.url, submission.title)
        print(text)
        if(scrape.mp4Link(submission.url)):
            bot.send_video(chat_id=secrets.telegram_chat_id, caption=submission.title,
                           video=scrape.mp4Link(submission.url))
        else:
            bot.send_message(chat_id=secrets.telegram_chat_id,
                             text=text, parse_mode=telegram.ParseMode.HTML)


def filter(title, url):
    if sum(team in title for team in buli) >= 2:
        if '-' in title:
            if any(host in url for host in hosts):
                return True

if __name__ == "__main__":
    main()
