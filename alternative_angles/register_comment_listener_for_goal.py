from typing import Union, List

from praw.reddit import Comment, Submission
from telegram import Message

from alternative_angles.reddit_comment_traversal import get_links_with_texts_from_comment, \
  get_aa_comment_from_submission, get_all_replies_from_comment


def register_comment_listener_for_goal(apis, submission: Submission, message: Message):
  # scrape existing comments for angles
  existing_aa_comment = get_aa_comment_from_submission(submission)
  all_children: List[Comment] = get_all_replies_from_comment(existing_aa_comment)
  for comment in all_children:  # Is the comment a reply to the aa_comment?
    links, texts = get_links_with_texts_from_comment(comment)
    for link, text in zip(links, texts):
      html_text = f'<b><a href="{link}">{text}</a></b>'
      message.reply_html(text=html_text)

  # listen for future comments for angles
  print('[COMMENT LISTENER] Started comment listener for goal with submission_id: {submission_id}')
  for comment in apis.subreddit.stream.comments():
    aa_comment = is_comment_relevant(comment, submission.id)
    if aa_comment:
      print(f'[COMMENT FILTER] Comment {comment.id} passed the filter!')
      links, texts = get_links_with_texts_from_comment(comment)
      for link, text in zip(links, texts):
        html_text = f'<b><a href="{link}">{text}</a></b>'
        message.reply_html(text=html_text)


def is_comment_relevant(comment: Comment, submission_id: str) -> Union[Comment, None]:
  submission_id_of_comment = comment.link_id
  aa_comment = get_aa_comment_from_submission(comment.submission)
  if aa_comment is not None and submission_id_of_comment == submission_id:
    print(f'[COMMENT FILTER] aa_comment {aa_comment.id} of comment {aa_comment.id} is on watchlist')
    all_children = get_all_replies_from_comment(aa_comment)
    if comment in all_children:  # Is the comment a reply to the aa_comment?
      return aa_comment
  return None
