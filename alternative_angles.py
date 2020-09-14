from pyquery import PyQuery as pq
from existing_comments import get_existing_comments, get_alternative_angles_comment_from_submission
import itertools
from multiprocessing import Process
import time

# title = "Arsenal [2] - 1 Chelsea - Aubameyang 67' [FA Cup Final]"
watch_list = []


def comment_listener(apis):
    print('[COMMENT LISTENER] started comment listener')
    for comment in apis["subreddit"].stream.comments():
        print(f"[COMMENT LISTENER] Analyzing this: {comment.body}")
        submission = comment.submission
        if get_alternative_angles_comment_from_submission(submission).id in watch_list:
            print(f"[COMMENT LISTENER] HIT!")
        else:
            print(f"[COMMENT LISTENER] NO HIT!")


def queue_handler(queue, apis):
    print('[QUEUE HANDLER] started queue handler')
    listen_for_comments_process = Process(target=comment_listener, args=(apis,))
    while True:
        try:
            new_submission_id = queue.get()
            print(f"[QUEUE HANDLER] received new submission id in queue: {new_submission_id}")
            watch_list.append(new_submission_id)
        except Exception as e:
            print(e)
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
    return links, submission.id
