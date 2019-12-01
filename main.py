import secrets
import scrape
import telegram
from datetime import datetime
import setup
import sys

hosts = ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']

def main():
    try: competition = setup.competition(sys.argv[1])
    except IndexError: competition = setup.competition('buli')

    bot = setup.telegramBot()
    subreddit = setup.redditBot()

    try:
        # for submission in subreddit.stream.submissions():
        #     process_submission(submission, bot, competition)

        # Use this for testing!
        submissions = subreddit.new(limit=200)
        for submission in submissions:
            process_submission(submission, bot, competition)

    except KeyboardInterrupt:
        print('CTRL + C detected. closing...')
        quit()


def process_submission(submission, bot, competition):
    normalized_title = submission.title.lower().split()
    text = '<a href="{}">{}</a>'.format(submission.url, submission.title)
    if filter(normalized_title, submission.url, submission.created_utc, competition):
        print(text)
        try:
            mp4Link = scrape.mp4Link(submission.url)
            if mp4Link:
                bot.send_video(chat_id=secrets.telegram_chat_id, caption=submission.title,
                               video=mp4Link)
                print('Successfully scraped mp4 link. Sening video...')
            else:
                print('Couldnt scrape mp4 link. Sening link...')
                bot.send_message(chat_id=secrets.telegram_chat_id,
                                 text=text, parse_mode=telegram.ParseMode.HTML)
        except:
            bot.send_message(chat_id=secrets.telegram_chat_id,
                             text='Whoops! Something went wrong when scraping this URL: ' + submission.url)
            bot.send_message(chat_id=secrets.telegram_chat_id,
                             text=text, parse_mode=telegram.ParseMode.HTML)


def filter(title, url, date, competition):
    # title must contain two bundesliga teams.
    if competition.isCompetition(title):
        # title must contain a hyphen.
        if '-' in title:
            # video must be hosted on one of the specified services.
            if any(host in url for host in hosts):
                diff = datetime.utcnow() - datetime.utcfromtimestamp(date)
                # post must be younger than 3 minutes.
                if ((diff.total_seconds() / 60) < 3):
                    return True


if __name__ == '__main__':
    main()
