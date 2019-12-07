import scrape
import telegram
import asyncio
from datetime import datetime
from bcolors import bcolors
import setup
import sys
from urllib.error import HTTPError
from traceback import print_exception
from telegram.error import BadRequest

hosts = ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']


async def main():
    try:
        competition = setup.competition(sys.argv[1])
        chat_id = setup.chat_id(sys.argv[1])
    except IndexError:
        competition = setup.competition('buli')
        chat_id = setup.chat_id('buli')

    bot = setup.telegramBot()
    subreddit = setup.redditBot()

    try:
        for submission in subreddit.stream.submissions():
            await process_submission(submission, bot, competition, chat_id)

        # Use this for testing!
        # submissions = subreddit.new(limit=1000)
        # for submission in submissions:
        #     await process_submission(submission, bot, competition, chat_id)

    except KeyboardInterrupt:
        print(bcolors.FAIL + 'CTRL + C detected. closing...' + bcolors.ENDC)
        quit()
    except Exception as e:
        print(bcolors.FAIL + 'Exception in main() occured: ' + str(e) + bcolors.ENDC)


async def process_submission(submission, bot, competition, chat_id):
    normalized_title = submission.title.lower().split()
    text = '<a href="{}">{}</a>'.format(submission.url, submission.title)
    if filter(normalized_title, submission.url, submission.created_utc, competition):
        print(text)
        try:
            mp4Link = await scrape.mp4Link(submission.url)
            if mp4Link:
                print(mp4Link)
                # TODO: Improve Error handling. When scraping dead links nothin should happen. Example of dead link: https://streamja.com/da6n
                bot.send_video(chat_id=chat_id, caption=submission.title,
                               video=mp4Link, timeout=240)
                print('Successfully scraped mp4 link. Sening video...')
            else:
                print(bcolors.WARNING +
                      'Couldnt scrape mp4 link. Sening link...' + bcolors.ENDC)
                bot.send_message(chat_id=chat_id,
                                 text=text, parse_mode=telegram.ParseMode.HTML)
        except BadRequest as e:
            print(bcolors.WARNING + 'tried to process dead link.' + bcolors.ENDC)
        except Exception as e:
            print(bcolors.FAIL + 'This URL ' + submission.url + ' caused an Exception in process_submission(): ' + str(e) + bcolors.ENDC)
            # bot.send_message(chat_id=chat_id,
            #                  text='Whoops! Something went wrong when scraping this URL: ' + submission.url)
            bot.send_message(chat_id=chat_id,
                             text=text, parse_mode=telegram.ParseMode.HTML)


def filter(title, url, date, competition):
    # title must contain a hyphen.
    if any('-' in e for e in title):
        # video must be hosted on one of the specified services.
        if any(host in url for host in hosts):
            diff = datetime.utcnow() - datetime.utcfromtimestamp(date)
            # post must be younger than 3 minutes.
            if ((diff.total_seconds() / 60) < 3):
                # title must contain two bundesliga teams.
                if competition.isCompetition(title):
                    return True


if __name__ == '__main__':
    asyncio.run(main())
