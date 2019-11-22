import praw
import secrets
import telegram
import scrape

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
            bot.send_message(chat_id=secrets.telegram_chat_id_test,
                             text='Whoops! Something went wrong when processing reddit posts. The script is going to restart now...')
            main()
        else:
            bot.send_message(chat_id=secrets.telegram_chat_id_test,
                             text='Something is seriously wrong here. Felix should look into this!')

def process_submission(submission):
    normalized_title = submission.title.lower()
    text = '<a href="{}">{}</a>'.format(submission.url, submission.title)
    print(text)
    if filter(normalized_title, submission.url):
        try:
            mp4Link = scrape.mp4Link(submission.url)
            if mp4Link:
                bot.send_video(chat_id=secrets.telegram_chat_id_test, caption=submission.title,
                               video=mp4Link)
        except:
            bot.send_message(chat_id=secrets.telegram_chat_id_test,
                             text='Whoops! Something went wrong when scraping this URL: ' + submission.url)
            bot.send_message(chat_id=secrets.telegram_chat_id_test,
                             text=text, parse_mode=telegram.ParseMode.HTML)
    else:
        bot.send_message(chat_id=secrets.telegram_chat_id_test,
                         text=text, parse_mode=telegram.ParseMode.HTML)

def filter(title, url):
    if sum(team in title for team in buli) >= 2:
        if '-' in title:
            if any(host in url for host in hosts):
                return True

if __name__ == "__main__":
    main()
