import scrape
import telegram
from datetime import datetime
from setup import setup
from telegram.error import BadRequest
import time
from multiprocessing import Process

hosts = ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']


def main():
    apis = setup()

    try:
        # for submission in apis["subreddit"].stream.submissions():
        #     process_submission(submission, apis["bot"], apis["competition"], apis["chat_id"])

        # Use this for testing!
        submissions = apis["subreddit"].top("week",limit=50)
        for submission in submissions:
            process_submission(submission, apis["bot"], apis["competition"], apis["chat_id"])

    except KeyboardInterrupt:
        print('CTRL + C detected. closing...')
        quit()
    except Exception as e:
        print('Exception in main() occured: ' + str(e))


def process_submission(submission, bot, competition, chat_id):
    normalized_title = submission.title.lower().split()
    text = f'<a href="{submission.url}">{submission.title}</a>'
    if filter(normalized_title, submission.url, submission.created_utc, competition):
        def scrape_and_send(submission, bot, chat_id, retries=1):
            if retries <= 5:
                try:
                    mp4Link = scrape.mp4Link(submission.url)
                    if mp4Link:
                        bot.send_video(chat_id=chat_id, caption=submission.title,
                                    video=mp4Link, timeout=500)
                        print('[SUCCESS]', submission.title, mp4Link, submission.created_utc)
                    else:
                        return
                except BadRequest as e:
                    print('[BAD REQUEST]', submission.title, submission.url, mp4Link, str(e))
                    if e.message is 'Wrong file identifier/http url specified':
                        bot.send_message(chat_id=chat_id, text=text, parse_mode=telegram.ParseMode.HTML)
                except Exception as e:
                    # In most cases the video is not processed yet. We will try again in a few secs.
                    time.sleep(15)
                    print(f'[RETRY NO {retries}]', submission.title, mp4Link, submission.created_utc, str(e))
                    scrape_and_send(submission, bot, chat_id, retries+1)
            else:
                print('[ERR AFTER RETRIES]', submission.title, mp4Link, submission.created_utc, submission.url)
                bot.send_message(chat_id=chat_id, text=text, parse_mode=telegram.ParseMode.HTML)
        scrape_and_send(submission, bot, chat_id)

def filter(title, url, date, competition):
    # title must contain a hyphen AND not be a u19 game.
    if any('-' in e for e in title) and not any('u19' in e for e in title):
        # video must be hosted on one of the specified services.
        if any(host in url for host in hosts):
            diff = datetime.utcnow() - datetime.utcfromtimestamp(date)
            # post must be younger than 3 minutes.
            # if ((diff.total_seconds() / 60) < 3):
                # title must contain two teams of the specified competition.
            if competition.isCompetition(title):
                return True


if __name__ == '__main__':
    while True:
        main()
        print('Praw API is down. Restarting in 3 mins...')
        time.sleep(180)
