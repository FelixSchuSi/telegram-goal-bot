from typing import Union, List

from asyncpraw.reddit import Comment, Submission
from telegram import Message

from alternative_angles.reddit_comment_traversal import get_links_with_texts_from_comment, \
  get_aa_comment_from_submission, get_all_replies_from_comment


async def register_comment_listener_for_goal(apis, submission: Submission, message: Message):
  print(f'goal registered: {submission.title}')
  # scrape existing comments for angles
  existing_aa_comment: Union[Comment, None] = await get_aa_comment_from_submission(submission)
  if existing_aa_comment is None:
    return
  print(f'existing_aa_comment: {existing_aa_comment.body}')
  all_children: List[Comment] = await get_all_replies_from_comment(existing_aa_comment)
  for comment in all_children:  # Is the comment a reply to the aa_comment?
    links_with_texts = await get_links_with_texts_from_comment(comment)
    for link, text in links_with_texts:
      html_text = f'<b><a href="{link}">{text}</a></b>'
      print(f'replying to this message: {message}')
      print(f'sending this html message: {html_text}')
      message.reply_html(text="html_text")

  # listen for future comments for angles
  print('[COMMENT LISTENER] Started comment listener for goal with submission_id: {submission_id}')
  async for comment in apis.subreddit.stream.comments():
    aa_comment = await is_comment_relevant(comment, submission.id)
    if aa_comment:
      print(f'[COMMENT FILTER] Comment {comment.id} passed the filter!')
      links_with_texts = await get_links_with_texts_from_comment(comment)
      for link, text in links_with_texts:
        html_text = f'<b><a href="{link}">{text}</a></b>'
        print(f'replying to this message: {message}')
        print(f'sending this html message: {html_text}')
        message.reply_html(text="html_text")


async def is_comment_relevant(comment: Comment, submission_id: str) -> Union[Comment, None]:
  submission_id_of_comment = comment.link_id
  aa_comment = await get_aa_comment_from_submission(comment.submission)
  if aa_comment is not None and submission_id_of_comment == submission_id:
    print(f'[COMMENT FILTER] aa_comment {aa_comment.id} of comment {aa_comment.id} is on watchlist')
    all_children = await get_all_replies_from_comment(aa_comment)
    if comment in all_children:  # Is the comment a reply to the aa_comment?
      return aa_comment
  return None
