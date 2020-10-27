from scrape import scrape_with_retries
from telegram_wrapper import send_message, send_video
from datetime import datetime
from setup import setup
import time


def main():
  apis = setup()
  try:
    # Use this for testing!
    submissions = apis["subreddit"].new()
    for submission in submissions:
      print(submission.title)
      process_submission(apis, submission)

    # for submission in apis["subreddit"].stream.submissions():
    #   process_submission(apis, submission)
  except KeyboardInterrupt:
    print('CTRL + C detected. closing...')
    quit()
  except Exception as e:
    print('Exception in main() occured: ' + str(e))


def process_submission(apis, submission):
  passed_filter = filter_submission(submission, apis["competition"])
  if not passed_filter:
    return
  mp4_link = scrape_with_retries(submission.url, submission.title)
  send_video(apis, submission.title, (submission.url, mp4_link)) if mp4_link else send_message(apis, submission.title,
                                                                                               submission.url)


def filter_submission(submission, competition):
  title = submission.title.lower().split()
  # title must contain a hyphen AND not be a u19 or u21 game.
  if any('-' in e for e in title) and not any('u19' in e for e in title) and not any('u21' in e for e in title):
    # video must be hosted on one of the specified services.
    if any(host in submission.url for host in
           ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']):
      diff = datetime.utcnow() - datetime.utcfromtimestamp(submission.created_utc)
      # post must be younger than 3 minutes.
      # if (diff.total_seconds() / 60) < 3:
        # title must contain two teams of the specified competition.
      if competition.is_competition(title):
        return True


if __name__ == '__main__':
  while True:
    main()
    print('Praw API is down. Restarting in 3 mins...')
    time.sleep(180)
