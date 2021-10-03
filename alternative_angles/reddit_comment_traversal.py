from typing import Union, Tuple, List

from asyncpraw.models import MoreComments
from asyncpraw.reddit import Comment, Submission
from pyquery import PyQuery as pq
import itertools


async def comment_forest_to_lists(comment_forest) -> Tuple[List[Comment], List[MoreComments]]:
  more_comments = []
  comments = []
  for commentOrMoreComments in comment_forest:
    if isinstance(commentOrMoreComments, MoreComments):
      more_comments.append(commentOrMoreComments)
    elif isinstance(commentOrMoreComments, Comment):
      comments.append(commentOrMoreComments)
    else:
      print(f"What is this: {commentOrMoreComments}")
  return comments, more_comments

# obsolete?
async def parse_title(submission):
  relevant_comments = await get_existing_comments(submission)
  links = await get_links_with_texts_from_comments(relevant_comments)
  return links

# Is also absolete if parse_title is
async def get_existing_comments(submission):
  a_a_comment = await get_aa_comment_from_submission(submission)
  comments = await get_all_replies_from_comment(a_a_comment)
  return comments


async def get_all_replies_from_comment(comment_or_more_comments, temp_list=None):
  temp_list = [] if temp_list is None else temp_list
  if isinstance(comment_or_more_comments, MoreComments):
    for c in await comment_or_more_comments.comments():
      await get_all_replies_from_comment(c, temp_list)
  elif isinstance(comment_or_more_comments, Comment):
    temp_list.append(comment_or_more_comments)
    for child in await comment_or_more_comments.replies.list():
      await get_all_replies_from_comment(child, temp_list)
  else:
    print(f"What is this: {comment_or_more_comments}")
  return temp_list


async def get_aa_comment_from_submission(submission: Submission) -> Union[Comment, None]:
  comments, more_comments = await comment_forest_to_lists(await (await submission.comments()).list())

  for comment in comments:
    if comment.author == "AutoModerator":
      return comment
  return None
  # If you have trouble finding the alternative angle comment in the future,
  # you should serach in the list of MoreComments objects!
  # The a_a_comment might not even exist yet, since it is created by a bot after
  # the post is created. You might want to call this function again in a few secs.


async def get_links_with_texts_from_comments(comments):
  links_with_texts = []
  for comment in comments:
    links_with_texts.append(await get_links_with_texts_from_comment(comment))

  flat = list(itertools.chain.from_iterable(links_with_texts))
  filtered = await filter_links(flat)
  return filtered


async def filter_links(links):
  filtered = []
  for elem in links:
    link, text = elem
    if any(host in link for host in
           ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi', 'streamwo', 'streamye']):
      # Maybe also filter by text?
      filtered.append(elem)
  return filtered


async def get_links_with_texts_from_comment(comment):
  html = comment.body_html
  d = pq(html)
  links_with_texts = []
  # TODO: Refactor with list comprehension
  for link in d("a"):
    is_link = link.text is None or 'http' in link.text
    text = '' if is_link else link.text
    links_with_texts.append((link.attrib["href"], text))
  return links_with_texts
