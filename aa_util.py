from praw import models
from pyquery import PyQuery as pq
import itertools
from multiprocessing import Process
from datetime import datetime
import time

# {aa_comment_id: [("telegram_user_id_1", created_at), ("telegram_user_id_2", created_at)]}

watch_list = {}


def comment_listener(apis):
    print('[COMMENT LISTENER] started comment listener')
    for comment in apis["subreddit"].stream.comments():
        aa_comment = get_alternative_angles_comment_from_submission(comment.submission)
        print('[COMMENT LISTENER]', aa_comment.id) if aa_comment is not None else print('no aa_comment found')
        if aa_comment is not None:
            # element[0] here is the key of a dict entry.
            # the key of an entry within the watch list is the ID of the aa_comment
            hit = tuple(filter(lambda element: element[0] == aa_comment.id, watch_list.items()))
            if any(hit):

                registered_users = hit[0]
                for user in registered_users:
                    telegram_id, created_at = user
                    # send video here using telegram_id
                    # scrape comment first and only send if link was found

                # print('[COMMENT LISTENER] aa_comment lookup: ', x, type(x))
                print(f"[COMMENT LISTENER] HIT!")
                # TODO: check if comment is child of aa_comment then parse and send


def queue_handler(queue, apis):
    print('[QUEUE HANDLER] started queue handler')
    listen_for_comments_process = Process(target=comment_listener, args=(apis,))
    listen_for_comments_process.start()
    watch_list_last_updated = datetime.utcnow()
    while True:
        try:
            new_item = queue.get()
            new_submission, user_id = new_item
            print(f"[QUEUE HANDLER] received new submission in queue: {new_submission}")
            aa_comment = get_alternative_angles_comment_from_submission(new_submission)
            if aa_comment is not None and user_id is not None:
                created_at = datetime.utcnow()
                print(f"[QUEUE HANDLER] putting aa_comment of {new_submission} in watch list")
                if watch_list.get(aa_comment.id) is None:
                    watch_list[aa_comment.id] = [(user_id, created_at)]
                else:
                    if isinstance(watch_list[aa_comment.id], list):
                        watch_list[aa_comment.id].append((user_id, created_at))
                    else:
                        print(
                            f'[QUEUE HANDLER] watch list item does not have the expected format: {watch_list[aa_comment.id]}')
        except Exception as e:
            print(e)
        # new code, needs to be tested
        diff = datetime.utcnow() - watch_list_last_updated
        if (diff.total_seconds() / 60 / 60) >= 1:
            for aa_comment_id, registered_users in watch_list:
                for user in registered_users:
                    telegram_user_id, created_at = user
                    if (datetime.utcnow() - created_at).total_seconds() / 60 / 60 > 4:
                        if len(registered_users) == 1:
                            del watch_list[aa_comment_id]
                        else:
                            new_registered_users = registered_users.remove(user)
                            del watch_list[aa_comment_id]
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
    a_a_comment = get_alternative_angles_comment_from_submission(submission)
    replies = a_a_comment.replies.list()
    comments = get_all_replies_from_comment(a_a_comment)
    return comments


def get_alternative_angles_comment_from_submission(submission):
    tuple_thing = comment_forest_to_lists(submission.comments.list())
    comments, more_comments = tuple_thing

    for comment in comments:
        if comment.author == "AutoModerator":
            return comment
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
