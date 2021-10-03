from typing import Union

from asyncpraw.reddit import Comment
from telegram import Message

from alternative_angles.reddit_comment_traversal import get_links_with_texts_from_comment, \
  get_aa_comment_from_submission, get_all_replies_from_comment
from alternative_angles.watch_list import WatchList
from setup import setup


def reddit_comment_listener(watch_list: WatchList) -> None:
  apis = setup()
  print('[COMMENT LISTENER] Started comment listener')
  for comment in apis.subreddit.stream.comments():
    aa_comment = is_comment_relevant(comment, watch_list)
    if aa_comment:
      print(f'[COMMENT FILTER] Comment {comment.id} passed the filter!')
      links, texts = get_links_with_texts_from_comment(comment)
      (bot_message_id, created_at) = watch_list[aa_comment.id]
      goal_message = Message(bot_message_id)
      for link, text in zip(links, texts):
        html_text = f'<b><a href="{link}">{text}</a></b>'
        goal_message.reply_html(text=html_text)


def is_comment_relevant(comment: Comment, watch_list: WatchList) -> Union[Comment, None]:
  aa_comment = get_aa_comment_from_submission(comment.submission)
  print(f'[COMMENT LISTENER] Curernt watchlist: {watch_list.watchlist}')
  if aa_comment is not None and aa_comment.id in watch_list:  # Is The Post of the comment relevant?
    print(f'[COMMENT FILTER] aa_comment {aa_comment.id} of comment {aa_comment.id} is on watchlist')
    all_children = get_all_replies_from_comment(aa_comment)
    if comment in all_children:  # Is the comment a reply to the aa_comment?
      return aa_comment
  return None
