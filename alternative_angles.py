from time import sleep
from setup import setup
from praw import models
from pyquery import PyQuery as pq
from existing_comments import getRelevantComments
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
        print(f"link: {link} | text: {text}")
        if any(host in link for host in ['streamja', 'streamable', 'imgtc', 'clippituser', 'vimeo', 'streamvi']):
            # Maybe also filter by text?
            filtered.append(elem)
    return filtered

def getLinksFromComments(comments):
    links = []
    for comment in comments:
        links.append(getLinksFromComment(comment))

    flat = list(itertools.chain.from_iterable(links))
    print("unfiltered: ", flat)
    filtered = filterLinks(flat)
    return filtered

def getLinksFromComment(comment):
    html = comment.body_html
    d = pq(html)
    links = []
    # TODO: Refactor with list comprehension
    for link in d("a"):
        links.append((link.attrib["href"], link.text))
    return links

def parseTitle(title):
    submission = findSubmissionByTitle(title)
    relevant_comments = getRelevantComments(submission)
    links = getLinksFromComments(relevant_comments)
    # scraped_links = list(map(lambda x: (mp4_link(x[0]), x[1]), links))
    return links

# print(scraped_links)

# print('!!! SENDING NOW !!!')
# print(apis["chat_id"])
# print(apis["bot"])
# for i in scraped_links:
#     link, text = i
#     print("TRYING TO SEND THIS: ", i)
#     send_video(apis, text, link)
#     print("SENT THIS: ", i)
#     sleep(500)

# i = scraped_links[3]
# link, text = i
# print("TRYING TO SEND THIS: ", i)
# send_video(apis, text, link)
# print("SENT THIS: ", i)
# print('!!! SUCCESS !!!')


