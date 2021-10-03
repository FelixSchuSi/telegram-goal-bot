from telegram import Message

from alternative_angles.register_comment_listener_for_goal import register_comment_listener_for_goal
from scrape import scrape_with_retries
from telegram_wrapper import send_message, send_video, get_copy_of_message_in_comment_group
from datetime import datetime
from setup import setup
import time
import asyncio


async def main():
  apis = await setup()
  try:
    # Use this for testing!
    submissions = apis["subreddit"].new(limit=20)
    async for submission in submissions:
      await process_submission(apis, submission)

    print(f'done but processing')
    await asyncio.sleep(1000)
    # async for submission in apis["subreddit"].stream.submissions():
    #   process_submission(apis, submission)
  except KeyboardInterrupt:
    print('CTRL + C detected. closing...')
    quit()
  except Exception as e:
    print('Exception in main() occured: ' + str(e))


async def process_submission(apis, submission):
  passed_filter = await filter_submission(submission, apis["competition"])
  if not passed_filter:
    return
  mp4_link = scrape_with_retries(submission.url, submission.title)

  message: Message

  if mp4_link:
    message = send_video(apis, submission.title, (submission.url, mp4_link))
  else:
    message = send_message(apis, submission.title, submission.url)

  print(1)
  get_copy_of_message_in_comment_group(apis, message)
  print(2)
  # loop.create_task(register_comment_listener_for_goal(apis, submission, message))


async def filter_submission(submission, competition):
  title = submission.title.lower().split()
  # title must contain a hyphen AND not be a u19 or u21 game.
  if (any('-' in e for e in title) and not any('u19' in e for e in title) and not any('u21' in e for e in title) and not any('w' in e for e in title)) or len(competition.teams) == 0:
    # video must be hosted on one of the specified services.
    if any(host in submission.url for host in
           ['streamwo', 'streamja', 'streamye', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']):
      diff = datetime.utcnow() - datetime.utcfromtimestamp(submission.created_utc)
      # post must be younger than 3 minutes.
      # if (diff.total_seconds() / 60) < 3:
      # title must contain two teams of the specified competition.
      if competition.is_competition(title):
        return True


if __name__ == '__main__':
  while True:
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    loop.close()
    print('Praw API is down. Restarting in 3 mins...')
    time.sleep(180)
