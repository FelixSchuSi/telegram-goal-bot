import scrape
import telegram
from datetime import datetime
from setup import setup
import sys
from urllib.error import HTTPError
from traceback import print_exception
from telegram.error import BadRequest
import time
from multiprocessing import Process

hosts = ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']


def main():
    try:
        setupObject = setup(sys.argv[1])
    except IndexError:
        setupObject = setup('buli')

    try:
        # for submission in setupObject.subreddit.stream.submissions():
        #     Process(target=process_submission, args=(submission, setupObject.bot, setupObject.competition, setupObject.chat_id)).start()

        # Use this for testing!
        submissions = setupObject.subreddit.new(limit=1000)
        for submission in submissions:
            Process(target=process_submission, args=(submission, setupObject.bot, setupObject.competition, setupObject.chat_id)).start()

    except KeyboardInterrupt:
        print('CTRL + C detected. closing...')
        quit()
    except Exception as e:
        print('Exception in main() occured: ' + str(e))


def process_submission(submission, bot, competition, chat_id):
    normalized_title = submission.title.lower().split()
    text = '<a href="{}">{}</a>'.format(submission.url, submission.title)
    if filter(normalized_title, submission.url, submission.created_utc, competition):
        print(text)
        try:
            mp4Link = scrape.mp4Link(submission.url)
            if mp4Link:
                print(mp4Link)
                # TODO: Improve Error handling. When scraping dead links nothin should happen. Example of dead link: https://streamja.com/da6n
                bot.send_video(chat_id=chat_id, caption=submission.title,
                               video=mp4Link, timeout=500)
                print('Successfully scraped mp4 link. Sening video...')
            else:
                print('Couldnt scrape mp4 link. Sening link...')
                bot.send_message(chat_id=chat_id,
                                 text=text, parse_mode=telegram.ParseMode.HTML)
        except BadRequest as e:
            print('Tried to process dead link (BadRequest): ' + str(e))
        except Exception as e:
            print('This URL ' + submission.url + ' caused an Exception in process_submission(): ' + str(e))
            # bot.send_message(chat_id=chat_id,
            #                  text='Whoops! Something went wrong when scraping this URL: ' + submission.url)
            bot.send_message(chat_id=chat_id,
                             text=text, parse_mode=telegram.ParseMode.HTML)


def filter(title, url, date, competition):
    # title must contain a hyphen AND not be a u19 game.
    if any('-' in e for e in title) and not any('u19' in e for e in title):
        # video must be hosted on one of the specified services.
        if any(host in url for host in hosts):
            diff = datetime.utcnow() - datetime.utcfromtimestamp(date)
            # post must be younger than 3 minutes.
            # if ((diff.total_seconds() / 60) < 3):
            # title must contain two bundesliga teams.
            if competition.isCompetition(title):
                return True


if __name__ == '__main__':
    while True:
        main()
        print('Script crashed due to praw Error. Restarting in 3 mins...')
        time.sleep(180)