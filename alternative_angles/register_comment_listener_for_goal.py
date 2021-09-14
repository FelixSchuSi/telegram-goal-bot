from typing import Union

from praw.reddit import Comment
from telegram import Message

from alternative_angles.reddit_comment_traversal import get_links_with_texts_from_comment, \
  get_aa_comment_from_submission, get_all_replies_from_comment


def register_comment_listener_for_goal(apis, submission_id: str, telegram_message_id: str):
  print('[COMMENT LISTENER] Started comment listener for goal with submission_id: {submission_id}')
  for comment in apis.subreddit.stream.comments():
    aa_comment = is_comment_relevant(comment, submission_id)
    if aa_comment:
      print(f'[COMMENT FILTER] Comment {comment.id} passed the filter!')
      links, texts = get_links_with_texts_from_comment(comment)
      goal_message = Message(telegram_message_id)
      for link, text in zip(links, texts):
        html_text = f'<b><a href="{link}">{text}</a></b>'
        goal_message.reply_html(text=html_text)


def is_comment_relevant(comment: Comment, submission_id: str) -> Union[Comment, None]:
  submission_id_of_comment = comment.link_id
  aa_comment = get_aa_comment_from_submission(comment.submission)
  if aa_comment is not None and submission_id_of_comment == submission_id:
    print(f'[COMMENT FILTER] aa_comment {aa_comment.id} of comment {aa_comment.id} is on watchlist')
    all_children = get_all_replies_from_comment(aa_comment)
    if comment in all_children:  # Is the comment a reply to the aa_comment?
      return aa_comment
  return None
