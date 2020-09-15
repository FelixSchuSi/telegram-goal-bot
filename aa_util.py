from praw import models
from pyquery import PyQuery as pq
import itertools
from multiprocessing import Process
from datetime import datetime
from aa_watchlist import WatchList
import time

# {aa_comment_id: [("telegram_user_id_1", created_at), ("telegram_user_id_2", created_at)]}


watch_list = WatchList();


def comment_listener(apis):
  print('[COMMENT LISTENER] Started comment listener')
  for comment in apis["subreddit"].stream.comments():
    aa_comment = get_aa_comment_from_submission(comment.submission)
    print('[COMMENT LISTENER]', aa_comment.id) if aa_comment is not None else print('no aa_comment found')
    if aa_comment is not None:
      # Aufrufen von get_aa_comment_from_submission und None behanlung an neue Funktion is_comment_childof_aa_comment auslagern
      is_comment_childof_aa_comment()
      if aa_comment.id in watch_list:
        # TODO: check if comment is child of aa_comment then parse and send


def queue_handler(queue, apis):
  print('[QUEUE HANDLER] Started queue handler')
  listen_for_comments_process = Process(target=comment_listener, args=(apis,))
  listen_for_comments_process.start()
  while True:
    try:
      new_item = queue.get()
      new_submission, telegram_user_id = new_item
      print(f"[QUEUE HANDLER] Received new submission in queue: {new_submission}")
      aa_comment = get_aa_comment_from_submission(new_submission)
      if aa_comment is not None and telegram_user_id is not None:
        watch_list.append(aa_comment.id, telegram_user_id)
    except Exception as e:
      print('[QUEUE HANDLER] ', e)

    diff = datetime.utcnow() - watch_list.last_expiration_check
    if (diff.total_seconds() / 60 / 60) >= 1:
      # Every hour we remove expired entries from the watch list
      print('[QUEUE HANDLER] Initiated removal of expired entries in watchlist')
      watch_list.remove_expired_entries()
    time.sleep(1)


def find_submission_by_title(title, apis):
  search_results = list(apis["subreddit"].search(title))
  # For now, we return the first search result.
  return search_results[0]


def filter_links(links):
  filtered = []
  for elem in links:
    link, text = elem
    if any(host in link for host in ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']):
      # Maybe also filter by text?
      filtered.append(elem)
  return filtered


def get_links_from_comments(comments):
  links = []
  for comment in comments:
    links.append(get_links_from_comment(comment))

  flat = list(itertools.chain.from_iterable(links))
  filtered = filter_links(flat)
  return filtered


def get_links_from_comment(comment):
  html = comment.body_html
  d = pq(html)
  links = []
  # TODO: Refactor with list comprehension
  for link in d("a"):
    is_link = link.text is None or 'http' in link.text
    text = '' if is_link else link.text
    links.append((link.attrib["href"], text))
  return links


def parse_title(title, apis):
  submission = find_submission_by_title(title, apis)
  relevant_comments = get_existing_comments(submission)
  links = get_links_from_comments(relevant_comments)
  # scraped_links = list(map(lambda x: (mp4_link(x[0]), x[1]), links))
  return links, submission


def get_existing_comments(submission):
  a_a_comment = get_aa_comment_from_submission(submission)
  replies = a_a_comment.replies.list()
  comments = get_all_replies_from_comment(a_a_comment)
  return comments


def get_aa_comment_from_submission(submission):
  tuple_thing = comment_forest_to_lists(submission.comments.list())
  comments, more_comments = tuple_thing

  for comment in comments:
    if comment.author == "AutoModerator":
      return comment
  return None
  # If you have trouble finding the alternative angle comment in the future,
  # you should serach in the list of MoreComments objects!
  # The a_a_comment might not even exist yet, since it is created by a bot after
  # the post is created. You might want to call this function again in a few secs.


def get_all_replies_from_comment(comment_or_more_comments, temp_list=None):
  temp_list = [] if temp_list is None else temp_list
  if isinstance(comment_or_more_comments, models.MoreComments):
    for c in comment_or_more_comments.comments():
      get_all_replies_from_comment(c, temp_list)
  elif isinstance(comment_or_more_comments, models.Comment):
    temp_list.append(comment_or_more_comments)
    for child in comment_or_more_comments.replies.list():
      get_all_replies_from_comment(child, temp_list)
  else:
    print(f"What is this: {comment_or_more_comments}")
  return temp_list


def comment_forest_to_lists(comment_forest):
  more_comments = []
  comments = []
  for commentOrMoreComments in comment_forest:
    if isinstance(commentOrMoreComments, models.MoreComments):
      more_comments.append(commentOrMoreComments)
    elif isinstance(commentOrMoreComments, models.Comment):
      comments.append(commentOrMoreComments)
    else:
      print(f"What is this: {commentOrMoreComments}")
  return comments, more_comments

# def is_relevant(comment):
#     # Does this comment belong to one of the posts on the watchlist?
#     if list(filter(lambda x: x["post_id"] == comment.link_id, watch_list)):
#         root_comment = getAlternativeAnglesCommentFromSubmission(comment.submission)
#         # Is this comment a child of the stickied 'Mirrors / Alternate angles' comment?
#         if list(filter(lambda x: x["root_comment_id"] == root_comment.id, watch_list)):
#             return True
#     return False
