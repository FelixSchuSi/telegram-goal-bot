import time
from multiprocessing import Queue
from typing import List

from asyncpraw.reddit import Comment

from alternative_angles.reddit_comment_traversal import get_aa_comment_from_submission



def goal_registration_handler(queue: Queue, listen_for_comments_queue: Queue) -> None:
  while True:
    try:
      new_item = queue.get()
      new_submission, bot_message_id = new_item
      print(f"[QUEUE HANDLER] Received new submission in queue: {new_submission}")
      aa_comment = get_aa_comment_from_submission(new_submission)
      if aa_comment is not None and bot_message_id is not None:
        listen_for_comments_queue.put()
        watch_list.append(aa_comment.id, bot_message_id)
    except Exception as e:
      print('[QUEUE HANDLER] ', e)

    # Every time a goal is registered we remove expired entries from the watch list
    # watch_list.remove_expired_entries()
    time.sleep(1)
