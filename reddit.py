import praw
import secrets
import telegram

buli = ["gladbach", "mönchengladbach", "monchengladbach", "leipzig", "bayern", "münchen", "munchen", "munich", "freiburg", "hoffenheim", "dortmund", "schalke", "leverkusen", "bayer",
        "frankfurt", "wolfsburg", "union", "berlin", "hertha", "fortuna", "düsseldorf", "dusseldorf", "werder", "bremen", "augsburg", "mainz", "köln", "koln", "cologne", "paderborn"]
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
    if filter(normalized_title) >= 3:
        text = '<a href="{}">{}</a>'.format(submission.url, submission.title)
        print(text)
        bot.send_message(chat_id=secrets.telegram_chat_id,
                         text=text, parse_mode=telegram.ParseMode.HTML)


def filter(title):
    count = 0
    for team in buli:
        if team in title:
            count = count+1
    if '-' in title:
        count = count+1
    return count


def send_video(caption, url):
    # TODO: crawl streamja/streamable links and extract direct link to the mp4 file. Then do this:
    # bot.send_video(chat_id=secrets.telegram_chat_id, caption="guck dir das Tor an!",
    # video="https://tiger.cdnja.co/v/g3/G3oe.mp4?secure=QYHStbyRfxUYqnfp4u0kNw&expires=1574289000")
    return


if __name__ == "__main__":
    main()
