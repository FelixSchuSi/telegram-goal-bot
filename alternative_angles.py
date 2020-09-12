from time import sleep
from setup import setup
from praw import models
from pyquery import PyQuery as pq
from existing_comments import getExistingComments
from scrape import mp4_link
from telegram_wrapper import send_video
import itertools

title = "Arsenal [2] - 1 Chelsea - Aubameyang 67' [FA Cup Final]"
apis = setup()
watch_list = []

# TODO: add functionality to listen for comments on posts that are on the watch_list.
# for comment in apis["subreddit"].stream.comments():
#     if is_relevant(comment):
#         # test if comment is child of saved comment
#         print(comment.body)
#     else:
#         print(comment.body)

def findSubmissionByTitle(title):
    search_results = list(apis["subreddit"].search(title))
    # For now, we return the first search result.
    return search_results[0]

def filterLinks(links):
    filtered = []
    for elem in links:
        link, text = elem
        if any(host in link for host in ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']):
            # Maybe also filter by text?
            filtered.append(elem)
    return filtered

def getLinksFromComments(comments):
    links = []
    for comment in comments:
        links.append(getLinksFromComment(comment))

    flat = list(itertools.chain.from_iterable(links))
    filtered = filterLinks(flat)
    return filtered

def getLinksFromComment(comment):
    html = comment.body_html
    d = pq(html)
    links = []
    # TODO: Refactor with list comprehension
    for link in d("a"):
        is_link = link.text == None or 'http' in link.text
        text = '' if is_link else link.text
        links.append((link.attrib["href"], text))
    return links

def parseTitle(title):
    submission = findSubmissionByTitle(title)
    relevant_comments = getExistingComments(submission)
    links = getLinksFromComments(relevant_comments)
    # scraped_links = list(map(lambda x: (mp4_link(x[0]), x[1]), links))
    return links
