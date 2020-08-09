import scrape
from telegram_wrapper import send_message, send_video
from datetime import datetime
from setup import setup
from telegram.error import BadRequest
import time

def main():
    apis = setup()
    try:
        # Use this for testing!
        # submissions = apis["subreddit"].top("week",limit=50)
        # for submission in submissions:
        #     process_submission(apis, submission)

        for submission in apis["subreddit"].stream.submissions():
            process_submission(apis, submission)
    except KeyboardInterrupt:
        print('CTRL + C detected. closing...')
        quit()
    except Exception as e:
        print('Exception in main() occured: ' + str(e))

def process_submission(apis, submission):
    passed_filter = filter(submission, apis["competition"])
    if not passed_filter: return

    def scrape_and_send(retries=1):
        if retries <= 5:
            try:
                mp4_link = scrape.mp4_link(submission.url)
                if not mp4_link: return
                send_video(apis, submission.title, mp4_link)
                print('[SUCCESS]', submission.title, mp4_link, submission.created_utc)
            except BadRequest as e:
                print('[BAD REQUEST]', submission.title, submission.url, mp4_link, str(e))
                if e.message is 'Wrong file identifier/http url specified':
                    send_message(apis, submission)
            except Exception as e:
                # In most cases the video is not processed yet. We will try again in a few secs.
                print(f'[RETRY NO {retries}]', submission.title, mp4_link, submission.created_utc, str(e))
                time.sleep(15)
                scrape_and_send(retries+1)
        else:
            print(f'[ERR AFTER {retries} RETRIES]', submission.title, mp4_link, submission.created_utc, submission.url)
            send_message(apis, submission)
            
    scrape_and_send()

def filter(submission, competition):
    title = submission.title.lower().split()
    # title must contain a hyphen AND not be a u19 or u21 game.
    if any('-' in e for e in title) and not any('u19' in e for e in title) and not any('u21' in e for e in title):
        # video must be hosted on one of the specified services.
        if any(host in submission.url for host in ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']):
            diff = datetime.utcnow() - datetime.utcfromtimestamp(submission.created_utc)
            # post must be younger than 3 minutes.
            if ((diff.total_seconds() / 60) < 3):
                # title must contain two teams of the specified competition.
                if competition.isCompetition(title):
                    return True

if __name__ == '__main__':
    while True:
        main()
        print('Praw API is down. Restarting in 3 mins...')
        time.sleep(180)
