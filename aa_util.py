from praw import models
from pyquery import PyQuery as pq
import itertools
from multiprocessing import Process
from datetime import datetime
from aa_watchlist import WatchList
from scrape import scrape_with_retries
from telegram_wrapper import send_message, send_video
import time

# {aa_comment_id: [("telegram_user_id_1", created_at), ("telegram_user_id_2", created_at)]}
watchlist = WatchList()


def comment_listener(apis):
  print('[COMMENT LISTENER] Started comment listener')
  for comment in apis["subreddit"].stream.comments():
    aa_comment = get_aa_comment_from_submission(comment.submission)
    if aa_comment is not None:
      if aa_comment.id in watchlist:
        print(f'[COMMENT FILTER] aa_comment {aa_comment.id} of comment {aa_comment.id} is on watchlist')
        all_children = get_all_replies_from_comment(aa_comment)
        if comment in all_children:
          print(f'[COMMENT FILTER] Comment {comment.id} is child of aa_comment {aa_comment.id}')
          print(f'[COMMENT FILTER] Comment {comment.id} passed the filter!')
          links_with_texts = get_links_from_comments(all_children)
          registered_users = watchlist.get_registered_users(aa_comment.id)
          for telegram_user_id, created_at in registered_users:
            # TODO: Get is_eng in the registration and pass it to this function instead of False
            send_links_with_texts(apis, links_with_texts, telegram_user_id, False)


def queue_handler(queue, passed_apis):
  print('[QUEUE HANDLER] Started queue handler')  #
  apis = passed_apis
  listen_for_comments_process = Process(target=comment_listener, args=(passed_apis,))
  listen_for_comments_process.start()
  while True:
    try:
      new_item = queue.get()
      new_submission, telegram_user_id = new_item
      print(f"[QUEUE HANDLER] Received new submission in queue: {new_submission}")
      aa_comment = get_aa_comment_from_submission(new_submission)
      if aa_comment is not None and telegram_user_id is not None:
        watchlist.append(aa_comment.id, telegram_user_id)
    except Exception as e:
      print('[QUEUE HANDLER] ', e)

    diff = datetime.utcnow() - watchlist.last_expiration_check
    if (diff.total_seconds() / 60 / 60) >= 1:
      # Every hour we remove expired entries from the watch list
      print('[QUEUE HANDLER] Initiated removal of expired entries in watchlist')
      watchlist.remove_expired_entries()
    time.sleep(1)


def find_submission_by_title(title, apis):
  search_results = list(apis["subreddit"].search(title))
  # For now, we return the first search result.

  return search_results[0] if len(search_results) > 0 else None


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
  if submission is None:
    return None, None
  relevant_comments = get_existing_comments(submission)
  links = get_links_from_comments(relevant_comments)
  return links, submission


def get_existing_comments(submission):
  a_a_comment = get_aa_comment_from_submission(submission)
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


def is_comment_childof(child, parent):
  all_children = get_all_replies_from_comment(parent)
  return child in all_children


def send_links_with_texts(apis, links_with_texts, user_id, is_eng):
  for i, linkWithText in enumerate(links_with_texts):
    print(f'[EXISTING COMMENTS] parsing link {i + 1} of {len(links_with_texts)}')
    link, title = linkWithText
    print('[EXISTING COMMENTS] linkWithText', linkWithText)
    mp4_link, new_title = parse_link_with_text(linkWithText, is_eng)
    links = (link, mp4_link)
    try:
      send_video(apis, new_title, links, user_id) if mp4_link else send_message(apis, title, link, user_id)
    except Exception as e:
      print('[EXISTING COMMENTS] Error when sending this: ' + linkWithText)
      print(e)


def parse_link_with_text(link_with_text, is_eng):
  ger_no_desc = "Ohne Beschreibung"
  eng_no_desc = "No description"
  no_desc = eng_no_desc if is_eng else ger_no_desc

  link, title = link_with_text
  scraped_link = scrape_with_retries(link, title)
  string = no_desc if title == '' else title
  return scraped_link, string
