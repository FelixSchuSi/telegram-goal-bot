import praw
import secrets
import telegram
import scrape
from datetime import datetime
from datetime import timedelta

buli = ["gladbach", "mönchengladbach", "monchengladbach", "leipzig", "bayern", "münchen", "munchen", "munich", "freiburg", "hoffenheim", "dortmund", "schalke", "leverkusen", "bayer",
        "frankfurt", "wolfsburg", "union", "berlin", "hertha", "fortuna", "düsseldorf", "dusseldorf", "werder", "bremen", "augsburg", "mainz", "köln", "koln", "cologne", "paderborn"]
hosts = ["streamja", "streamable", "twitter", "imgtc",
         "clippituser", "instagram", "vimeo", "youtu"]
bot = telegram.Bot(token=secrets.telegram_token)
restartcount = 0


def main():
    try:
        reddit = praw.Reddit(
            user_agent=secrets.reddit_user_agent,
            client_id=secrets.reddit_client_id,
            client_secret=secrets.reddit_client_secret
        )

        subreddit = reddit.subreddit("soccer")
        for submission in subreddit.stream.submissions():
            process_submission(submission)

        # Use this for testing!
        # submissions = subreddit.search("great goal")
        # for submission in submissions:
        #     process_submission(submission)

    except:
        global restartcount
        if restartcount < 2:
            restartcount += 1
            print('crashed. restarting...')
            main()
        else:
            print('crashed third time.')


def process_submission(submission):
    normalized_title = submission.title.lower()
    text = '<a href="{}">{}</a>'.format(submission.url, submission.title)
    if filter(normalized_title, submission.url, submission.created_utc):
        print(text)
        try:
            mp4Link = scrape.mp4Link(submission.url)
            if mp4Link:
                print('Successfully scraped mp4 link. Sening video...')
                bot.send_video(chat_id=secrets.telegram_chat_id, caption=submission.title,
                               video=mp4Link)
            else:
                print('Couldnt scrape mp4 link. Sening link...')
                bot.send_message(chat_id=secrets.telegram_chat_id,
                                 text=text, parse_mode=telegram.ParseMode.HTML)
        except:
            bot.send_message(chat_id=secrets.telegram_chat_id,
                             text='Whoops! Something went wrong when scraping this URL: ' + submission.url)
            bot.send_message(chat_id=secrets.telegram_chat_id,
                             text=text, parse_mode=telegram.ParseMode.HTML)


def filter(title, url, date):
    # title must contain two bundesliga teams.
    if sum(team in title for team in buli) >= 2:
        # title must contain a hyphen.
        if '-' in title:
            # video must be hosted on one of the specified services.
            if any(host in url for host in hosts):
                diff = datetime.utcnow() - datetime.utcfromtimestamp(date)
                # post must be younger than 10 minutes.
                if ((diff.total_seconds() / 60) < 10):
                    return True


if __name__ == "__main__":
    main()
