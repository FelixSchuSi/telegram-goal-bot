from praw.reddit import Submission
from telegram import Message
from alternative_angles.goal_registration_handler import goal_registration_handler
from alternative_angles.reddit_comment_listener import reddit_comment_listener
from multiprocessing import Process, Queue


class AlternativeAngles:
  def __init__(self, apis):
    self.apis = apis
    self.listen_for_comments_queue = Queue()
    self.listen_for_comments_process = Process(target=reddit_comment_listener, args=(self.listen_for_comments_queue,))
    self.goal_registration_queue = Queue()
    self.goal_registration_process = Process(target=goal_registration_handler,
                                             args=(self.goal_registration_queue, self.listen_for_comments_queue,))
    self.listen_for_comments_process.start()
    self.goal_registration_process.start()

  def register_goal(self, submission: Submission, bot_message: Message) -> None:
    bot_message_id = bot_message.message_id
    self.goal_registration_queue.put((submission, bot_message_id))
    # TODO: Scrape the existing comments and post goals as reply
